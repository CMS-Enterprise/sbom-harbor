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
use platform::persistence::mongodb::Store;

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

/// Concrete implementation of the command handler. Responsible for
/// dispatching command to the correct logic handler based on args passed.
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    let storage: Box<dyn StorageProvider>;
    let token = harbcore::config::snyk_token().map_err(|e| Error::Config(e.to_string()))?;

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

    let store = Arc::new(
        Store::new(&cx)
            .await
            .map_err(|e| Error::Enrich(e.to_string()))?,
    );

    let provider = SyncTask::new(
        store.clone(),
        SnykService::new(token),
        PackageService::new(store.clone()),
        VulnerabilityService::new(store, Some(storage)),
    )
    .map_err(|e| Error::Enrich(e.to_string()))?;

    match &args.snyk_args {
        None => {
            let mut task: Task =
                Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Snyk))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::enrich::snyk::SnykArgs;
    use crate::commands::enrich::{EnrichArgs, EnrichmentProviderKind};
    use crate::Error;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        execute(&EnrichArgs {
            provider: EnrichmentProviderKind::Snyk,
            debug: true,
            snyk_args: Some(SnykArgs {
                org_id: None,
                project_id: None,
            }),
        })
        .await
    }
}
