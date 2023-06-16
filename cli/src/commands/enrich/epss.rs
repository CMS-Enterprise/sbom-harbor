use harbcore::entities::enrichments::VulnerabilityProviderKind;
use std::sync::Arc;

use crate::commands::enrich::EnrichArgs;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::tasks::enrichments::vulnerabilities::epss::SyncTask;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::{Context, Store};

use crate::Error;

/// Strategy pattern implementation that handles EPSS Enrich commands.
pub struct EpssProvider {}

impl EpssProvider {
    /// Factory method to create new instance of type.
    async fn new_provider(cx: Context) -> Result<SyncTask, Error> {
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Enrich(e.to_string()))?,
        );

        let provider = SyncTask::new(store);

        Ok(provider)
    }

    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
        let cx = match &args.debug {
            false => {
                harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
            }
            true => {
                harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
            }
        };

        let mut task: Task = Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Epss))
            .map_err(|e| Error::Enrich(e.to_string()))?;

        let provider = EpssProvider::new_provider(cx)
            .await
            .map_err(|e| Error::Enrich(e.to_string()))?;

        provider
            .execute(&mut task)
            .await
            .map_err(|e| Error::Enrich(e.to_string()))
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
        let cx = dev_context(Some("harbor")).map_err(|e| Error::Config(e.to_string()))?;

        let mut task: Task = Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Epss))
            .map_err(|e| Error::Enrich(e.to_string()))?;

        let provider = EpssProvider::new_provider(cx).await?;

        match provider.execute(&mut task).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Enrich(msg))
            }
        }
    }
}
