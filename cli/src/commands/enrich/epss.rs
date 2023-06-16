use harbcore::entities::enrichments::VulnerabilityProviderKind;
use std::sync::Arc;

use crate::commands::enrich::EnrichArgs;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::tasks::enrichments::vulnerabilities::epss::SyncTask;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::Store;

use crate::Error;

/// Concrete implementation of the command handler. Responsible for
/// dispatching command to the correct logic handler based on args passed.
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    let cx = match &args.debug {
        false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    let store = Arc::new(
        Store::new(&cx)
            .await
            .map_err(|e| Error::Enrich(e.to_string()))?,
    );

    let provider = SyncTask::new(store);

    let mut task: Task = Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Epss))
        .map_err(|e| Error::Enrich(e.to_string()))?;

    provider
        .execute(&mut task)
        .await
        .map_err(|e| Error::Enrich(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::execute;
    use crate::commands::enrich::{EnrichArgs, EnrichmentProviderKind};
    use crate::Error;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        execute(&EnrichArgs {
            provider: EnrichmentProviderKind::Epss,
            debug: true,
            snyk_args: None,
        })
        .await
    }
}
