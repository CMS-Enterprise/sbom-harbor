use crate::tasks::ionchannel::{MetricsTask, VulnerabilityTask};
use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use harbcore::entities::enrichments::{Vulnerability, VulnerabilityProviderKind};
use harbcore::entities::tasks::Task;
use harbcore::services::tasks::TaskProvider;
use std::str::FromStr;

/// Enumerates the supported enrichment providers.
#[derive(Clone, Debug)]
enum TaskKind {
    /// Invoke the Vulnerability enrichment provider for Ion Channel.
    Vulnerability,

    /// Invoke the Metrics enrichment provider for Ion Channel.
    Metrics,
}

impl ValueEnum for TaskKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Vulnerability, Self::Metrics]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Vulnerability => {
                PossibleValue::new("vulnerability").help("Run the vulnerability enrichment task")
            }
            Self::Metrics => PossibleValue::new("metrics").help("Run the metrics enrichment task"),
        })
    }
}

impl FromStr for TaskKind {
    type Err = ();

    fn from_str(s: &str) -> Result<EnrichmentProviderKind, Self::Err> {
        let value = s.to_lowercase();
        let value = value.as_str();
        match value {
            "vulnerability" | "v" => Ok(TaskKind::Vulnerability),
            "metrics" | "m" => Ok(TaskKind::Metrics),
            _ => Err(()),
        }
    }
}

/// Specifies the CLI args for the Packages command.
#[derive(Debug, Parser)]
pub struct IonChannelArgs {
    /// Specifies which Enrichment Provider to invoke.
    #[arg(short, long)]
    pub kind: TaskKind,

    /// Specifies to run the command against the local debug environment.
    #[arg(short, long)]
    debug: bool,

    /// Specifies the Package URL when retrieving Metrics for a single [Package].
    #[arg(long)]
    purl: String,
}

/// The Ion Channel Command handler.
pub async fn execute(args: &IonChannelArgs) -> Result<(), Error> {
    let cx = match &args.debug {
        false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    let token = std::env::var("ION_CHANNEL_TOKEN")
        .map_err(|e| Error::Config(e.to_string()))
        .unwrap();

    match &args.kind {
        TaskKind::Vulnerability => {
            // CMS Uses a custom Ion Channel integration specific to their environment. We can
            // use the built-in TaskKind but cannot use a universal task implementation.
            let mut task = Task::new(harbcore::entities::tasks::TaskKind::Vulnerabilities(
                VulnerabilityProviderKind::IonChannel,
            ))
            .map_err(|e| Error::IonChannel(e.to_string()))?;

            let provider = VulnerabilityTask::new(cx, token)
                .await
                .map_err(|e| Error::IonChannel(e.to_string()))?;

            provider
                .execute(&mut task)
                .await
                .map_err(|e| Error::IonChannel(e.to_string()))
        }
        TaskKind::Metrics => {
            // CMS Uses a custom Ion Channel integration specific to their environment. We can
            // use the built-in TaskKind but cannot use a universal task implementation.
            let mut task = Task::new(harbcore::entities::tasks::TaskKind::Extension(
                "ion-channel::metrics".to_string(),
            ))
            .map_err(|e| Error::IonChannel(e.to_string()))?;

            let provider = MetricsTask::new(cx, token)
                .await
                .map_err(|e| Error::IonChannel(e.to_string()))?;

            provider
                .execute(&mut task)
                .await
                .map_err(|e| Error::IonChannel(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute_vulnerabilities() -> Result<(), Error> {
        execute(&IonChannelArgs {
            kind: TaskKind::Vulnerability,
            debug: true,
            purl: "".to_string(),
        })
        .await
    }

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute_metrics() -> Result<(), Error> {
        execute(&IonChannelArgs {
            kind: TaskKind::Metrics,
            debug: true,
            purl: "".to_string(),
        })
        .await
    }
}
