use std::collections::HashMap;

use async_trait::async_trait;
use mockall::mock;

use harbor_client::client::{
    Client as V1HarborClient,
    SBOMUploadResponse
};
use platform::hyper::{
    ContentType,
    Error as HyperError,
    Http,
    Hyper
};
use platform::mongodb::{
    Service as MongoService,
    Store as MongoStore
};

use crate::clients::github::{
    clone_path,
    clone_repo,
    Commit,
    get_last_commit_url,
    get_page_of_repos,
    get_pages,
    remove_clone,
    Repo,
    should_skip,
    syft
};
use crate::config::*;
use crate::services::providers::github::counter::Counter;
use crate::services::providers::github::env::GitHubProviderEnvironmentConfig;
use crate::services::providers::github::error::Error;
use crate::services::providers::github::mongo::{
    get_default_context,
    GitHubProviderMongoService
};
use crate::services::providers::github::mongo::GitHubSbomProviderEntry;
use crate::services::providers::SbomProvider;

mod mongo;
pub mod counter;
pub(crate) mod error;
mod env;

/// Definition of the GitHubProvider
///
pub struct GitHubSbomProvider {
    org: Option<String>,
}

/// Impl for GitHubSbomProvider
///
impl GitHubSbomProvider {

    /// new method sets the organization for the struct
    ///
    pub fn new(org: String) -> Result<Self, Error> {
        Ok(
            GitHubSbomProvider {
                org: Some(org)
            }
        )
    }
}

/// The implementation of Provider for
/// GitHub Provider
///
#[async_trait]
impl SbomProvider<(), Error> for GitHubSbomProvider {

    async fn provide_sboms(&self) -> Result<(), Error> {

        // Here is where we create the Http type to thread in...
        let http = Hyper::new();

        println!("Scanning GitHub...");

        let harbor_config = match GitHubProviderEnvironmentConfig::extract() {
            Ok(config) => config,
            Err(err) => panic!("Error trying to extract config from environment: {}", err)
        };

        let mut counter = Counter::default();

        let org_name: String = match &self.org {
            Some(org_name) => org_name,
            None => panic!("No organization name provided, quitting...")
        }.to_string();

        let repos: Vec<Repo> = match get_repos(org_name, http).await {
            Ok(value) => value,
            Err(err) => panic!("Panic trying to extract value from Result: {}", err),
        };

        for repo in repos.iter() {

            let name = repo.full_name.clone().unwrap_or(
                String::from("<Name Missing>")
            );

            // TODO if this is empty, we have nothing to do.
            let url = repo.html_url.clone().unwrap();

            if !should_skip(repo, name, url.clone(), &mut counter) {
                match process_repo(repo, &harbor_config, &mut counter).await {
                    Ok(_) => println!("PROCESSING> One Repo Down. {} is ok.", url),
                    Err(err) => println!("PROCESSING> Repo processing failure: {}", err)
                }
            }
        }

        println!("Collection Run Complete: {:#?}", counter);

        Ok(())
    }
}

async fn get_repos(org: String, http: Hyper) -> Result<Vec<Repo>, Error> {

    let mut pages = match get_pages(&org, http.as_ref()).await {
        Ok(pages) => pages,
        Err(err) => panic!("Unable to get pages of repos from GitHub: {:#?}", err)
    };

    let token = match get_gh_token() {
        Ok(value) => value,
        Err(err) => return Err(
            Error::Configuration(
                String::from(
                    format!("GitHub token not in environment {}", err)
                )
            )
        )
    };

    let token: String = String::from("Bearer ") + &token;
    let mut repo_vec: Vec<Repo> = Vec::new();

    for (page, per_page) in pages.iter_mut().enumerate() {

        let mut gh_org_rsp = match get_page_of_repos(&org, page+1, per_page, &token, http.as_ref()).await {
            Ok(vector) => vector,
            Err(err) => return Err(
                Error::GitHubRequest(
                    format!("Error getting a page of repos: {}", err)
                )
            ),
        };

        for (repo_num, repo) in gh_org_rsp.iter_mut().enumerate() {

            let github_last_commit_url = get_last_commit_url(repo);

            println!("({}) Making call to {}", repo_num, github_last_commit_url);

            let response: Result<Option<Commit>, HyperError> = hyper.get(
                github_last_commit_url.as_str(),
                ContentType::Json,
                token.as_str(),
                None::<String>,
            ).await;

            let gh_commits_rsp = match response {
                Ok(option) => match option {
                    Some(value) => value,
                    None => panic!("Nothing in here!"),
                },
                Err(err) => {
                    if let HyperError::Remote(status, _msg) = err {

                        if status == 409 {
                            repo.mark_repo_empty();
                        }

                        Commit {
                            sha: Some(String::from("<empty repo>"))
                        }
                    } else {
                        panic!("No matching Error Type: {}", err)
                    }
                },
            };

            match gh_commits_rsp.sha {
                Some(val) => repo.add_last_hash(val),
                None => panic!("No value for commit found!")
            }
        }

        repo_vec.extend(gh_org_rsp);
    }

    Ok(repo_vec)
}

