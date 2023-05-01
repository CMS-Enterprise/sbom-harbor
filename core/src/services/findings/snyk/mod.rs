pub mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use crate::config::{mongo_context, snyk_token};
    use crate::services::findings::snyk::provider::FindingScanProvider;
    use crate::services::findings::{FileSystemStorageProvider, FindingService};
    use crate::services::packages::PackageService;
    use crate::services::scans::ScanProvider;
    use crate::services::snyk::SnykService;
    use crate::Error;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn debug_scan_findings() -> Result<(), Error> {
        let mut provider = debug_provider()?;
        provider.scan().await
    }

    fn debug_provider() -> Result<Box<dyn ScanProvider>, Error> {
        let token = snyk_token()?;
        let cx = mongo_context(Some("core-test"))?;
        let provider = FindingScanProvider::new(
            cx.clone(),
            SnykService::new(token, cx.clone()),
            PackageService::new(cx.clone()),
            FindingService::new(
                cx.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/findings".to_string(),
                )),
            ),
        )?;

        Ok(Box::new(provider))
    }
}
