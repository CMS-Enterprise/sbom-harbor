/// Publish the GitHub sync module
pub mod sync;

use std::collections::HashMap;
pub use sync::*;

/// Temporary way to associate build files with Syft catalogers.  Syft
/// can use the same cataloger for multiple types of files, that is the reason
/// we need to use a vector to manage them.
pub fn get_cataloger_to_build_target_map() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    // Add java pom.xml
    map.insert("java-pom".to_string(), vec!["pom.xml".to_string()]);

    // Add support for finding javascript build files
    map.insert(
        "javascript-package".to_string(),
        vec!["package.json".to_string()],
    );

    // Add support for python requirements.txt build files
    map.insert(
        "python-index".to_string(),
        vec!["requirements.txt".to_string()],
    );

    // Add support for Ruby Gemfiles
    map.insert("ruby-gemfile".to_string(), vec!["Gemfile".to_string()]);

    map
}

#[cfg(test)]
mod tests {
    use crate::config_util::{dev_context, snyk_token};
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
            SbomService::new(store.clone(), Some(storage), Some(package_service)),
        )?;

        Ok(provider)
    }
}
