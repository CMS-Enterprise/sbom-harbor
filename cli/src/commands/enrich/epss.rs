use harbcore::entities::enrichments::VulnerabilityProviderKind;
use std::sync::Arc;

use crate::commands::enrich::EnrichArgs;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::enrichments::vulnerabilities::epss::EpssScoreTask;
use harbcore::services::tasks::TaskProvider;
use platform::mongodb::{Context, Store};

use crate::Error;

/// Strategy pattern implementation that handles EPSS Enrich commands.
pub struct EpssProvider {}

impl EpssProvider {
    /// Factory method to create new instance of type.
    async fn new_provider(cx: Context) -> Result<EpssScoreTask, Error> {
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Enrich(e.to_string()))?,
        );

        let provider = EpssScoreTask::new(store);

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
    use harbcore::entities::packages::Purl;
    use uuid::Uuid;

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

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn migrate_vulnerabilities() -> Result<(), Error> {
        let cx = dev_context(Some("harbor")).map_err(|e| Error::Config(e.to_string()))?;

        let store = Store::new(&cx)
            .await
            .map_err(|e| Error::Enrich(e.to_string()))?;

        let purls: Vec<Purl> = match store.list().await {
            Ok(purls) => purls,
            Err(e) => {
                return Err(Error::Enrich(format!("run::{}", e)));
            }
        };

        for purl in purls.iter() {
            if purl.vulnerabilities.is_none() {
                continue;
            }

            let mut vulns = purl.vulnerabilities.clone().unwrap();
            for vuln in vulns.iter_mut() {
                vuln.id = Uuid::new_v4().to_string();
                vuln.purl = purl.purl.clone();
                store
                    .insert(vuln)
                    .await
                    .map_err(|e| Error::Enrich(e.to_string()))?;
            }
        }

        Ok(())
    }
}