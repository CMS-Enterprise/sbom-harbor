/// Publish the GitHub sync module
pub mod sync;

use std::collections::HashMap;
pub use sync::*;

lazy_static! {

    /// Static HashMap of Syft catalogers to build targets. This is used to associate
    /// build files with Syft catalogers.  Syft can use the same cataloger for multiple
    /// types of files, that is the reason we need to use a vector to manage them.
    static ref BUILD_TARGETS: HashMap<&'static str, Vec<&'static str>> = {
            HashMap::from([
            ("java-pom-cataloger", vec!["pom.xml"]),
            ("javascript-package-cataloger", vec!["package.json"]),
            ("python-index-cataloger", vec!["requirements.txt"]),
            ("ruby-gemfile-cataloger", vec!["Gemfile"]),
        ])
    };
}

#[cfg(test)]
mod tests {
    use crate::config::{dev_context, snyk_token};
    use crate::entities::tasks::{Task, TaskKind};
    use crate::services::github::mongo::GitHubProviderMongoService;
    use crate::services::github::service::GitHubService;
    use crate::services::packages::PackageService;
    use crate::services::sboms::{FileSystemStorageProvider, SbomService};
    use crate::tasks::sboms::github::sync::SyncTask;
    use crate::tasks::TaskProvider;
    use crate::{entities, Error};
    use platform::persistence::mongodb::Store;
    use std::sync::Arc;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_process_sboms() -> Result<(), Error> {
        let provider = test_provider().await?;

        let mut task = Task::new(TaskKind::Sbom(entities::sboms::SbomProviderKind::Snyk))?;

        match provider.execute(&mut task).await {
            Ok(()) => {}
            Err(e) => {
                let err_msg = format!("cannot_process_sboms::{}", e);
                let gh_error = Error::GitHub(err_msg);
                return Err(gh_error);
            }
        };

        Ok(())
    }

    async fn test_provider() -> Result<SyncTask, Error> {
        let org = String::from("cmsgov");

        let token = snyk_token()?;
        let cx = dev_context(None)?;
        let store = Arc::new(Store::new(&cx).await?);
        let mongo_service = GitHubProviderMongoService::new(store.clone());
        let package_service = PackageService::new(store.clone());
        let storage = Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/sboms/github".to_string(),
        ));

        let provider = SyncTask::new(
            mongo_service,
            GitHubService::new(org, token),
            SbomService::new(store, Some(storage), Some(package_service)),
        )?;

        Ok(provider)
    }
}
