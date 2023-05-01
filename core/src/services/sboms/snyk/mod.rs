pub mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use crate::config::{mongo_context, snyk_token};
    use crate::services::packages::PackageService;
    use crate::services::sboms::snyk::provider::SbomScanProvider;
    use crate::services::sboms::{FileSystemStorageProvider, SbomService};
    use crate::services::scans::ScanProvider;
    use crate::services::snyk::adapters::Project;
    use crate::services::snyk::SnykService;
    use crate::Error;
    use platform::mongodb::{Service, Store};

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn debug_scan_sboms_from_local() -> Result<(), Error> {
        let mut provider = local_provider().await?;

        provider.set_count();

        match provider.scan_targets().await {
            Ok(errors) => {
                if !errors.is_empty() {
                    println!("{:#?}", errors);
                    println!("review me");
                }
            }
            Err(e) => {
                return Err(Error::Snyk(
                    format!("can_build_change_set::{}", e).to_string(),
                ));
            }
        };

        provider.commit_scan().await?;

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn debug_scan_sboms() -> Result<(), Error> {
        let mut provider = debug_provider()?;

        match provider.scan().await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(
                    format!("can_build_change_set::{}", e).to_string(),
                ));
            }
        };

        Ok(())
    }

    fn debug_provider() -> Result<Box<dyn ScanProvider>, Error> {
        let token = snyk_token()?;
        let cx = mongo_context(Some("core-test"))?;

        let provider = SbomScanProvider::new(
            cx.clone(),
            SnykService::new(token, cx.clone()),
            PackageService::new(cx.clone()),
            SbomService::new(
                cx.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/sboms".to_string(),
                )),
            ),
        )?;

        Ok(Box::new(provider))
    }

    async fn local_provider() -> Result<Box<dyn ScanProvider>, Error> {
        let token = snyk_token()?;
        let cx = mongo_context(Some("core-test"))?;

        let mut provider = SbomScanProvider::new(
            cx.clone(),
            SnykService::new(token, cx.clone()),
            PackageService::new(cx.clone()),
            SbomService::new(
                cx.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/sboms".to_string(),
                )),
            ),
        )?;

        provider.targets = local_projects().await?;

        Ok(Box::new(provider))
    }

    // Use this to load projects that have already been stored. Speeds up testing dramatically.
    async fn local_projects() -> Result<Vec<Project>, Error> {
        let cx = mongo_context(Some("core-test"))?;
        let store = Store::new(&cx).await?;
        store
            .list::<Project>()
            .await
            .map_err(|e| Error::Runtime(e.to_string()))
    }
}
