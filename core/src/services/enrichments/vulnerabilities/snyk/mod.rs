mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use crate::config::{dev_context, snyk_token};
    use crate::entities::scans::{Scan, ScanKind};
    use crate::services::enrichments::vulnerabilities::snyk::VulnerabilityProvider;
    use crate::services::enrichments::vulnerabilities::{
        FileSystemStorageProvider, VulnerabilityService,
    };
    use crate::services::packages::PackageService;
    use crate::services::scans::ScanProvider;
    use crate::services::snyk::SnykService;
    use crate::{entities, Error};
    use platform::mongodb::Store;
    use std::sync::Arc;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_scan_purls() -> Result<(), Error> {
        let provider = test_provider().await?;

        let mut scan = Scan::new(ScanKind::Vulnerabilities(
            entities::enrichments::vulnerabilities::VulnerabilityProviderKind::Snyk,
        ))?;

        match provider.execute(&mut scan).await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(format!("can_scan_purls::{}", e)));
            }
        };

        Ok(())
    }

    async fn test_provider() -> Result<VulnerabilityProvider, Error> {
        let token = snyk_token()?;
        let cx = dev_context(None)?;
        let store = Arc::new(Store::new(&cx).await?);
        let provider = VulnerabilityProvider::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            VulnerabilityService::new(
                store,
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/vulnerabilities".to_string(),
                )),
            ),
        )?;

        Ok(provider)
    }
}
