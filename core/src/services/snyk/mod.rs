mod adapters;
mod changeset;
mod findings;
mod sboms;
mod service;

pub(in crate::services::snyk) mod client;
pub(in crate::services::snyk) use client::*;

use crate::entities::xrefs::Xref;
use serde::{Deserialize, Serialize};
pub use service::*;
use std::collections::HashMap;

const SNYK_DISCRIMINATOR: &str = "snyk";

#[allow(missing_docs)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnykRef {
    pub org_id: String,
    pub org_name: String,
    pub group_id: String,
    pub group_name: String,
    pub project_id: String,
    pub project_name: String,
}

impl From<SnykRef> for Xref {
    fn from(snyk_ref: SnykRef) -> Self {
        HashMap::from([
            ("group_id".to_string(), snyk_ref.group_id.clone()),
            ("group_name".to_string(), snyk_ref.group_name.clone()),
            ("org_id".to_string(), snyk_ref.org_id.clone()),
            ("org_name".to_string(), snyk_ref.org_name.clone()),
            ("project_id".to_string(), snyk_ref.project_id.clone()),
            ("project_name".to_string(), snyk_ref.project_name.clone()),
        ])
    }
}

impl SnykRef {
    pub fn eq(&self, xref: &SnykRef) -> bool {
        self.org_id == xref.org_id
            && self.group_id == xref.group_id
            && self.project_id == xref.project_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::mongo_context;
    use crate::services::sboms::SbomProvider;
    use crate::services::snyk::changeset::ChangeSet;
    use crate::Error;

    fn test_service() -> Result<SnykService, Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let cx = mongo_context(Some("core-test"))?;

        let service = SnykService::new(token, cx);
        Ok(service)
    }

    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_build_change_set() -> Result<(), Error> {
        let service = test_service()?;

        let mut change_set = ChangeSet::new();
        service.build_change_set(&mut change_set).await?;

        assert_ne!(0, change_set.sboms.len());
        assert_ne!(0, change_set.packages.len());
        assert_ne!(0, change_set.purls.len());
        assert_ne!(0, change_set.dependencies.len());
        assert_ne!(0, change_set.unsupported.len());

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_sync_sboms() -> Result<(), Error> {
        let service = test_service()?;

        service.sync().await?;

        Ok(())
    }

    // #[async_std::test]
    // async fn can_sync_purls() -> Result<(), Error> {
    //     let service = test_service()?;
    //
    //     service
    //         .register_purls()
    //         .await
    //         .map_err(|e| Error::Snyk(e.to_string()))?;
    //
    //     //service.registry_issues(purls).await?;
    //
    //     Ok(())
    // }
    //
    // #[async_std::test]
    // async fn can_sync_issues() -> Result<(), Error> {
    //     let service = test_service()?;
    //
    //     service
    //         .register_issues()
    //         .await
    //         .map_err(|e| Error::Snyk(e.to_string()))?;
    //
    //     Ok(())
    // }
    //
    // #[async_std::test]
    // async fn can_register_sboms() -> Result<(), Error> {
    //     let service = test_service()?;
    //
    //     service
    //         .register_sboms()
    //         .await
    //         .map_err(|e| Error::Snyk(e.to_string()))?;
    //
    //     Ok(())
    // }
}
