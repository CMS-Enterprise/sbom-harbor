pub mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use crate::config::{mongo_context, snyk_token};
    use crate::entities::packages::Purl;
    use crate::entities::sboms::{Sbom, SbomProviderKind};
    use crate::entities::scans::Scan;
    use crate::services::packages::PackageService;
    use crate::services::sboms::snyk::provider::SbomScanProvider;
    use crate::services::sboms::SbomProvider;
    use crate::services::sboms::{FileSystemStorageProvider, SbomService};
    use crate::services::snyk::adapters::Project;
    use crate::services::snyk::SnykService;
    use crate::services::snyk::API_VERSION;
    use crate::Error;
    use platform::mongodb::{Context, Service};
    use std::fmt::Debug;
    use tracing::log::debug;

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
    async fn can_scan_sboms_from_local() -> Result<(), Error> {
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
            None,
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

        for project in projects.iter_mut() {
            match service.scan_target(&mut scan, project).await {
                Err(e) => {
                    let msg = format!("{}", e);
                    debug!(
                        "can_store_sbom_change_set_from_local::process_project::{}",
                        msg
                    );
                    scan.ref_errs(project.id.clone(), e.to_string())
                }
                _ => {}
            }
        }

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_scan_sboms() -> Result<(), Error> {
        let service = test_service()?;

        let mut scan = match SbomProvider::init_scan(
            &service,
            SbomProviderKind::Snyk {
                api_version: API_VERSION.to_string(),
            },
            None,
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

        // Populate the ChangeSet
        match service.scan_targets(&mut scan).await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(
                    format!("can_build_change_set::{}", e).to_string(),
                ));
            }
        };

        Ok(())
    }

    fn debug_service() -> Result<DebugService, Error> {
        let token = snyk_token()?;
        let cx = mongo_context(Some("core-test"))?;

        let service = DebugService::new(token, cx);
        Ok(service)
    }

    fn test_service() -> Result<SbomScanProvider, Error> {
        let token = snyk_token()?;
        let cx = mongo_context(Some("core-test"))?;

        let service = SbomScanProvider::new(
            cx.clone(),
            SnykService::new(token, cx.clone()),
            PackageService::new(cx.clone()),
            SbomService::new(
                cx.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/sboms".to_string(),
                )),
            ),
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

            let mut projects = match snyk_service.snyk.projects().await {
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
