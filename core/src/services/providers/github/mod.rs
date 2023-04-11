use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use platform::mongodb::{
    Service as MongoService
};

use crate::services::{
    clone_path,
    clone_repo,
    remove_clone,
    syft
};
use harbor_client::client::{
    Client as V1HarborClient,
    SBOMUploadResponse
};
use mongodb::Collection;
use platform::hyper::{
    ContentType,
    Error as HyperError,
    get
};
use platform::mongodb::{Service, Store};

use crate::clients::github::{
    Commit,
    Repo,
    clone_path,
    clone_repo,
    get_last_commit_url,
    get_page_of_repos,
    get_pages,
    remove_clone,
    should_skip,
    syft
};
use crate::config::*;
use crate::services::github::{get_harbor_config, GhProviderError};
use crate::services::github::mongo::GitHubProviderDocument;
use crate::services::providers::github::mongo::{get_mongo_db, GitHubProviderMongoService, update_last_hash_in_mongo};
use crate::services::providers::github::mongo::GitHubSbomProviderEntry;
use crate::services::providers::SbomProvider;

mod mongo;

/// Definition of the GitHubProvider
///
pub struct GitHubSbomProvider {}

/// GitHubProvider's own implementation
///
impl GitHubSbomProvider {

    async fn get_repos(org: String) -> Result<Vec<Repo>, GhProviderError> {

        let mut pages = match get_pages(&org).await {
            Ok(pages) => pages,
            Err(err) => panic!("Unable to get pages of repos from GitHub: {:#?}", err)
        };

        let gh_fetch_token = match get_env_var("GH_FETCH_TOKEN") {
            Some(value) => value,
            None => panic!("Missing GitHub Token. export GH_FETCH_TOKEN=<token>")
        };

        let token: String = String::from("Bearer ") + &gh_fetch_token;
        let mut repo_vec: Vec<Repo> = Vec::new();

        for (page, per_page) in pages.iter_mut().enumerate() {

            let mut gh_org_rsp = match get_page_of_repos(&org, page+1, per_page, &token).await {
                Ok(vector) => vector,
                Err(err) => return Err(
                    GhProviderError::GitHubRequest(
                        format!("Error getting a page of repos: {}", err)
                    )
                ),
            };

            for (repo_num, repo) in gh_org_rsp.iter_mut().enumerate() {

                let github_last_commit_url = get_last_commit_url(repo);

                println!("({}) Making call to {}", repo_num, github_last_commit_url);

                let response: Result<Option<Commit>, HyperError> = get(
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

                            Commit { sha: Some(String::from("<empty repo>")) }
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
}

/// The implementation of Provider for
/// GitHub Provider
///
#[async_trait]
impl SbomProvider for GitHubSbomProvider {

    async fn provide_sboms(&self) {

        let harbor_config = get_harbor_config().unwrap();

        let mut counter = Counter::default();

        println!("Scanning GitHub...");
        let org_name: String = String::from("cmsgov");
        let repos_result = GitHubSbomProvider::get_repos(org_name).await;
        let repos: Vec<Repo> = match repos_result {
            Ok(value) => value,
            Err(err) => panic!("Panic trying to extract value from Result: {}", err),
        };

        for repo in repos.iter() {

            let name = repo.full_name.clone().unwrap();
            let url = repo.html_url.clone().unwrap();

            if !should_skip(repo, name, url, &mut counter) {
                process_repo(repo, &harbor_config, &mut counter).await;
            }
        }

        println!("Collection Run Complete: {:#?}", counter);
    }
}

/// Create the entities like project and codebase
/// in Harbor V1.
///
async fn create_harbor_entities(
    github_url: String,
    harbor_config: &HarborConfig,
    language: String,
) -> Result<HashMap<String, String>, GhProviderError> {

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
                GhProviderError::EntityCreation(err_msg)
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
    harbor_config: &HarborConfig,
) -> Result<SBOMUploadResponse, GhProviderError> {

    let clone_path = clone_path(&document.repo_url);

    match clone_repo(&clone_path, &document.repo_url.as_str()) {
        Ok(()) => println!("{} Cloned Successfully", document.repo_url),
        Err(err) => panic!("Unable to clone Repo {}, {}", document.repo_url, err)
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
            GhProviderError::Pilot(
                String::from(format!("Pilot Error: {}", err))
            )
        )
    }
}

/// Given the Mongo Collection we want to put a document into
/// and the Harbor Entities, put this document in Mongo
///
async fn create_document_in_db(
    entities: HashMap<String, String>,
    collection: Collection<GitHubSbomProviderEntry>,
    url: String,
    last_hash: String,
) -> Result<GitHubSbomProviderEntry, GhProviderError> {

    let document = GitHubSbomProviderEntry {
        id: Uuid::new_v4().to_string(),
        repo_url: url.to_string(),
        last_hash: last_hash.to_string(),
        team_id: entities.get(TEAM_ID_KEY).unwrap().to_string(),
        project_id: entities.get(PROJECT_ID_KEY).unwrap().to_string(),
        codebase_id: entities.get(CODEBASE_ID_KEY).unwrap().to_string(),
    };

    match collection.insert_one(document.clone(), None).await {
        Ok(result) => result,
        Err(err) => return Err(
            GhProviderError::MongoDb(
                format!("Error inserting into mongo: {}", err)
            )
        ),
    };

    println!("PROCESSING> Added NEW Document to MongoDB: {:#?}", document);

    Ok(document)
}

async fn create_data_structures(
    repo: &Repo,
    harbor_config: &HarborConfig,
    url: String,
    collection: Collection<GitHubSbomProviderEntry>,
) -> Result<GitHubSbomProviderEntry, GhProviderError> {

    // TODO This will create entries without the ability to rollback.
    //  Probably don't care because it's only v1.

    println!("PROCESSING> No Document found document in mongo. Creating entities in harbor");

    // This arm executes when nothing is found in the database associated
    // to the given repo_url.  This means we need to create the project and codebase
    // in Harbor before we can send SBOMs to that target.

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
        collection,
        url.clone(),
        String::from(""),
    ).await
}

async fn process_repo(repo: &Repo, harbor_config: &HarborConfig, counter: &mut Counter) {

    let mongo_service = GitHubProviderMongoService {
        name: String::from("Test name whatever")
    };

    let url: &String = match &repo.html_url {
        Some(url) => url,
        None => panic!("No URL for Repository"),
    };

    let repo_name: &String = match &repo.full_name {
        Some(name) => name,
        None => panic!("No Full Name for Repository"),
    };

    let last_hash: &String = &repo.last_hash;

    println!("PROCESSING> Will be processing {}@{}", repo_name, url);

    let db = match get_mongo_db().await {
        Ok(db) => db,
        Err(err) => panic!("Problem getting DB: {:#?}", err)
    };

    let collection = db.collection::<GitHubSbomProviderEntry>(COLLECTION);
    let filter = doc! { "repo_url":  url.as_str() };

    println!("PROCESSING> Looking in Mongo for this document: {:#?}", filter);

    let doc_option: Option<GitHubSbomProviderEntry>
        = match collection.find_one(filter, None).await {
        Ok(option) => option,
        Err(err) => panic!("Cursor - Error: {}", err)
    };

    let document = match doc_option {
        Some(document) => document,
        None => match create_data_structures(
            repo,
            harbor_config,
            url.clone(),
            collection.clone(),
        ).await {
            Ok(document) => document,
            Err(err) => panic!("Error creating data structures in Mongo or Harbor: {}", err)
        }
    };

    println!("Comparing Repo({}) to MongoDB({})", last_hash, document.last_hash);

    if last_hash.to_string() != document.last_hash {

        println!("PROCESSING> Hashes are not equal, sending to pilot");

        // Use the document to construct a request to Pilot
        match send_to_v1(&document, &harbor_config).await {
            Ok(upload_resp) => {

                // Upload is OK, update Mongo

                println!("PROCESSING> One SBOM Down! {:#?}", upload_resp);
                update_last_hash_in_mongo(document, collection, last_hash.clone()).await;
                counter.processed += 1;
            },
            Err(err) => {
                counter.upload_errors += 1;
                println!("Error Uploading SBOM!! {:#?}", err)
            }
        }
    } else {
        counter.hash_matched += 1;
        println!("PROCESSING> Hashes are equal, skipping pilot");
    }
}


/// Args for generating one ore more SBOMs from a GitHub Organization.
#[derive(Clone, Debug, Parser)]
pub struct GitHubProviderConfig {

    /// This is the GUID that is in DynamoDB that
    /// belongs to the team we are using.
    cms_team_id: String,

    /// This is the token from that team
    cms_team_token: String,

    /// This is the Cloudfront Domain of the API endpoints
    cf_domain: String,

    /// The username we use to get the JWT and make API calls
    cognito_username: String,

    /// The password we use to get the JWT and make API calls
    cognito_password: String,
}

#[derive(Debug)]
pub struct GitHubProviderService {
    store: Arc<Store>,
}

impl GitHubProviderService {
    /// Factory method to create new instances of a [TeamService].
    pub fn new(store: Arc<Store>) -> GitHubProviderService {
        GitHubProviderService { store }
    }
}

impl Service<GitHubSbomProviderEntry> for GitHubProviderService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

/// Snag a bunch of environment variables and put them into a struct
///
fn get_harbor_config() -> Result<GitHubProviderConfig, GhProviderError> {

    let cms_team_id = match get_cms_team_id() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team Id of V1 Team")
            )
        )
    };

    let cms_team_token = match get_cms_team_token() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team token of V1 Team")
            )
        )
    };

    let cf_domain = match get_cf_domain() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_username = match get_v1_username() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_password = match get_v1_password() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Password")
            )
        )
    };

    Ok(
        GitHubProviderConfig {
            cms_team_id,
            cms_team_token,
            cf_domain,
            cognito_username,
            cognito_password,
        }
    )
}

