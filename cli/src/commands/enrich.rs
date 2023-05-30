use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use harbcore::entities::enrichments::VulnerabilityProviderKind;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::enrichments::vulnerabilities::snyk::VulnerabilityScanTask;
use harbcore::services::enrichments::vulnerabilities::{
    FileSystemStorageProvider, S3StorageProvider, StorageProvider, VulnerabilityService,
};
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::sbom_scorecard::{show_sbom_scorecard, compare_sbom_scorecards};
use harbcore::services::snyk::SnykService;
use harbcore::services::tasks::TaskProvider;
use platform::mongodb::{Context, Store};

use crate::Error;

/// Specifies the CLI args for the Enrich command.
#[derive(Debug, Parser)]
pub struct EnrichArgs {
    /// Specifies with Enrichment Provider to invoke.
    #[arg(short, long)]
    pub provider: EnrichmentProviderKind,

    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,

    /// Flattened args for use with the Snyk enrichment provider.
    #[command(flatten)]
    pub snyk_args: Option<SnykArgs>,

    /// Flattened args for use with the Dependency Track enrichment provider.
    #[command(flatten)]
    pub dt_args: Option<DependencyTrackArgs>,

    /// Flattened args for use with the Sbom Scorecard enrichment
    #[command(flatten)]
    pub sbom_scorecard_args: Option<SbomScorecardArgs>,
}

/// The Enrich Command handler.
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    match args.provider {
        EnrichmentProviderKind::DependencyTrack => {
            todo!()
        }
        EnrichmentProviderKind::Snyk => SnykProvider::execute(args).await,
        EnrichmentProviderKind::SbomScorecard => SbomScorecardProvider::execute(args).await,
    }
}

/// Enumerates the supported enrichment providers.
#[derive(Clone, Debug)]
pub enum EnrichmentProviderKind {
    /// Use the Dependency Track enrichment provider.
    DependencyTrack,
    /// Use the Snyk enrichment provider.
    Snyk,
    /// Use the sbom-scorecard provider
    SbomScorecard,
}

impl ValueEnum for EnrichmentProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::DependencyTrack, Self::Snyk, Self::SbomScorecard]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::DependencyTrack => {
                PossibleValue::new("dependency-track").help("Run Dependency Track enrichment")
            }
            Self::Snyk => PossibleValue::new("snyk").help("Run Snyk enrichment"),
            Self::SbomScorecard => PossibleValue::new("sbom-scorecard").help("Run Sbom Scorecard"),
        })
    }
}

impl FromStr for EnrichmentProviderKind {
    type Err = ();

    fn from_str(s: &str) -> Result<EnrichmentProviderKind, Self::Err> {
        let value = s.to_lowercase();
        let value = value.as_str();
        match value {
            "dependencytrack" | "dependency-track" | "d" | "dt" => {
                Ok(EnrichmentProviderKind::DependencyTrack)
            }
            "snyk" | "s" => Ok(EnrichmentProviderKind::Snyk),
            _ => Err(()),
        }
    }
}

/// Args for use with the Dependency Track enrichment provider.
#[derive(Clone, Debug, Parser)]
pub struct DependencyTrackArgs {}

/// Args for use with the Snyk enrichment provider.
#[derive(Clone, Debug, Parser)]
pub struct SnykArgs {
    /// The Snyk Org ID for the enrichment target.
    #[arg(short, long)]
    pub org_id: Option<String>,
    /// The Snyk Project ID for the enrichment target.
    #[arg(long)]
    pub project_id: Option<String>,
}

/// Args for use with the Sbom scorecard enrichment.
#[derive(Clone, Debug, Parser)]
pub struct SbomScorecardArgs {
        /// The file path to create a scorecard from
        #[arg(long)]
        pub sbom_file_path_1: Option<String>,
        /// Second file path to compare with the first
        #[arg(long)]
        pub sbom_file_path_2: Option<String>,
}

