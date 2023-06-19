use clap::Parser;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::tasks::TaskProvider;

use crate::tasks::example::ExampleTask;
use crate::Error;

/// Specifies the CLI args for the Packages command.
#[derive(Debug, Parser)]
pub struct ExampleArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,
}

/// The Example Command handler.
pub async fn execute(args: &ExampleArgs) -> Result<(), Error> {
    let cx = match &args.debug {
        false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    let mut task = Task::new(TaskKind::Extension("example".to_string()))
        .map_err(|e| Error::Task(e.to_string()))?;

    let provider = ExampleTask::new(cx)
        .await
        .map_err(|e| Error::Task(e.to_string()))?;

    provider
        .execute(&mut task)
        .await
        .map_err(|e| Error::Task(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        execute(&ExampleArgs { debug: true }).await
    }
}
