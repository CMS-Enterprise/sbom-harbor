mod provider;
pub use provider::*;

#[cfg(test)]
mod tests {
    use crate::config::{dev_context, snyk_token};
    use crate::entities::tasks::{Task, TaskKind};
    use crate::services::packages::PackageService;
    use crate::services::sboms::snyk::provider::SbomSyncTask;
    use crate::services::sboms::{FileSystemStorageProvider, SbomService};
    use crate::services::snyk::{SnykService, API_VERSION};
    use crate::services::tasks::TaskProvider;
    use crate::{entities, Error};
    use platform::mongodb::Store;
    use std::sync::Arc;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_process_sboms() -> Result<(), Error> {
        let provider = test_provider().await?;

        let mut task = Task::new(TaskKind::Sbom(entities::sboms::SbomProviderKind::Snyk {
            api_version: API_VERSION.to_string(),
        }))?;

        match provider.execute(&mut task).await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(format!("can_process_sboms::{}", e)));
            }
        };

        Ok(())
    }

    async fn test_provider() -> Result<SbomSyncTask, Error> {
        let token = snyk_token()?;
        let cx = dev_context(None)?;
        let store = Arc::new(Store::new(&cx).await?);

        let provider = SbomSyncTask::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            SbomService::new(
                store,
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/sboms".to_string(),
                )),
            ),
        )?;

        Ok(provider)
    }
}
