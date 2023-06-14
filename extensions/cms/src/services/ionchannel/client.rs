use platform::hyper;
use platform::hyper::ContentType;
use serde::{Deserialize, Serialize};
use harbcore::entities::cyclonedx::Vulnerability;

use crate::Error;

#[allow(dead_code)]
fn vulnerabilities_url() -> String {
    // TODO: Get custom URL
    "https://api.ionchannel.io/v1/vulnerability/getVulnerabilities".to_string()
}

fn metrics_url() -> String {
    "https://api.ionchannel.io/v1/score/getMetricsForEntity".to_string()
}
/// A purpose built Ion Channel HTTP Client.
#[derive(Debug)]
pub struct Client {
    token: String,
    inner: hyper::Client,
}

impl Client {
    /// Factory method for creating new instances of a Client.
    pub fn new(token: String) -> Self {
        let inner = hyper::Client::new();
        Self { token, inner }
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

    #[allow(dead_code)]
    pub async fn vulnerabilities(&self) -> Result<Vec<Vulnerability>, Error> {
        let response: Option<OrgTagObjectListGetResponse> = self
            .inner
            .get(
                &vulnerabilities_url(),
                ContentType::Json,
                &self.token(),
                None::<Vulnerability>,
            )
            .await
            .map_err(|e| Error::IonChannel(e.to_string()))?;

        match response {
            None => Err(Error::IonChannel("ion-channel failed to list vulnerabilities".to_string())),
            Some(r) => Ok(r.data),
        }
    }

    #[allow(dead_code)]
    pub async fn metrics(&self, package_id: String) -> Result<Vec<Metrics>, Error> {
        let request = MetricsRequest{ package_id };

        let response: Option<Vec<Metrics>> = self
            .inner
            .get(
                &metrics_url(),
                ContentType::Json,
                &self.token(),
                None::<MetricsRequest>,
            )
            .await
            .map_err(|e| Error::IonChannel(e.to_string()))?;

        match response {
            None => Err(Error::IonChannel("ion-channel failed to list metrics".to_string())),
            Some(r) => Ok(r.data),
        }
    }
}

struct MetricsRequest {
    package_id: String,
}

#[cfg(test)]
mod tests {
    use crate::services::ionchannel::client::Client;
    use crate::Error;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_vulnerabilities() -> Result<(), Error> {
        let token = std::env::var("ION_CHANNEL_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let client = Client::new(token);
        let vulnerabilities = client.vulnerabilities().await?;

        assert!(!vulnerabilities.is_empty());

        for vulnerability in vulnerabilities {
            // DO SOMETHING
        }

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_metrics() -> Result<(), Error> {
        let token = std::env::var("ION_CHANNEL_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let client = Client::new(token);
        let metrics = client.metrics().await?;

        assert!(!metrics.is_empty());

        for metric in metrics {
            DO SOMETHING
        }

        Ok(())
    }
}