/// The Counter struct is used to keep track of
/// what happened to an attempt to submit an SBOM.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Counter {

    /// This value is incremented if the Repo is archived
    archived: i32,

    /// This value is incremented if the Repo is disabled
    disabled: i32,

    /// This value is incremented if the Repo is empty
    empty: i32,

    /// This value is incremented if the Repo is processed successfully
    processed: i32,

    /// This value is incremented if the last commit hash of
    /// the repo is in the database already. This happens when
    /// there has been no change in the repo since last run
    hash_matched: i32,

    /// This value is incremented if there is an error when trying to upload the SBOM.
    upload_errors: i32,
}

/// Default, completely 0'd out default Counter
///
impl Default for Counter {
    fn default() -> Self {
        Self {
            archived: 0,
            disabled: 0,
            empty: 0,
            processed: 0,
            hash_matched: 0,
            upload_errors: 0,
        }
    }
}

/// Represents all handled Errors for the GitHub Crawler.
///
#[derive(Error, Debug)]
pub enum GhProviderError {

    /// Raised when we have a generic MongoDB Error
    #[error("error getting database: {0}")]
    MongoDb(String),

    /// This is raised when there is an issue creating entities
    #[error("error creating Harbor v1 entities: {0}")]
    EntityCreation(String),

    /// This is raised when there is a problem getting
    /// configuration from the environment.
    #[error("error creating Harbor v1 entities: {0}")]
    Configuration(String),

    /// This is Raised when the Pilot has issues doing its job
    #[error("error running pilot: {0}")]
    Pilot(String),

    /// This error is raised when there is a problem communicating
    /// with GitHub over HTTP.
    #[error("error running pilot: {0}")]
    GitHubRequest(String),
}

#[tokio::test]
async fn test_get_github_data() {
    let provider = GitHubSbomProvider {};
    provider.provide_sboms().await;
}