/// Create the entities like project and codebase
/// in Harbor V1.
///
async fn create_harbor_entities(
    github_url: String,
    harbor_config: &GitHubProviderEnvironmentConfig,
    language: String,
) -> Result<HashMap<String, String>, Error> {

    let client = V1HarborClient::new(
        harbor_config.cf_domain.clone(),
        harbor_config.cognito_username.clone(),
        harbor_config.cognito_password.clone(),
    ).await.unwrap();

    let result = client.create_project(
        harbor_config.cms_team_id.clone(),
        github_url,
        language,
    ).await;

    let project = match result {
        Ok(project) => project,
        Err(err) => {
            let err_msg = format!("Error trying to create a project: {}", err);
            return Err(
                Error::EntityCreation(err_msg)
            )
        }
    };

    // Each project will have only one codebase in this implementation
    let codebase = project.codebases.into_iter().nth(0);

    let mut id_map = HashMap::new();

    id_map.insert(
        String::from(TEAM_ID_KEY),
        harbor_config.cms_team_id.to_string(),
    );

    id_map.insert(
        String::from(PROJECT_ID_KEY),
        project.id
    );

    id_map.insert(
        String::from(CODEBASE_ID_KEY),
        codebase.unwrap().id
    );

    Ok(id_map)
}

/// Clones a repo, generates an SBOM, and then uploads to the Enrichment Engine.
///
async fn send_to_v1(
    document: &GitHubSbomProviderEntry,
    harbor_config: &GitHubProviderEnvironmentConfig,
) -> Result<SBOMUploadResponse, Error> {

    let clone_path = clone_path(&document.id);

    match clone_repo(&clone_path, &document.id.as_str()) {
        Ok(()) => println!("{} Cloned Successfully", document.id),
        Err(err) => panic!("Unable to clone Repo {}, {}", document.id, err)
    }

    let syft_result = match syft(&clone_path) {
        Ok(map) => map,
        Err(err) => panic!("Unable to Syft the Repo we cloned [{}]??", err)
    };

    match remove_clone(&clone_path) {
        Ok(()) => println!("Clone removed successfully"),
        Err(err) => panic!("Unable to blow away the clone [{}]??", err)
    };

    match V1HarborClient::upload_sbom(
        harbor_config.cf_domain.as_str(),
        harbor_config.cms_team_token.as_str(),
        harbor_config.cms_team_id.clone(),
        document.project_id.clone(),
        document.codebase_id.clone(),
        syft_result,
    ).await {
        Ok(response) => Ok(response),
        Err(err) => return Err(
            Error::SbomUpload(
                String::from(format!("Error uploading SBOM: {}", err))
            )
        )
    }
}

/// Given the Mongo Collection we want to put a document into
/// and the Harbor Entities, put this document in Mongo
///
async fn create_document_in_db(
    entities: HashMap<String, String>,
    mongo_service: GitHubProviderMongoService,
    url: String,
    last_hash: String,
) -> Result<GitHubSbomProviderEntry, Error> {

    let mut document = GitHubSbomProviderEntry {
        id: url.to_string(),
        last_hash: last_hash.to_string(),
        team_id: entities.get(TEAM_ID_KEY).unwrap().to_string(),
        project_id: entities.get(PROJECT_ID_KEY).unwrap().to_string(),
        codebase_id: entities.get(CODEBASE_ID_KEY).unwrap().to_string(),
    };

    match mongo_service.insert(&mut document).await {
        Ok(result) => result,
        Err(err) => return Err(
            Error::MongoDb(
                format!("Error inserting into mongo: {}", err)
            )
        ),
    };

    println!("PROCESSING> Added NEW Document to MongoDB: {:#?}", document);

    Ok(document)
}

