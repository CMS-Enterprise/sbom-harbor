mod adapters;
mod changeset;
mod findings;
mod sboms;
mod service;

pub(in crate::services::snyk) mod client;
pub(in crate::services::snyk) use client::*;

pub type IssueSnyk = models::CommonIssueModel;

use crate::entities::xrefs::Xref;
use crate::Error;
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
    use crate::entities::sboms::{Finding, Sbom};
    use crate::entities::xrefs::{XrefKind, Xrefs};
    use crate::services::findings::{FileSystemStorageProvider, StorageProvider};
    use crate::services::sboms::{
        FileSystemStorageProvider as SbomFileSystemStorageProvider, SbomProvider, SbomService,
    };
    use crate::services::snyk::adapters::Project;
    use crate::services::snyk::changeset::ChangeSet;
    use crate::services::snyk::client::models::ProjectStatus;
    use crate::Error;
    use platform::mongodb::{Context, Service};
    use std::fmt::{Debug, Formatter};
    use tracing::log::debug;
    use tracing::log::kv::Source;

    fn debug_service() -> Result<DebugService, Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let cx = mongo_context(Some("core-test"))?;

        let service = DebugService::new(token, cx);
        Ok(service)
    }

    fn test_service() -> Result<SnykService, Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let cx = mongo_context(Some("core-test"))?;

        let service = SnykService::new(
            token,
            cx.clone(),
            SbomService::new(cx, Box::new(SbomFileSystemStorageProvider::new(None))),
        );

        Ok(service)
    }

    /// Used for manual local debugging.
    #[derive(Debug)]
    struct DebugService {
        cx: Context,
        token: String,
    }

    impl Service<Project> for DebugService {
        fn cx(&self) -> &Context {
            &self.cx
        }
    }

    impl Service<Sbom> for DebugService {
        fn cx(&self) -> &Context {
            &self.cx
        }
    }

    impl DebugService {
        fn new(token: String, cx: Context) -> DebugService {
            DebugService { cx, token }
        }

        // Use this to load projects that have already been stored. Speeds up testing dramatically.
        async fn projects(&self) -> Result<Vec<Project>, Error> {
            self.list().await.map_err(|e| Error::Runtime(e.to_string()))
        }

        // Use this to load projects that have already been stored. Speeds up testing dramatically.
        async fn sboms(&self) -> Result<Vec<Sbom>, Error> {
            self.list().await.map_err(|e| Error::Runtime(e.to_string()))
        }

        // Use this when you need to reload the projects to the local mongo instance.
        async fn load_projects_from_snyk(&self) -> Result<(), Error> {
            let snyk_service = test_service()?;

            let mut projects = match snyk_service.projects().await {
                Ok(p) => p,
                Err(e) => {
                    let msg = format!("debug_service::load_projects_from_snyk::{}", e);
                    debug!("{}", msg);
                    return Err(Error::Snyk(msg));
                }
            };

            if projects.is_empty() {
                return Err(Error::Snyk("build_change_set::no_projects".to_string()));
            }

            for project in projects.iter_mut() {
                match self.insert(project).await {
                    Ok(_) => {}
                    Err(e) => {
                        let msg = format!("project::insert::{}::{}", project.project_name, e);
                        debug!("{}", msg);
                        continue;
                    }
                }
            }

            Ok(())
        }
    }

    #[async_std::test]
    #[ignore = "used to load projects from Snyk to local Mongo for debugging"]
    async fn load_projects_from_snyk() -> Result<(), Error> {
        let debug_service = debug_service()?;

        match debug_service.load_projects_from_snyk().await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(
                    format!("load_projects_from_snyk::{}", e).to_string(),
                ));
            }
        };

        Ok(())
    }

    #[async_std::test]
    #[ignore = "used to debug building changesets from local mongo instance"]
    async fn can_store_change_set_from_local() -> Result<(), Error> {
        let debug_service = debug_service()?;
        let mut projects = debug_service.projects().await?;

        // Take a subset
        //let mut projects = &mut projects[0..5];

        let test_service = test_service()?;
        let mut change_set = ChangeSet::new();

        for project in projects.iter_mut() {
            match test_service.process_project(&mut change_set, project).await {
                Err(e) => {
                    let msg = format!("{}", e);
                    debug!("can_build_change_set_from_local::process_project::{}", msg);
                }
                _ => {}
            }
        }

        assert_ne!(0, change_set.sboms.len());
        assert_ne!(0, change_set.packages.len());
        assert_ne!(0, change_set.purls.len());
        assert_ne!(0, change_set.dependencies.len());
        assert_ne!(0, change_set.unsupported.len());

        match test_service.commit(&mut change_set).await {
            Err(e) => {
                let msg = format!("{}", e);
                debug!("can_build_change_set_from_local::commit::{}", msg);
            }
            _ => {}
        }

        Ok(())
    }

    #[async_std::test]
    async fn can_store_findings_from_local() -> Result<(), Error> {
        let debug_service = debug_service()?;
        let mut sboms = debug_service.sboms().await?;

        let findings_storage_provider = FileSystemStorageProvider::new(None);
        let mut results = HashMap::<String, Vec<Finding>>::new();

        let service = test_service()?;
        let mut change_set = ChangeSet::new();
        let snyk_kind = XrefKind::External(SNYK_DISCRIMINATOR.to_string());

        for sbom in sboms.iter_mut() {
            match service.scan_sbom(&mut change_set, sbom).await {
                Ok(()) => {
                    debug!("can_store_findings_from_local::success");
                    for (purl, package) in change_set.packages.iter() {
                        let snyk_ref = match &package.xrefs {
                            None => {
                                continue;
                            }
                            Some(xrefs) => match xrefs.get(&snyk_kind) {
                                None => {
                                    continue;
                                }
                                Some(xref) => xref,
                            },
                        };

                        match service
                            .findings(
                                snyk_ref.get("org_id").unwrap(),
                                purl.as_str(),
                                package.xrefs.clone(),
                            )
                            .await
                        {
                            Ok(findings) => {
                                match findings {
                                    Some(findings) => {
                                        // results.insert(purl.to_string(), findings.clone());
                                        match findings_storage_provider.write(purl, &findings).await
                                        {
                                            Ok(()) => {
                                                debug!(
                                                    "can_store_findings_from_local::write_success"
                                                );
                                            }
                                            Err(e) => {
                                                let msg = format!("{}", e);
                                                debug!(
                                                    "can_store_findings_from_local::write::{}",
                                                    msg
                                                );
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {
                                let msg = format!("{}", e);
                                debug!("can_store_findings_from_local::{}", msg);
                            }
                        };
                    }
                }
                Err(e) => {
                    let msg = format!("{}", e);
                    debug!("can_store_findings_from_local::{}", msg);
                }
            }
        }

        // for (purl, sbom) in change_set.sboms.iter() {
        //     match service
        //         .findings(org_id.as_str(), purl.as_str(), sbom.xrefs.clone())
        //         .await
        //     {
        //         Ok(findings) => match findings {
        //             Some(findings) => {
        //                 results.insert(purl.to_string(), findings.clone());
        //             }
        //             _ => {}
        //         },
        //         Err(e) => {
        //             let msg = format!("{}", e);
        //             debug!("can_store_findings_from_local::{}", msg);
        //         }
        //     };
        // }
        //
        // for (purl, findings) in results.iter() {
        //     match findings_storage_provider.write(purl, findings).await {
        //         Ok(()) => {
        //             debug!("can_store_findings_from_local::write_success");
        //         }
        //         Err(e) => {
        //             let msg = format!("{}", e);
        //             debug!("can_store_findings_from_local::write::{}", msg);
        //         }
        //     }
        // }

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_build_change_set() -> Result<(), Error> {
        let service = test_service()?;
        let mut change_set = ChangeSet::new();

        // Populate the ChangeSet
        match service.build_change_set(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(
                    format!("can_build_change_set::{}", e).to_string(),
                ));
            }
        };

        assert_ne!(0, change_set.sboms.len());
        assert_ne!(0, change_set.packages.len());
        assert_ne!(0, change_set.purls.len());
        assert_ne!(0, change_set.dependencies.len());
        assert_ne!(0, change_set.unsupported.len());

        Ok(())
    }

    // #[async_std::test]
    // #[ignore = "manual run only"]
    // async fn can_sync_sboms() -> Result<(), Error> {
    //     let service = test_service()?;
    //
    //     service.sync().await?;
    //
    //     Ok(())
    // }

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
