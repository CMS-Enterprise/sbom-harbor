use harbcore::entities::enrichments::VulnerabilityProviderKind;
use std::sync::Arc;

use crate::commands::enrich::EnrichArgs;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::analytics::sboms::service::AnalyticService;
use harbcore::services::purl2cpe::service::Purl2CpeService;
use harbcore::tasks::enrichments::identity::cpe::sync::SyncTask;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::Store;

use crate::Error;

/// Concrete implementation of the command handler. Responsible for
/// dispatching command to the correct logic handler based on args passed.
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    println!("Executing purl2cpe enrichment provider");

    let cx = match &args.debug {
        false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    let store = Arc::new(
        Store::new(&cx)
            .await
            .map_err(|e| Error::Enrich(e.to_string()))?,
    );

    let analytic_service = AnalyticService::new(store.clone(), None);
    let purl_2_cpe_service = Purl2CpeService::new(store.clone());
    let provider = SyncTask::new(store, analytic_service, purl_2_cpe_service);

    let task_kind = TaskKind::Vulnerabilities(VulnerabilityProviderKind::Purl2Cpe);
    let mut task: Task = Task::new(task_kind).map_err(|e| Error::Enrich(e.to_string()))?;

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
        let args = EnrichArgs {
            provider: EnrichmentProviderKind::Purl2Cpe,
            debug: true,
            snyk_args: None,
            sbom_scorecard_args: None,
        };

        execute(&args).await
    }
}
