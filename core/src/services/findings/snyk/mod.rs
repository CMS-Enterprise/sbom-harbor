mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::config::{dev_context, snyk_token};
    use crate::entities::packages::FindingProviderKind;
    use crate::services::findings::snyk::provider::FindingScanProvider;
    use crate::services::findings::{FileSystemStorageProvider, FindingProvider, FindingService};
    use crate::services::packages::PackageService;
    use crate::services::snyk::SnykService;
    use crate::Error;
    use tracing::log::debug;
    use platform::mongodb::Store;

    #[async_std::test]
    #[ignore = "debug"]
    async fn can_scan_from_local() -> Result<(), Error> {
        let service = test_service().await?;
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

    async fn test_service() -> Result<FindingScanProvider, Error> {
        let token = snyk_token()?;
        let cx = dev_context(Some("core-test"))?;
        let store = Arc::new(Store::new(&cx).await?);
        let service = FindingScanProvider::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            FindingService::new(
                store.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/findings".to_string(),
                )),
            ),
        );

        Ok(service)
    }
}
