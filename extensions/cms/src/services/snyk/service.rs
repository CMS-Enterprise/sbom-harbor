use harbcore::entities::xrefs::{Xref, XrefKind};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::Error;

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
                return Err(Error::Snyk(e.to_string()));
            }
        };

        if tags.is_empty() {
            return Err(Error::Snyk("org_tags_empty".to_string()));
        }

        Ok(tags)
    }
}

pub fn extract_xref(tag: &OrgTag) -> Result<Xref, Error> {
    let tags = match &tag.attributes.tags {
        None => return Err(Error::Snyk("org_tag_tags_none".to_string())),
        Some(tags) => tags,
    };

    let id = match tags.iter().find(|t| t.key == "FISMA_ACRONYM") {
        None => return Err(Error::Snyk("fisma_id_none".to_string())),
        Some(t) => t.value.clone(),
    };

    let name = match &tag.name {
        None => return Err(Error::Snyk("org_tag_name_none".to_string())),
        Some(name) => name.clone(),
    };

    Ok(Xref {
        kind: XrefKind::External("fisma".to_string()),
        map: HashMap::from([("id".to_string(), id), ("name".to_string(), name)]),
    })
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
