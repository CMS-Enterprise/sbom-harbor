use crate::commands::analyze::AnalyzeArgs;
use crate::Error;
use clap::Parser;
use harbcore::entities::analytics::AnalyticProviderKind;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::analytics::sboms::service::AnalyticService;
use harbcore::services::analytics::{
    FileSystemStorageProvider, S3StorageProvider, StorageProvider,
};
use harbcore::tasks::analytics::sboms::detail::DetailTask;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::{Context, Store};
use std::sync::Arc;

/// Args for generating a Detailed Report
#[derive(Clone, Debug, Parser)]
pub struct DetailArgs {}

/// Strategy pattern implementation that handles Report generation commands.
pub struct SbomDetailProvider {}

impl SbomDetailProvider {
    async fn new_provider(
        cx: Context,
        storage: Arc<dyn StorageProvider>,
    ) -> Result<DetailTask, Error> {
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Sbom(e.to_string()))?,
        );

        let provider = DetailTask::new(AnalyticService::new(store, storage));

        Ok(provider)
    }

    pub(crate) async fn execute(args: &AnalyzeArgs) -> Result<(), Error> {
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

                let provider: DetailTask = SbomDetailProvider::new_provider(cx, storage)
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