/// Handles Sbom Scorecard enrichment commands
struct SbomScorecardProvider {}

impl SbomScorecardProvider {

    async fn execute(args: &EnrichArgs) -> Result<(), Error> {
        let mut test_sbom = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_sbom.push("src/services/sboms/sbom_scorecard/test_files/dropwizard.json");

        match &args.sbom_scorecard_args {
            Some(scorecard_args) => {
                match (scorecard_args.sbom_file_path_1.clone(), scorecard_args.sbom_file_path_2.clone()) {
                    (None, None) => {
                        let error_string = "No Paths provided, please use --sbom-file-path-1 <path>, and --sbom-file-path-2 <path>";
                        return Err(Error::SbomScorecard(format!("Failed to compare scorecards due to errors: {}", error_string)));
                    },
                    (None, Some(path)) => {
                        let results = show_sbom_scorecard(path);
                        match results {
                            Ok(valid_result) => println!("\n{}", valid_result),
                            Err(error) => println!("Failed with errors: \n{}", error),
                        }
                    },
                    (Some(path), None) => {
                        let results = show_sbom_scorecard(path);
                        match results {
                            Ok(valid_result) => println!("\n{}", valid_result),
                            Err(error) => println!("Failed with errors: \n{}", error),
                        }
                    },
                    (Some(path_1), Some(path_2)) => {
                        let results = compare_sbom_scorecards(path_1, path_2);
                        match results {
                            Ok(valid_result) => println!("\n{}", valid_result),
                            Err(error) => println!("Failed with errors: \n{}", error),
                        }
                    },
                }
            },
            None => {
                let error_string = "A path to an Sbom file must be provided, please use --sbom-file-path-1 <path>";
                return Err(Error::SbomScorecard(format!("Failed to compare scorecard due to errors: {}", error_string)))
            }
        }
        return Ok(());
    }
}

/// Strategy pattern implementation that handles Snyk Enrich commands.
struct SnykProvider {}

impl SnykProvider {
    /// Factory method to create new instance of type.
    async fn new_provider(
        cx: Context,
        storage: Box<dyn StorageProvider>,
    ) -> Result<VulnerabilityScanTask, Error> {
        let token = harbcore::config::snyk_token().map_err(|e| Error::Config(e.to_string()))?;
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Sbom(e.to_string()))?,
        );

        let provider = VulnerabilityScanTask::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            VulnerabilityService::new(store, storage),
        )
        .map_err(|e| Error::Enrich(e.to_string()))?;

        Ok(provider)
    }

    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    async fn execute(args: &EnrichArgs) -> Result<(), Error> {
        let storage: Box<dyn StorageProvider>;

        let cx = match &args.debug {
            false => {
                storage = Box::new(S3StorageProvider {});
                harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
            }
            true => {
                storage = Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor-debug/vulnerabilities".to_string(),
                ));
                harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
            }
        };

        match &args.snyk_args {
            None => {
                let mut task: Task =
                    Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Snyk))
                        .map_err(|e| Error::Enrich(e.to_string()))?;

                let provider = SnykProvider::new_provider(cx, storage)
                    .await
                    .map_err(|e| Error::Enrich(e.to_string()))?;

                provider
                    .execute(&mut task)
                    .await
                    .map_err(|e| Error::Sbom(e.to_string()))
            }
            Some(_a) => Err(Error::Sbom(
                "individual project not yet implemented".to_string(),
            )),
        }
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
        let cx = dev_context(Some("core-test")).map_err(|e| Error::Config(e.to_string()))?;
        let storage: Box<dyn StorageProvider> = Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/vulnerability".to_string(),
        ));

        let mut task: Task = Task::new(TaskKind::Vulnerabilities(VulnerabilityProviderKind::Snyk))
            .map_err(|e| Error::Enrich(e.to_string()))?;

        let provider = SnykProvider::new_provider(cx, storage).await?;

        match provider.execute(&mut task).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Sbom(msg))
            }
        }
    }
}
