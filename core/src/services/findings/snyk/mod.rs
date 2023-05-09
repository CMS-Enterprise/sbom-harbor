mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use crate::config::{dev_context, snyk_token};
    use crate::services::findings::snyk::provider::FindingScanProvider;
    use crate::services::findings::{FileSystemStorageProvider, FindingService};
    use crate::services::packages::PackageService;
    use crate::services::snyk::SnykService;
    use crate::Error;
    use platform::mongodb::Store;
    use std::sync::Arc;

    async fn test_service<'a>() -> Result<FindingScanProvider, Error> {
        let token = snyk_token()?;
        let cx = dev_context(Some("core-test"))?;
        let store = Arc::new(Store::new(&cx).await?);
        let provider = FindingScanProvider::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            FindingService::new(
                store.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/findings".to_string(),
                )),
            ),
        )?;

        Ok(provider)
    }
}
