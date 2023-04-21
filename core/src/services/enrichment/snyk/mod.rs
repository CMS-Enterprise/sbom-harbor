mod changeset;
pub mod findings;
pub mod sboms;

#[cfg(test)]
mod tests {
    use crate::config::mongo_context;
    use crate::entities::enrichment::{Scan, ScanRef};
    use crate::entities::packages::{Finding, FindingProviderKind, Purl};
    use crate::entities::sboms::{Sbom, SbomProviderKind};
    use crate::entities::xrefs::{XrefKind, Xrefs};
    use crate::services::enrichment::snyk::changeset::{ScanFindingsChangeSet, ScanSbomsChangeSet};
    use crate::services::enrichment::{FindingProvider, SbomProvider, ScanProvider};
    use crate::services::findings::{FileSystemStorageProvider, FindingService, StorageProvider};
    use crate::services::sboms::{
        FileSystemStorageProvider as SbomFileSystemStorageProvider, SbomService,
    };
    use crate::services::snyk::adapters::Project;
    use crate::services::snyk::{ProjectStatus, API_VERSION};
    use crate::services::snyk::{SnykService, SNYK_DISCRIMINATOR};
    use crate::Error;
    use platform::mongodb::{Context, Service};
    use std::collections::hash_map::Iter;
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use std::iter::Filter;
    use std::ops::Deref;
    use tracing::log::debug;
    use tracing::log::kv::Source;

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
    async fn can_store_sbom_change_set_from_local() -> Result<(), Error> {
        let debug_service = debug_service()?;
        let mut projects = debug_service.projects().await?;

        // Take a subset
        //let mut projects = &mut projects[0..5];

        let service = test_service()?;
        let mut scan = match SbomProvider::init_scan(
            &service,
            SbomProviderKind::Snyk {
                api_version: API_VERSION.to_string(),
            },
        )
        .await
        {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("can_store_sbom_change_set_from_local::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        };

        let mut change_set = ScanSbomsChangeSet::new(&mut scan);

        for project in projects.iter_mut() {
            match service.process_project(&mut change_set, project).await {
                Err(e) => {
                    let msg = format!("{}", e);
                    debug!(
                        "can_store_sbom_change_set_from_local::process_project::{}",
                        msg
                    );
                    change_set.ref_errs(project.id.clone(), e.to_string())
                }
                _ => {}
            }
        }

        assert_ne!(0, change_set.sboms.len());
        assert_ne!(0, change_set.packages.len());
        assert_ne!(0, change_set.purls.len());
        assert_ne!(0, change_set.dependencies.len());
        assert_ne!(0, change_set.unsupported.len());

        match service.commit_sboms(&mut change_set).await {
            Err(e) => {
                let msg = format!("{}", e);
                debug!(
                    "can_store_sbom_change_set_from_local::commit_sboms::{}",
                    msg
                );
            }
            _ => {}
        }

        match service.commit_scan(&mut scan).await {
            Err(e) => {
                let msg = format!("{}", e);
                debug!("can_store_sbom_change_set_from_local::commit_scan::{}", msg);
            }
            _ => {}
        }

        Ok(())
    }

    #[async_std::test]
    async fn can_enrich_from_local() -> Result<(), Error> {
        let service = test_service()?;
        match FindingProvider::enrich(&service, FindingProviderKind::Snyk).await {
            Ok(()) => Ok(()),
            Err(e) => {
                let msg = format!("can_store_findings_change_set_from_local::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        }
    }

    #[async_std::test]
    async fn can_store_findings_change_set_from_local() -> Result<(), Error> {
        let service = test_service()?;
        let mut scan = match FindingProvider::init_scan(&service, FindingProviderKind::Snyk).await {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("can_store_findings_change_set_from_local::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        };

        let mut change_set = ScanFindingsChangeSet::new(&mut scan);
        let snyk_kind = XrefKind::External(SNYK_DISCRIMINATOR.to_string());
        let mut with_findings = 0;

        let purls = service.list().await?;
        let mut purls_with_findings = HashMap::<String, u8>::new();

        for mut purl in purls.into_iter() {
            match service.scan_purl(&mut change_set, &mut purl).await {
                Ok(_) => {}
                Err(e) => {
                    debug!("{}", e);
                }
            }
        }

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_build_sbom_change_set() -> Result<(), Error> {
        let service = test_service()?;

        let mut scan = match SbomProvider::init_scan(
            &service,
            SbomProviderKind::Snyk {
                api_version: API_VERSION.to_string(),
            },
        )
        .await
        {
            Ok(scan) => scan,
            Err(e) => {
                return Err(Error::Snyk(
                    format!("can_build_change_set::init_scan::{}", e).to_string(),
                ));
            }
        };

        let mut change_set = ScanSbomsChangeSet::new(&mut scan);

        // Populate the ChangeSet
        match service.build_sboms(&mut change_set).await {
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

        let finding_service =
            FindingService::new(cx.clone(), Box::new(FileSystemStorageProvider::new(None)));

        let sbom_service = SbomService::new(
            cx.clone(),
            Box::new(SbomFileSystemStorageProvider::new(None)),
        );

        let service = SnykService::new(token, cx.clone(), sbom_service, finding_service);

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

    impl Service<Purl> for DebugService {
        fn cx(&self) -> &Context {
            &self.cx
        }
    }

    impl Service<Scan> for DebugService {
        fn cx(&self) -> &Context {
            &self.cx
        }
    }

    impl Service<ScanRef> for DebugService {
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

        // Use this to load purls that have already been stored. Speeds up testing dramatically.
        async fn purls(&self) -> Result<Vec<Purl>, Error> {
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
}
