use harbcore::entities::cyclonedx::Vulnerability;
use harbcore::entities::xrefs::{Xref, XrefKind};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::Error;

use crate::services::ionchannel::client::*;

/// Provides Ion Channel related data retrieval and type adaptation.
#[derive(Debug)]
pub struct IonChannelService {
    /// The Ion Channel API Client instance.
    client: Client,
}

impl IonChannelService {
    /// Factory method to create new instance of type.
    pub fn new(token: String) -> Self {
        let client = Client::new(token);
        Self { client }
    }

    /// Get Vulnerabilities from the Ion Channel API.
    pub async fn vulnerabilities(&self) -> Result<Vec<Vulnerability>, Error> {
        let metrics = match self.client.metrics().await {
            Ok(metrics) => metrics,
            Err(e) => {
                return Err(Error::IonChannel(e.to_string()));
            }
        };

        if metrics.is_empty() {
            return Err(Error::IonChannel("metrics_empty".to_string()));
        }

        Ok(metrics)
    }

    /// Get Metrics from the Ion Channel API.
    pub async fn metrics(&self) -> Result<Vec<Metrics>, Error> {
        let metrics = match self.client.metrics().await {
            Ok(metrics) => metrics,
            Err(e) => {
                return Err(Error::IonChannel(e.to_string()));
            }
        };

        if metrics.is_empty() {
            return Err(Error::IonChannel("metrics_empty".to_string()));
        }

        Ok(metrics)
    }
}

#[cfg(test)]
mod tests {
    use crate::services::ionchannel::IonChannelService;
    use crate::Error;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_vulnerabilities() -> Result<(), Error> {
        let token = std::env::var("ION_CHANNEL_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let service = IonChannelService::new(token);
        let tags = service.org_tags().await?;

        assert!(!tags.is_empty());

        for tag in tags {
            let id = tag.id.unwrap().to_string();
            let link = tag.links.unwrap().param_self.unwrap().to_string();
            assert!(link.contains(&id));
        }

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_metrics() -> Result<(), Error> {
        let token = std::env::var("ION_CHANNEL_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let service = IonChannelService::new(token);
        let tags = service.org_tags().await?;

        assert!(!tags.is_empty());

        for tag in tags {
            let id = tag.id.unwrap().to_string();
            let link = tag.links.unwrap().param_self.unwrap().to_string();
            assert!(link.contains(&id));
        }

        Ok(())
    }
}
