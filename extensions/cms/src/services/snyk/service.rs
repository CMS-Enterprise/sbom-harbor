use std::fmt::Debug;

use crate::Error;
use tracing::debug;

use crate::services::snyk::client::*;

/// Provides Snyk related data retrieval and type adaptation.
#[derive(Debug)]
pub struct SnykService {
    /// The Snyk API Client instance.
    client: Client,
}

impl SnykService {
    /// Factory method to create new instance of type.
    pub fn new(token: String) -> Self {
        let client = Client::new(token);
        Self { client }
    }

    /// Get OrgTags from the Snyk API.
    pub async fn org_tags(&self) -> Result<Vec<OrgTag>, Error> {
        let tags = match self.client.org_tags().await {
            Ok(orgs) => orgs,
            Err(e) => {
                return Err(Error::Task(e.to_string()));
            }
        };

        match tags {
            None => Err(Error::Snyk("orgs_tag_not_found".to_string())),
            Some(tags) => {
                if tags.is_empty() {
                    return Err(Error::Task("org_tags_empty".to_string()));
                }

                let mut results = vec![];

                tags.into_iter().for_each(|inner| {
                    results.push(Organization::new(inner));
                });

                Ok(results)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::snyk::SnykService;
    use crate::Error;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_list_org_tags() -> Result<(), Error> {
        let token = std::env::var("SNYK_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let service = SnykService::new(token);
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
