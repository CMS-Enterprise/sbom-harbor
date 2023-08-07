use crate::commands::ingest::IngestArgs;
use crate::Error;
use clap::Parser;
use harbcore::entities::sboms::SbomProviderKind;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::services::github::mongo::GitHubProviderMongoService;
use harbcore::services::github::service::GitHubService;
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::{
    FileSystemStorageProvider, S3StorageProvider, SbomService, StorageProvider,
};
use harbcore::tasks::sboms::github::SyncTask;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::Store;
use std::string::ToString;
use std::sync::Arc;

/// Args for generating one or more SBOMs from a GitHub Organization.
#[derive(Clone, Debug, Parser)]
pub struct GitHubArgs {
    org: Option<String>,
}

pub(crate) async fn execute(args: &IngestArgs) -> Result<(), Error> {
    let storage: Box<dyn StorageProvider>;
    let token = harbcore::config::github_pat().map_err(|e| Error::Config(e.to_string()))?;

    let org = match &args.github_args {
        Some(gh_args) => match &gh_args.org {
            Some(org) => org.to_string(),

            // If no org in the arguments, error
            None => return Err(Error::Sbom("GitHub organization not specified".to_string())),
        },

        // If no GitHub arguments, error.
        None => return Err(Error::Sbom("GitHub organization not specified".to_string())),
    }
    .to_string();

    let cx = match &args.debug {
        false => {
            storage = Box::new(S3StorageProvider {});
            harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
        }
        true => {
            storage = Box::new(FileSystemStorageProvider::new(
                "/tmp/harbor-debug/sboms/github".to_string(),
            ));
            harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
        }
    };

    let store = Arc::new(
        Store::new(&cx)
            .await
            .map_err(|e| Error::Sbom(e.to_string()))?,
    );

    let package_service = PackageService::new(store.clone());

    let mongo_service = GitHubProviderMongoService::new(store.clone());

    let provider = SyncTask::new(
        mongo_service,
        GitHubService::new(org, token),
        SbomService::new(store.clone(), Some(storage), Some(package_service)),
    )
    .map_err(|e| Error::Sbom(e.to_string()))?;

    let task_kind = TaskKind::Sbom(SbomProviderKind::HarborSyft);

    let mut task: Task = Task::new(task_kind).map_err(|e| Error::Sbom(e.to_string()))?;

    provider
        .execute(&mut task)
        .await
        .map_err(|e| Error::Sbom(e.to_string()))
}

#[cfg(test)]
mod test {
    use crate::commands::ingest::github::{execute, GitHubArgs};
    use crate::commands::ingest::{IngestArgs, IngestionProviderKind};
    use crate::Error;

    #[tokio::test]
    async fn execute_on_harbor_test_org() -> Result<(), Error> {
        let gh_args = GitHubArgs {
            org: Some("harbor-test-org".to_string()),
        };

        let ingest_args = IngestArgs {
            provider: IngestionProviderKind::GitHub,
            debug: true,
            filesystem_args: None,
            github_args: Some(gh_args),
            snyk_args: None,
        };

        execute(&ingest_args)
        .await
        .expect("Panic at github ingest provider execute!");
        Ok(())
    }
}
