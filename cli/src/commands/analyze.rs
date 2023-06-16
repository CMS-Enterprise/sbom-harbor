use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use harbcore::entities::analytics::AnalyticProviderKind;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::analytics::sboms::provider::SbomDetailTask;
use harbcore::services::analytics::sboms::service::AnalyticService;
use harbcore::services::analytics::{
    FileSystemStorageProvider, S3StorageProvider, StorageProvider,
};
use harbcore::services::tasks::TaskProvider;
use platform::persistence::mongodb::{Context, Store};
use std::sync::Arc;

/// The SBOM Command handler.
pub async fn execute(args: &AnalyzeArgs) -> Result<(), Error> {
    match args.provider {
        AnalysisProviderKind::SbomDetail => SbomDetailProvider::execute(args).await,
    }
}

/// Enumerates which SBOM analysis provider to employ.
#[derive(Clone, Debug)]
pub(crate) enum AnalysisProviderKind {
    /// Generate a Detailed report from SBOM data.
    SbomDetail,
}

impl ValueEnum for AnalysisProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::SbomDetail]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            AnalysisProviderKind::SbomDetail => PossibleValue::new("detail")
                .help("Generates a detailed analysis of all SBOMs and related enrichment data."),
        })
    }
}

/// Specifies the CLI args for the SBOM command.
#[derive(Debug, Parser)]
pub struct AnalyzeArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,

    /// Specifies the type of provider
    provider: AnalysisProviderKind,

    /// Flattened args for use with the Detail Provider
    #[command(flatten)]
    pub detail_args: Option<DetailArgs>,
}

/// Args for generating a Detailed Report
#[derive(Clone, Debug, Parser)]
pub struct DetailArgs {}

/// Strategy pattern implementation that handles Report generation commands.
struct SbomDetailProvider {}

impl SbomDetailProvider {
    async fn new_provider(
        cx: Context,
        storage: Arc<dyn StorageProvider>,
    ) -> Result<SbomDetailTask, Error> {
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Sbom(e.to_string()))?,
        );

        let provider = SbomDetailTask::new(AnalyticService::new(store, storage));

        Ok(provider)
    }

    async fn execute(args: &AnalyzeArgs) -> Result<(), Error> {
        let storage: Arc<dyn StorageProvider>;

        let cx = match &args.debug {
            false => {
                storage = Arc::new(S3StorageProvider {});
                harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
            }
            true => {
                storage = Arc::new(FileSystemStorageProvider::new(
                    "/tmp/harbor-debug/analyze/sbom-detail".to_string(),
                ));
                harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
            }
        };

        match &args.detail_args {
            None => {
                let mut task: Task =
                    Task::new(TaskKind::Analytics(AnalyticProviderKind::SbomDetail))
                        .map_err(|e| Error::Analyze(e.to_string()))?;

                let provider: SbomDetailTask = SbomDetailProvider::new_provider(cx, storage)
                    .await
                    .map_err(|e| Error::Analyze(e.to_string()))?;

                provider
                    .execute(&mut task)
                    .await
                    .map_err(|e| Error::Analyze(e.to_string()))
            }
            Some(_a) => Err(Error::Analyze(String::from(
                "individual purl not yet implemented",
            ))),
        }
    }
}
