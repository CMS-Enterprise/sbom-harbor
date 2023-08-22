use clap::Parser;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::analytics::{
    FileSystemStorageProvider, S3StorageProvider, StorageProvider,
};
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::Store;
use std::sync::Arc;

use crate::tasks::export::{ExportService, ExportTask};
use crate::Error;

/// Specifies the CLI args for the Export command.
#[derive(Debug, Parser)]
pub struct ExportArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,
}

/// The Export Command handler.
pub async fn execute(args: &ExportArgs) -> Result<(), Error> {
    let storage: Arc<dyn StorageProvider>;

    let cx = match &args.debug {
        false => {
            storage = Arc::new(S3StorageProvider {});
            harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
        }
        true => {
            storage = Arc::new(FileSystemStorageProvider::new(
                "/tmp/harbor-debug/extensions/export".to_string(),
            ));
            harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
        }
    };

    let store = Arc::new(
        Store::new(&cx)
            .await
            .map_err(|e| Error::Export(e.to_string()))?,
    );

    let service = ExportService::new(store, storage);

    let mut task = Task::new(TaskKind::Extension("export".to_string()))
        .map_err(|e| Error::Export(e.to_string()))?;

    let provider = ExportTask::new(service);

    provider
        .execute(&mut task)
        .await
        .map_err(|e| Error::Export(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        execute(&ExportArgs { debug: true }).await
    }
}
