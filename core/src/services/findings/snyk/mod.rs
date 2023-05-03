mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use crate::config::{dev_context, snyk_token};
    use crate::entities::packages::FindingProviderKind;
    use crate::services::findings::snyk::provider::FindingScanProvider;
    use crate::services::findings::{FileSystemStorageProvider, FindingProvider, FindingService};
    use crate::services::packages::PackageService;
    use crate::services::snyk::SnykService;
    use crate::Error;
    use tracing::log::debug;

    #[async_std::test]
    #[ignore = "debug"]
    async fn can_scan_from_local() -> Result<(), Error> {
        let service = test_service()?;
        let mut scan = match service.init_scan(FindingProviderKind::Snyk, None).await {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("can_scan_from_local::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        };

        service.scan(&mut scan).await
    }

    fn test_service() -> Result<FindingScanProvider, Error> {
        let token = snyk_token()?;
        let cx = dev_context(Some("core-test"))?;
        let service = FindingScanProvider::new(
            cx.clone(),
            SnykService::new(token),
            PackageService::new(cx.clone()),
            FindingService::new(
                cx.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/findings".to_string(),
                )),
            ),
        );

        Ok(service)
    }
}
