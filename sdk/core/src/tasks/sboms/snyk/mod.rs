mod sync;
pub use sync::*;

#[cfg(test)]
mod tests {
    use crate::config::{dev_context, snyk_token};
    use crate::entities::tasks::{Task, TaskKind};
    use crate::services::packages::PackageService;
    use crate::services::sboms::{FileSystemStorageProvider, SbomService};
    use crate::services::snyk::SnykService;
    use crate::tasks::sboms::snyk::sync::SyncTask;
    use crate::tasks::TaskProvider;
    use crate::{entities, Error};
    use platform::persistence::mongodb::Store;
    use std::sync::Arc;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_process_sboms() -> Result<(), Error> {
        let provider = test_provider().await?;

        let mut task = Task::new(TaskKind::Sbom(entities::sboms::SbomProviderKind::Snyk))?;

        match provider.execute(&mut task).await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(format!("can_process_sboms::{}", e)));
            }
        };

        Ok(())
    }

    async fn test_provider() -> Result<SyncTask, Error> {
        let token = snyk_token()?;
        let cx = dev_context(None)?;
        let store = Arc::new(Store::new(&cx).await?);

        let provider = SyncTask::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            SbomService::new(
                store.clone(),
                Some(Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/sboms".to_string(),
                ))),
                Some(PackageService::new(store)),
            ),
        )?;

        Ok(provider)
    }
}
