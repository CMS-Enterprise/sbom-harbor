use clap::Parser;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::tasks::TaskProvider;

use crate::tasks::fisma::FismaTask;
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
    let cx = match &args.debug {
        false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    let token = std::env::var("SNYK_TOKEN")
        .map_err(|e| Error::Config(e.to_string()))
        .unwrap();

    let mut task = Task::new(TaskKind::Extension("fisma".to_string()))
        .map_err(|e| Error::Fisma(e.to_string()))?;

    let provider = FismaTask::new(cx, token)
        .await
        .map_err(|e| Error::Fisma(e.to_string()))?;

    provider
        .execute(&mut task)
        .await
        .map_err(|e| Error::Fisma(e.to_string()))
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
