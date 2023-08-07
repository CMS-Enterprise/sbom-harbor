use platform::persistence::mongodb::{Context, Service, Store};
use platform::testing::persistence::mongodb::DebugEntity;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::Error;

use crate::services::ionchannel::client::*;

/// Provides Ion Channel related data retrieval and type adaptation.
#[derive(Debug)]
pub struct IonChannelService {
    /// The Ion Channel API Client instance.
    client: Client,
    store: Arc<Store>,
}

impl Service<Metric> for IonChannelService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<DebugEntity> for IonChannelService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl IonChannelService {
    /// Factory method to create new instance of type.
    pub fn new(store: Arc<Store>, token: String, org_id: String) -> Self {
        let client = Client::new(token, org_id);
        Self { client, store }
    }

    /// Get Metrics from the Ion Channel API.
    pub async fn metrics(
        &self,
        repo_id: Option<&str>,
        package_id: Option<&str>,
        product_id: Option<&str>,
    ) -> Result<Vec<Metric>, Error> {
        let response = match self.client.metrics(repo_id, package_id, product_id).await {
            Ok(metrics) => metrics,
            Err(e) => {
                return Err(Error::IonChannel(e.to_string()));
            }
        };

        response.to_metrics()
    }

    pub async fn debug_metrics(
        &self,
        repo_id: Option<&str>,
        package_id: Option<&str>,
        product_id: Option<&str>,
    ) -> Result<DebugEntity, Error> {
        let response = match self
            .client
            .debug_metrics(repo_id, package_id, product_id)
            .await
        {
            Ok(metrics) => metrics,
            Err(e) => {
                return Err(Error::IonChannel(e.to_string()));
            }
        };

        Ok(response)
    }

    /// Save a set of metrics to the data store.
    pub async fn save_metrics(
        &self,
        purl: &str,
        mut metrics: Vec<Metric>,
    ) -> Result<HashMap<String, String>, Error> {
        let mut errs = HashMap::new();

        for metric in metrics.iter_mut() {
            metric.purl = Some(purl.clone().to_string());

            match self.insert(metric).await {
                Ok(_) => {}
                Err(e) => {
                    let key = format!(
                        "{}:{}",
                        purl,
                        metric.name.clone().unwrap_or("unknown".to_string())
                    );
                    errs.insert(key, e.to_string());
                }
            }
        }

        Ok(errs)
    }
}
#[cfg(test)]
mod tests {
    use crate::services::ionchannel::IonChannelService;
    use crate::Error;
    use harbcore::config::{dev_context, ion_channel_org_id, ion_channel_token};
    use platform::persistence::mongodb::Store;
    use std::sync::Arc;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_metrics() -> Result<(), Error> {
        let cx = dev_context(None).map_err(|e| Error::Config(e.to_string()))?;
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Config(e.to_string()))?,
        );
        let token = ion_channel_token().map_err(|e| Error::Config(e.to_string()))?;
        let org_id = ion_channel_org_id().map_err(|e| Error::Config(e.to_string()))?;

        let service = IonChannelService::new(store, token, org_id);
        let metrics = service
            .metrics(
                None,
                None,
                Some("cpe:/a:lodash:lodash:4.17.19::~~~node.js~~"),
            )
            .await?;

        let mut passed = false;
        for metric in metrics.iter() {
            if metric.name.is_some() && !metric.name.clone().unwrap().is_empty() {
                passed = true;
                break;
            }
        }

        assert!(passed);

        Ok(())
    }
}
