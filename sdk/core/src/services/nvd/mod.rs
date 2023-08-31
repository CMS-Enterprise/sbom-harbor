use std::{thread, time};
use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;

use platform::hyper;
use platform::persistence::mongodb::{Service as MongoService, Store};

use crate::entities::enrichments::Vulnerability;
use crate::services;
use crate::services::nvd::client::Client as NvdClient;
use crate::services::nvd::client::models::DefCveItem;

/// HTTP Client for the NVD Service
mod client;

pub struct ServiceItr {
    total_vulns: i32,
    batch_size: i32,
    position: i32,
    client: NvdClient,
}

impl ServiceItr {
    fn new(total_vulns: i32, batch_size: i32, client: NvdClient) -> Self {
        Self { total_vulns, batch_size, position: 0, client }
    }
}

impl ServiceItr {

    fn get_total(&self) -> i32 {
        self.total_vulns
    }

    fn get_remaining(&self) -> i32 {
        self.total_vulns - self.position
    }

    fn has_remaining(&self) -> bool {
        self.get_remaining() > 0
    }

    /// Get the next batch of vulnerabilities form NVD. Request rates are controlled, the
    /// policy from NIST is below:
    ///
    /// The public rate limit (without an API key) is 5 requests in a rolling 30 second window;
    /// the rate limit with an API key is 50 requests in a rolling 30 second window.
    async fn next_batch(&mut self) -> Result<Option<Vec<DefCveItem>>, Error> {
        if self.position == self.total_vulns {
            Ok(None)
        } else {

            let mut per_page = 0;
            let remaining = self.total_vulns - self.position;

            if remaining >= self.batch_size {
                per_page = self.batch_size;
            } else {
                per_page = remaining;
            }

            println!("Position: {} and Per Page: {}", self.position, per_page);
            let response = self.client.get_page_of_vulnerabilities(self.position, per_page).await?;
            self.position = self.position + per_page;

            Ok(response.vulnerabilities)
        }
    }
}

#[derive(Debug)]
pub struct Service {
    client: NvdClient,
    store: Arc<Store>,
}

#[async_trait]
impl MongoService<DefCveItem> for Service {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service {

    pub fn new(api_key: String, store: Arc<Store>) -> Self {
        let client = NvdClient::new(hyper::Client::new(), api_key);
        Self {
            client,
            store,
        }
    }

    pub async fn get_vulnerabilities(&self, batch_size: i32) -> Result<ServiceItr, Error> {
        let total_vulns = self.client.get_total_vulnerabilities().await?;
        Ok(ServiceItr::new(total_vulns, batch_size, self.client.clone()))
    }
}

#[derive(Error, Debug)]
pub enum Error {
    /// Error derived from our Nvd Client
    #[error(transparent)]
    NvdClient(#[from] services::nvd::client::Error)
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::{thread, time};

    use platform::persistence::mongodb::Store;

    use crate::config::{dev_context, nvd_api_key};
    use crate::services::nvd::{Error, Service};

    async fn test_store() -> Arc<Store> {
        let ctx = dev_context(None).unwrap();
        let store = Store::new(&ctx).await.unwrap();
        Arc::new(store)
    }

    #[tokio::test]
    async fn test_iter() -> Result<(), Error> {

        // !!! 2000 is the max.  If the number in the batch is larger
        // than 2000 (even 2001), then the call will return a 400 Bad Request
        // Http error code.
        let batch_size = 100;

        let key = nvd_api_key().unwrap();
        let store = test_store().await;

        let service = Service::new(key, store);
        let mut iter = service.get_vulnerabilities(batch_size).await?;

        // println!("Total vulns: {}, Batch size: {}", iter.get_total(), batch_size);

        while iter.has_remaining() {
            let batch = iter.next_batch().await?.unwrap();
            let remaining = iter.get_remaining();
            // println!("Batch has {} elements and {} remaining", batch.len(), remaining);

            let milli_s = time::Duration::from_secs(1);
            thread::sleep(milli_s);
        }

        Ok(())
    }
}