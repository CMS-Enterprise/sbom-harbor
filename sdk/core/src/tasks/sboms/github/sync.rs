use crate::entities::sboms::SbomProviderKind;
use crate::Error;

use crate::config::github_pat;
use crate::entities::tasks::Task;
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::github::client::Repo;
use crate::services::github::service::GitHubService;
use crate::services::github::Commit;
use crate::services::sboms::SbomService;
use crate::services::syft::Service as Syft;
use crate::tasks::sboms::github::BUILD_TARGETS;
use crate::tasks::TaskProvider;
use async_trait::async_trait;
use platform::persistence::mongodb::{Service, Store};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

/// Nifty function to merge HashMaps.
fn merge<K: Hash + Eq + Clone, V: Clone>(
    first_context: &HashMap<K, V>,
    second_context: &HashMap<K, V>,
) -> HashMap<K, V> {
    let mut new_context = HashMap::new();
    for (key, value) in first_context.iter() {
        new_context.insert(key.clone(), value.clone());
    }

    for (key, value) in second_context.iter() {
        new_context.insert(key.clone(), value.clone());
    }
    new_context
}

/// This function make sure that if there are certain directories in
/// the build target path, we should skip processing it as it is a dependency
/// rather than a primary target.  The immediate first example is node_modules.
/// node_modules is not supposed to be checked in, however at least one
/// example has been found and none of the packages under that directory
/// should be processed as primary build targets.
fn location_under_ignored_dir(build_target_location: String) -> bool {
    if build_target_location.contains("node_modules") {
        println!(
            "==> build target({}) is under node_modules, skipping",
            build_target_location
        );
        true
    } else {
        false
    }
}

/// Synchronizes SBOMS for a GitHub Group with Harbor.
#[derive(Debug)]
pub struct SyncTask {
    pub(in crate::tasks::sboms::github) github: GitHubService,
    sboms: SbomService,
}

#[async_trait]
impl TaskProvider for SyncTask {
    /// Builds the Packages Dependencies, Purls, and Unsupported from the GitHub API.
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        let mut repos: Vec<Repo> = match self.github.get_repos().await {
            Ok(value) => value,
            Err(err) => {
                println!("Error attempting to get the repos: {}", err);
                task.err_total += 1;
                Vec::new()
            }
        };

        let mut errors = HashMap::new();

        for repo in &mut repos {
            let name = repo.full_name.clone();
            let url = repo.html_url.clone();

            if !SyncTask::should_skip(repo, name, url.clone()) {
                match self.process_repo(repo, task).await {
                    Ok(option) => match option {
                        Some(raw_sboms) => {
                            for sbom in raw_sboms {
                                let ingest_errors =
                                    self.ingest_sbom(sbom, url.clone(), task).await?;
                                errors = merge(&errors, &ingest_errors);
                            }
                        }
                        None => println!("==> No work to do because the hashes matched"),
                    },
                    Err(err) => println!("==> Repo processing failure: {}", err),
                };
            }
        }

        Ok(errors.clone())
    }
}

impl Service<Task> for SyncTask {
    fn store(&self) -> Arc<Store> {
        self.github.store.clone()
    }
}

impl SyncTask {
    /* PRIVATE */

    async fn ingest_sbom(
        &self,
        sbom: String,
        url: String,
        task: &mut Task,
    ) -> Result<HashMap<String, String>, Error> {
        let mut errors: HashMap<String, String> = HashMap::new();

        let xref = Xref {
            kind: XrefKind::Product,
            map: HashMap::new(),
        };

        match self
            .sboms
            .ingest(
                sbom.as_str(),
                None,
                SbomProviderKind::HarborSyft,
                xref,
                Some(task),
            )
            .await
        {
            Ok(sbom) => {
                let opt = Some(sbom);
                println!("==> SBOM Ingestion went OK");
                opt
            }
            Err(err) => {
                task.err_total += 1;
                errors.insert(url.clone(), err.to_string());
                println!("==> SBOM Ingestion FAILED");
                None
            }
        };

        Ok(errors)
    }

