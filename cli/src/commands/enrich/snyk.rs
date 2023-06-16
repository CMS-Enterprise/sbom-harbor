use std::sync::Arc;

use crate::commands::enrich::EnrichArgs;
use clap::Parser;
use harbcore::entities::enrichments::VulnerabilityProviderKind;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::packages::PackageService;
use harbcore::services::snyk::SnykService;
use harbcore::services::vulnerabilities::{
    FileSystemStorageProvider, S3StorageProvider, StorageProvider, VulnerabilityService,
};
use harbcore::tasks::enrichments::vulnerabilities::snyk::SyncTask;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::{Context, Store};

use crate::Error;

/// Args for use with the Snyk enrichment provider.
#[derive(Clone, Debug, Parser)]
pub struct SnykArgs {
    /// The Snyk Org ID for the enrichment target.
    #[arg(short, long)]
    pub org_id: Option<String>,
    /// The Snyk Project ID for the enrichment target.
    #[arg(long)]
    pub project_id: Option<String>,
}

/// Strategy pattern implementation that handles Snyk Enrich commands.
pub struct SnykProvider {}

impl SnykProvider {
    /// Factory method to create new instance of type.
    async fn new_provider(
        cx: Context,
        storage: Box<dyn StorageProvider>,
    ) -> Result<SyncTask, Error> {
        let token = harbcore::config::snyk_token().map_err(|e| Error::Config(e.to_string()))?;
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Sbom(e.to_string()))?,
        );

        let provider = SyncTask::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            VulnerabilityService::new(store, storage),
        )
        .map_err(|e| Error::Enrich(e.to_string()))?;

        Ok(provider)
    }

    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
        let storage: Box<dyn StorageProvider>;

        let cx = match &args.debug {
            false => {
                storage = Box::new(S3StorageProvider {});
                harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
            }
            true => {
                storage = Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor-debug/vulnerabilities".to_string(),
                ));
                harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
            }
        };

        match &args.snyk_args {
            None => {
                let mut task: Task =
                    Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Snyk))
                        .map_err(|e| Error::Enrich(e.to_string()))?;

                let provider = SnykProvider::new_provider(cx, storage)
                    .await
                    .map_err(|e| Error::Enrich(e.to_string()))?;

                provider
                    .execute(&mut task)
                    .await
                    .map_err(|e| Error::Enrich(e.to_string()))
            }
            Some(_a) => Err(Error::Enrich(
                "individual project not yet implemented".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use harbcore::config::dev_context;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        let cx = dev_context(Some("core-test")).map_err(|e| Error::Config(e.to_string()))?;
        let storage: Box<dyn StorageProvider> = Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/vulnerability".to_string(),
        ));

        let mut task: Task = Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Snyk))
            .map_err(|e| Error::Enrich(e.to_string()))?;

        let provider = SnykProvider::new_provider(cx, storage).await?;

        match provider.execute(&mut task).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Enrich(msg))
            }
        }
    }
}
