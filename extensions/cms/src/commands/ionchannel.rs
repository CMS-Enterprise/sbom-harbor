use crate::tasks::ionchannel::IonChannelTask;
use crate::Error;

use clap::Parser;
use harbcore::config::{ion_channel_org_id, ion_channel_token};
use harbcore::entities::tasks::Task;
use harbcore::tasks::TaskProvider;

/// Specifies the CLI args for the ionchannel command.
#[derive(Debug, Parser)]
pub struct IonChannelArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(short, long)]
    debug: bool,
}

/// The Ion Channel Command handler.
pub async fn execute(args: &IonChannelArgs) -> Result<(), Error> {
    let cx = match &args.debug {
        false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    let token = ion_channel_token().map_err(|e| Error::Config(e.to_string()))?;
    let org_id = ion_channel_org_id().map_err(|e| Error::Config(e.to_string()))?;

    let mut task = Task::new(harbcore::entities::tasks::TaskKind::Extension(
        "ion-channel::metrics".to_string(),
    ))
    .map_err(|e| Error::IonChannel(e.to_string()))?;

    let provider = IonChannelTask::new(cx, token, org_id)
        .await
        .map_err(|e| Error::IonChannel(e.to_string()))?;

    provider
        .execute(&mut task)
        .await
        .map_err(|e| Error::IonChannel(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        execute(&IonChannelArgs { debug: true }).await
    }
}