    fn get_state_values(&self, repo: &Repo) -> Result<(String, String, String, String), Error> {
        let url: String = repo.html_url.clone();
        let full_name: String = repo.full_name.clone();

        Ok((url, full_name, repo.version.clone(), repo.last_hash.clone()))
    }

    /// Method to help finding documents in DocumentDB
    async fn find_commit(&self, last_hash: &str) -> Result<Option<Commit>, Error> {
        println!("==> Looking in Mongo for the document with id: {last_hash}");
        let option = self.github.find(last_hash).await?;
        Ok(option)
    }

    /// Should skip determines if the repository is disabled or
    /// archived and if so, skips processing them.
    fn should_skip(repo: &Repo, repo_name: String, url: String) -> bool {
        let mut skip: bool = false;

        match &repo.archived {
            Some(archived) => {
                if *archived {
                    println!("==> {} at {} is archived, skipping", repo_name, url);
                    skip = true;
                }
            }
            None => {
                println!("==> No value to determine if the repo is archived");
            }
        }

        match &repo.disabled {
            Some(disabled) => {
                if *disabled {
                    println!("==> {} at {} is disabled, skipping", repo_name, url);
                    skip = true;
                }
            }
            None => {
                println!("==> No value to determine if the repo is disabled, processing");
            }
        }

        if repo.empty {
            skip = true;
        }

        skip
    }

    /* PUBLIC */

    /// Factory method to create new instance of type.
    pub fn new(
        github: GitHubService,
        sboms: SbomService,
    ) -> Result<SyncTask, Error> {
        Ok(SyncTask {
            github,
            sboms,
        })
    }

    async fn update_db(&self, last_hash: &str, url: &str) -> Result<(), Error> {
        let mut commit = Commit {
            id: String::from(last_hash),
            url: String::from(url)
        };

        self.github.insert(&mut commit).await.map_err(
            |err| Error::GitHub(format!(
                "==> Failed to insert Commit into mongo with url: {}({})",
                url, err
            )))?;

        Ok(())
    }