/// This function executes when nothing is found in the database associated
/// to the given id.  This means we need to create the project and codebase
/// in Harbor before we can send SBOMs to that target.
///
async fn create_data_structures(
    repo: &Repo,
    harbor_config: &GitHubProviderEnvironmentConfig,
    url: String,
    mongo_service: GitHubProviderMongoService,
) -> Result<GitHubSbomProviderEntry, Error> {

    println!("PROCESSING> No Document found document in mongo. Creating entities in harbor");

    let entities: HashMap<String, String> = match create_harbor_entities(
        url.clone(),
        harbor_config,
        match &repo.language {
            Some(language) => language.to_string(),
            None => String::from("None"),
        },
    ).await {
        Ok(entities) => entities,
        Err(err) => panic!("Unable to create Harbor entities, {:#?}", err),
    };

    create_document_in_db(
        entities,
        mongo_service,
        url.clone(),
        String::from(""),
    ).await
}

async fn process_repo(
    repo: &Repo,
    harbor_config: &GitHubProviderEnvironmentConfig,
    counter: &mut Counter
) -> Result<(), Error> {

    let ctx = get_default_context();

    let mongo_service = GitHubProviderMongoService::new(
        match MongoStore::new(&ctx).await {
            Ok(store) => store,
            Err(err) => panic!("PROCESSING> Error getting store: {}", err)
        }
    );

    let url: &String = match &repo.html_url {
        Some(url) => url,
        None => panic!("PROCESSING> No URL for Repository"),
    };

    let last_hash: &String = &repo.last_hash;

    println!("PROCESSING> Looking in Mongo for the document with repo url: {}", url);

    let doc_option = match mongo_service.find(url).await {
        Ok(option) => option,
        Err(err) => panic!(
            "PROCESSING> Error attempting to find document in mongo with url: {}, Error: {}",
            url, err
        )
    };

    let mut document = match doc_option {
        Some(document) => {
            println!("PROCESSING> Got a Document From Mongo!");
            document
        },
        None => {
            println!("PROCESSING> No document exists in mongo with the id: {}", url);
            match create_data_structures(
                repo,
                harbor_config,
                url.clone(),
                mongo_service.clone(),
            ).await {
                Ok(document) => document,
                Err(err) => panic!(
                    "PROCESSING> Error creating data structures in Mongo or Harbor: {}",
                    err
                )
            }
        }
    };

    println!("PROCESSING> Comparing Repo({}) to MongoDB({})", last_hash, document.last_hash);

    return if last_hash.to_string() != document.last_hash {

        println!("PROCESSING> Hashes are not equal, sending to v1");

        // Use the document to construct a request to Harbor v1
        match send_to_v1(&document, &harbor_config).await {

            // Upload is OK, update Mongo
            Ok(_) => {
                document.last_hash = last_hash.to_string();
                match mongo_service.update(&document).await {

                    // Mongo update went OK.
                    Ok(()) => {
                        counter.processed += 1;
                        Ok(())
                    },

                    // Mongo update failed!
                    Err(err) => {
                        counter.store_error += 1;
                        println!("PROCESSING> Mongo service error!! {:#?}", err);
                        Err(Error::MongoDb(String::from(err.to_string())))
                    }
                }
            },

            // Error trying to upload
            Err(err) => {
                counter.upload_errors += 1;
                println!("PROCESSING> Error Uploading SBOM!! {:#?}", err);
                Err(Error::SbomUpload(String::from(err.to_string())))
            }
        }
    }

    // The last commit hash on the master/main GitHub matched the one in Mongo
    else {
        counter.hash_matched += 1;
        println!("PROCESSING> Hashes are equal, skipping sending to v1");
        Ok(())
    }
}

mock! {
    Hyper {}
}

#[tokio::test]
async fn test_get_github_data() {

    let provider = GitHubSbomProvider {
        org: Some(
            String::from("cmsgov")
        )
    };

    match provider.provide_sboms().await {
        Ok(_) => println!("FINISHED!"),
        Err(_) => panic!("Error getting github data in test")
    }
}

#[tokio::test]
async fn test_create_entry_in_store() {

    let entities: HashMap<String, String> = HashMap::from(
        [
            (TEAM_ID_KEY.to_string(), "team_id".to_string()),
            (PROJECT_ID_KEY.to_string(), "project_id".to_string()),
            (CODEBASE_ID_KEY.to_string(), "codebase_id".to_string()),
        ]
    );

    let store = match MongoStore::new(&get_default_context()).await {
        Ok(store) => store,
        Err(err) => panic!("Error in test: {}", err)
    };

    let mongo_service = GitHubProviderMongoService::new(store);

    let url: String = String::from("http://test.repos");
    let last_hash: String = String::from("abc123def456");

    match create_document_in_db(entities, mongo_service, url, last_hash).await {
        Ok(_) => println!("FINISHED!"),
        Err(_) => panic!("Error getting github data in test")
    }
}

