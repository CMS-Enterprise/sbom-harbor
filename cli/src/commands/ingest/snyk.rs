use crate::commands::ingest::IngestArgs;
use crate::Error;
use clap::Parser;
use harbcore::entities;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::{
    FileSystemStorageProvider, S3StorageProvider, SbomService, StorageProvider,
};
use harbcore::services::snyk::SnykService;
use harbcore::tasks::sboms::snyk::SyncTask;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::Store;
use std::sync::Arc;

/// Args for generating one or more SBOMs from the Snyk API.
#[derive(Clone, Debug, Parser)]
pub struct SnykArgs {
    /// The Snyk Org ID for the SBOM target.
    #[arg(short, long)]
    pub org_id: Option<String>,
    /// The Snyk Project ID for the SBOM target.
    #[arg(long)]
    pub project_id: Option<String>,
}

/// Concrete implementation of the command handler. Responsible for
/// dispatching command to the correct logic handler based on args passed.
pub(crate) async fn execute(args: &IngestArgs) -> Result<(), Error> {
    let storage: Box<dyn StorageProvider>;
    let token = harbcore::config::snyk_token().map_err(|e| Error::Config(e.to_string()))?;

    let cx = match &args.debug {
        false => {
            storage = Box::new(S3StorageProvider {});
            harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
        }
        true => {
            storage = Box::new(FileSystemStorageProvider::new(
                "/tmp/harbor-debug/sboms".to_string(),
            ));
            harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
        }
    };

    let store = Arc::new(
        Store::new(&cx)
            .await
            .map_err(|e| Error::Sbom(e.to_string()))?,
    );

    let provider = SyncTask::new(
        store.clone(),
        SnykService::new(token),
        PackageService::new(store.clone()),
        SbomService::new(store, storage),
    )
    .map_err(|e| Error::Sbom(e.to_string()))?;

    match &args.snyk_args {
        None => {
            let mut task: Task = Task::new(TaskKind::Sbom(entities::sboms::SbomProviderKind::Snyk))
                .map_err(|e| Error::Sbom(e.to_string()))?;

            provider
                .execute(&mut task)
                .await
                .map_err(|e| Error::Sbom(e.to_string()))
        }
        Some(args) => {
            let (_, _) = (&args.org_id, &args.project_id);
            Err(Error::Sbom(
                "individual projects not yet implemented".to_string(),
            ))
        }
    }
}
