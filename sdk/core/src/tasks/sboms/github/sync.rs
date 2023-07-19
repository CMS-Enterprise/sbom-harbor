use crate::entities::sboms::SbomProviderKind;
use crate::Error;

use crate::entities::tasks::Task;
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::github::client::Repo;
use crate::services::github::mongo::GitHubProviderMongoService;
use crate::services::github::service::GitHubService;
use crate::services::github::Commit;
use crate::services::sboms::SbomService;
use crate::services::syft::Service as Syft;
use crate::tasks::sboms::github::get_cataloger_to_build_target_map;
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
    mongo_svc: GitHubProviderMongoService,
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

        let cataloger_to_build_file_map = get_cataloger_to_build_target_map();
        println!(
            "==> Acquired cataloger to build target map, map contains: {} keys",
            cataloger_to_build_file_map.len()
        );

        for repo in &mut repos {
            let name = repo
                .full_name
                .clone()
                .unwrap_or(String::from("name/missing"));

            let url = match repo.html_url.clone() {
                Some(url) => url,
                None => continue,
            };

            if !SyncTask::should_skip(repo, name, url.clone()) {
                match self
                    .process_repo(repo, task, &cataloger_to_build_file_map)
                    .await
                {
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
        self.mongo_svc.store.clone()
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
                task,
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

    fn get_state_values(&self, repo: &Repo) -> Result<(String, String), Error> {
        let url: String = match repo.html_url.clone() {
            Some(url) => url,
            None => return Err(Error::GitHub("==> No URL for Repository".to_string())),
        };

        let full_name: String = match repo.full_name.clone() {
            Some(full_name) => full_name,
            None => return Err(Error::GitHub("==> Repo is missing full_name".to_string())),
        };

        Ok((url, full_name))
    }

    /// Method to help finding documents in DocumentDB
    async fn find_document(&self, id: &str) -> Result<Commit, Error> {
        println!("==> Looking in Mongo for the document with id: {id}");

        match self.mongo_svc.find(id).await {
            Ok(option) => match option {
                Some(document) => {
                    println!("==> Got a Document From Mongo!");
                    Ok(document)
                }
                None => {
                    println!("==> No document exists in mongo with the id: {}", id);
                    self.mongo_svc
                        .create_document(String::from(id), String::from(""))
                        .await
                        .map_err(|e| Error::GitHub(e.to_string()))
                }
            },
            Err(err) => Err(Error::GitHub(format!(
                "==> Unable to find document in mongo with url: {}({})",
                id, err
            ))),
        }
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
        mongo_svc: GitHubProviderMongoService,
        github: GitHubService,
        sboms: SbomService,
    ) -> Result<SyncTask, Error> {
        Ok(SyncTask {
            mongo_svc,
            github,
            sboms,
        })
    }

    async fn process_repo(
        &self,
        repo: &mut Repo,
        task: &mut Task,
        cataloger_to_build_file_map: &HashMap<String, Vec<String>>,
    ) -> Result<Option<Vec<String>>, Error> {
        let (url, full_name) = self.get_state_values(repo)?;

        // The url is the id of the document
        let mut document = match self.find_document(url.as_str()).await {
            Ok(document) => document,
            Err(err) => {
                return Err(Error::GitHub(format!(
                    "Error attempting to find document: {}",
                    err
                )))
            }
        };

        let last_hash: &String = &repo.last_hash;

        println!(
            "==> Comparing Repo({}) to MongoDB({})",
            last_hash,
            document.last_hash.clone().unwrap()
        );

        if *last_hash != document.last_hash.unwrap() {
            let url = &document.id;

            let clone_path = self
                .github
                .clone_repo(url.as_str())
                .map_err(|err| Error::GitHub(format!("Error attempting clone repo: {}", err)))?;

            let syft = Syft::new(clone_path.clone());

            let mut syft_results: Vec<String> = vec![];

            let mut total_build_targets = 0;

            for (cataloger, build_targets) in cataloger_to_build_file_map {
                for build_target in build_targets {
                    println!(
                        "==> Looking for build targets named: {}, using cataloger: {}",
                        build_target, cataloger
                    );

                    let build_target_locations_result =
                        self.github
                            .find(url.to_string(), build_target.clone(), clone_path.clone());

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
                                let search_string = format!("/{}", build_target.as_str());
                                let trimmed_build_target_location =
                                    build_target_location.replace(search_string.as_str(), "");

                                println!(
                                    "==> Running Syft in build target location ({}), using cataloger ({})",
                                    build_target_location, cataloger
                                );

                                let syft_result = syft.execute(
                                    full_name.to_owned(),
                                    last_hash.to_string(),
                                    Some(cataloger.clone()),
                                    Some(trimmed_build_target_location),
                                );

                                let syft_result_value = match syft_result {
                                    Ok(value) => value,
                                    Err(err) => {
                                        task.count += 1;
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
                    syft.execute(full_name.to_owned(), last_hash.to_string(), None, None)
                        .map_err(|err| {
                            Error::GitHub(format!("Error using Syft on a Repo: {}", err))
                        })?,
                );
            }

            self.github.remove_clone(&clone_path).map_err(|err| {
                Error::GitHub(format!("Error attempting to remove cloned Repo: {}", err))
            })?;

            document.last_hash = Some(last_hash.to_string());
            match self.mongo_svc.update(&document).await {
                Ok(_) => {}
                Err(err) => {
                    println!("==> Mongo service error!! {}", err);
                    return Err(Error::GitHub(err.to_string()));
                }
            };

            println!("==> Updated Mongo, returning Syft Result");
            Ok(Some(syft_results))
        }
        // The last commit hash on the master/main GitHub matched the one in Mongo
        else {
            println!("==> Hashes are equal, skipping.");
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::services::github::client::Repo;
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
            full_name: None,
            html_url: None,
            default_branch: None,
            language: None,
            archived: Some(true),
            disabled: None,
            empty: false,
            last_hash: "".to_string(),
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
            full_name: None,
            html_url: None,
            default_branch: None,
            language: None,
            archived: None,
            disabled: Some(true),
            empty: false,
            last_hash: "".to_string(),
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
            full_name: None,
            html_url: None,
            default_branch: None,
            language: None,
            archived: None,
            disabled: None,
            empty: true,
            last_hash: "".to_string(),
        };

        if !SyncTask::should_skip(&test_repo, test_name, test_url) {
            panic!("should_skip should be true for an empty repo"); // test panic
        } else {
            println!("Skipped! PASS");
        }
    }
}