    async fn process_repo(
        &self,
        repo: &Repo,
        task: &mut Task,
    ) -> Result<Option<Vec<String>>, Error> {

        let (url, full_name, version, last_hash) = self.get_state_values(repo)?;

        match self.find_commit(last_hash.as_str()).await? {

            // No Commit exists in the database, so it should be processed
            None => {

                println!("==> No document exists in mongo with last_hash({}), creating", last_hash);

                let pat = Some(github_pat()?);
                let clone_path = self.github
                    .clone_repo(url.as_str(), pat)
                    .map_err(|err| Error::GitHub(
                        format!("Error cloning Repo: {}", err)
                    ))?;
                let syft = Syft::new(clone_path.clone());

                let mut syft_results: Vec<String> = vec![];
                let mut total_build_targets = 0;

                for (cataloger, build_targets) in BUILD_TARGETS.iter() {
                    for build_target in build_targets {
                        println!(
                            "==> Looking for build targets named: {}, using cataloger: {}",
                            build_target, cataloger
                        );

                        // TODO refactor find_build_targets() to take a &str
                        let build_target_locations_result = self.github.find_build_targets(
                            url.to_string(),
                            String::from(*build_target),
                            clone_path.clone(),
                        );

                        let build_target_locations = match build_target_locations_result {
                            Ok(build_target_locations) => build_target_locations,
                            Err(err) => {
                                task.count += 1;
                                task.ref_errs(build_target.to_string(), err.to_string());
                                continue;
                            }
                        };

                        println!(
                            "==> Found ({}) build targets named: {}, using cataloger: {}",
                            build_target_locations.len(),
                            build_target,
                            cataloger
                        );

                        if !build_target_locations.is_empty() {
                            total_build_targets += build_target_locations.len();

                            for build_target_location in build_target_locations {
                                if !location_under_ignored_dir(build_target_location.clone()) {
                                    // Slice off the name of the file and the leading '/' as
                                    // execute() function only takes a path
                                    let search_string = format!("/{}", build_target);
                                    let trimmed_build_target_location =
                                        build_target_location.replace(search_string.as_str(), "");

                                    println!(
                                        "==> Running Syft in build target location ({}), using cataloger ({})",
                                        build_target_location, cataloger
                                    );

                                    // TODO Refactor execute() to take &str
                                    let syft_result = syft.execute(
                                        full_name.to_owned(),
                                        version.to_owned(),
                                        Some(String::from(*cataloger)),
                                        Some(trimmed_build_target_location),
                                    );

                                    let syft_result_value = match syft_result {
                                        Ok(value) => value,
                                        Err(err) => {
                                            task.ref_errs(build_target.to_string(), err.to_string());
                                            continue;
                                        }
                                    };

                                    syft_results.push(syft_result_value);
                                }
                            }
                        }
                    }
                }

                // If no build targets are found, run Syft without
                // a cataloger at the top level of the repo
                if total_build_targets == 0 {
                    println!(
                        "==> No build targets found in ({url}) hail mary using default cataloger at root",
                    );

                    syft_results.push(
                        syft.execute(full_name.to_owned(), version.to_string(), None, None)
                            .map_err(|err| {
                                Error::GitHub(format!("Error using Syft on a Repo: {}", err))
                            })?,
                    );
                }

                self.github.remove_clone(&clone_path).map_err(|err| {
                    Error::GitHub(format!("Error attempting to remove cloned Repo: {}", err))
                })?;

                self.update_db(last_hash.as_str(), url.as_str()).await?;
                println!("==> Updated Mongo, returning Syft Result");

                Ok(Some(syft_results))
            }

            // If a commit with that id exists already, then the repo has
            // already been processed and we can skip it.
            Some(_) => {
                println!("==> latest commit from repo ({}) has been found, skipping", repo.html_url);
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::services::github::client::{default_version, empty_string, Repo};
    use crate::tasks::sboms::github::sync::location_under_ignored_dir;
    use crate::tasks::sboms::github::SyncTask;

    #[test]
    fn test_location_under_ignored_dir_without_ignored_dir() {
        let build_target_path = "/some/perfectly/fine/path/pom.xml";
        assert!(!location_under_ignored_dir(build_target_path.to_string()));
    }

    #[test]
    fn test_location_under_ignored_dir_with_ignored_dir() {
        let build_target_path = "/some/horrible/node_modules/path/package.json";
        assert!(location_under_ignored_dir(build_target_path.to_string()));
    }

    #[tokio::test]
    async fn test_should_skip_archived() {
        let test_name = String::from("test name, ignore");
        let test_url = String::from("test url, ignore");

        let test_repo = Repo {
            full_name: empty_string(),
            html_url: empty_string(),
            default_branch: None,
            language: None,
            archived: Some(true),
            disabled: None,
            empty: false,
            last_hash: empty_string(),
            version: default_version(),
        };

        if !SyncTask::should_skip(&test_repo, test_name, test_url) {
            panic!("should_skip should be true for an archived repo"); // test panic
        } else {
            println!("Skipped! PASS");
        }
    }

    #[tokio::test]
    async fn test_should_skip_disabled() {
        let test_name = String::from("test name, ignore");
        let test_url = String::from("test url, ignore");

        let test_repo = Repo {
            full_name: empty_string(),
            html_url: empty_string(),
            default_branch: None,
            language: None,
            archived: None,
            disabled: Some(true),
            empty: false,
            last_hash: empty_string(),
            version: default_version(),
        };

        if !SyncTask::should_skip(&test_repo, test_name, test_url) {
            panic!("should_skip should be true for an disabled repo"); // test panic
        } else {
            println!("Skipped! PASS");
        }
    }

    #[tokio::test]
    async fn test_should_skip_empty() {
        let test_name = String::from("test name, ignore");
        let test_url = String::from("test url, ignore");

        let test_repo = Repo {
            full_name: empty_string(),
            html_url: empty_string(),
            default_branch: None,
            language: None,
            archived: None,
            disabled: None,
            empty: true,
            last_hash: empty_string(),
            version: default_version(),
        };

        if !SyncTask::should_skip(&test_repo, test_name, test_url) {
            panic!("should_skip should be true for an empty repo"); // test panic
        } else {
            println!("Skipped! PASS");
        }
    }
}
