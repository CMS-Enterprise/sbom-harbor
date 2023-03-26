use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::fmt::Error;
use std::ops::Deref;
use std::pin::Pin;

use anyhow::{anyhow, Result as AnyhowResult};
use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::TryStreamExt;
use hyper::StatusCode;
use mongodb::{Client as MongoClient, Collection, Cursor, Database};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use harbcore::services::{clone_repo, clone_path, remove_clone, syft};

use platform::hyper::{ContentType, Error as HyperError, get};
use platform::mongodb::{Context as MongoContext, mongo_doc, MongoDocument};
use platform::mongodb::service::Service;

use harborclient::client::{Client as V1HarborClient, SBOMUploadResponse};

use crate::commands::{get_env_var, Provider};

pub const DB_IDENTIFIER: &str = "harbor";
pub const KEY_NAME: &str = "id";
pub const COLLECTION: &str = "pilot";

pub const TEAM_ID_KEY: &str = "team_id";
pub const CF_DOMAIN_KEY: &str = "CF_DOMAIN";
pub const PROJECT_ID_KEY: &str = "project_id";
pub const CODEBASE_ID_KEY: &str = "codebase_id";
pub const GH_FT_KEY: &str = "GH_FETCH_TOKEN";
pub const V1_TEAM_ID_KEY: &str = "V1_CMS_TEAM_ID";
pub const V1_TEAM_TOKEN_KEY: &str = "V1_CMS_TEAM_TOKEN";
pub const V1_HARBOR_USERNAME_KEY: &str = "V1_HARBOR_USERNAME";
pub const V1_HARBOR_PASSWORD_KEY: &str = "V1_HARBOR_PASSWORD";

/// Configuration from the environment for V1 Harbor
///
struct HarborConfig {
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

const GH_URL: &str = "https://api.github.com";

pub struct LocalContext;

impl LocalContext {
    pub fn connection_string() -> String {
        String::from("mongodb://localhost:27017")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GitHubCrawlerMongoDocument {
    /// Unique id of the [GitHubCrawlerMongoDocument].
    pub id: String,
    /// Url of the GitHub Repository
    pub repo_url: String,
    /// Last commit hash of the repository
    pub last_hash: String,
    /// Harbor v1 team id
    pub team_id: String,
    /// Harbor v1 project id
    pub project_id: String,
    /// Harbor v1 codebase id
    pub codebase_id: String,
}
mongo_doc!(GitHubCrawlerMongoDocument);

/// Commit represents the returned Json from a commits
/// request from the GitHub API.
///
#[derive(Debug, Serialize, Deserialize)]
struct Commit {
    sha: Option<String>
}

/// Org is used to extract the number of Public Repos
/// in a given GitHub Organization.
///
#[derive(Debug, Serialize, Deserialize)]
struct Org {
    /// The number of Public Repos in
    /// this organization
    public_repos: Option<u32>
}

/// Repo is used to extract several values from a Request for
/// the Repositories in a given GitHub Organization
///
#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    full_name: Option<String>,
    ssh_url: Option<String>,
    default_branch: Option<String>,
    language: Option<String>,
    archived: Option<bool>,
    disabled: Option<bool>,

    #[serde(default = "default_bool")]
    empty: bool,

    #[serde(default = "empty_string")]
    last_hash: String,
}

/// Little function to define default booleans
/// for struct values that are to be used to collect Json
///
fn default_bool() -> bool {
    false
}

/// Little function to define default Strings
/// for struct values that are to be used to collect Json
///
fn empty_string() -> String {
    "".to_string()
}

/// Repo impl
///
impl Repo {

    /// This method allows us to add the last hash to a
    /// Repo if it is newer that what is already in Mongo
    ///
    fn add_last_hash(&mut self, last_hash: String) {
        self.last_hash = last_hash.to_string();
    }

    /// Method to mark the Repository as empty
    ///
    fn mark_repo_empty(&mut self) {
        self.empty = true;
    }
}

/// Should skip determines if the repository is disabled or archived.
/// and if so, skips processing them.
///
fn should_skip(repo: &Repo, repo_name: String, url: String) -> bool {

    let mut skip: bool = false;

    match &repo.archived {
        Some(archived) => {
            if *archived {
                println!("{} at {} is archived, skipping", repo_name, url);
                skip = true;
            }
        },
        None => {
            println!("No value to determine if the repo is archived");
        }
    }

    match &repo.disabled {
        Some(disabled) => {
            if *disabled {
                println!("{} at {} is disabled, skipping", repo_name, url);
                skip = true;
            }
        },
        None => {
            println!("No value to determine if the repo is disabled, processing");
        }
    }

    if repo.empty {
        skip = true
    }

    return skip;
}

async fn get_num_pub_repos(org: String) -> AnyhowResult<Option<u32>> {

    let token = match get_env_var(GH_FT_KEY) {
        Some(value) => value,
        None => panic!("GitHub token not in environment. Variable name: GH_FETCH_TOKEN")
    };

    let token: String = format!("Bearer {}", &token);
    let org_url: String = format!("{GH_URL}/orgs/{org}");

    let response: Result<Option<Org>, HyperError> = get(
        org_url.as_str(),
        ContentType::Json,
        token.as_str(),
        None::<String>,
    ).await;

    match response {
        Ok(option) => match option {
            Some(value) => return Ok(value.public_repos),
            None => panic!("Nothing in here!"),
        },
        Err(err) => panic!("Error in the response: {}", err),
    }
}

async fn get_pages(org: &String) -> Vec<u32> {

    let num_repos = match get_num_pub_repos(org.to_string()).await {
        Ok(option) => match option {
            Some(num) => num,
            None => panic!("No Repos in the cmsgov ORG!!!")
        },
        Err(err) => panic!("Error Attempting to get num Repos: {}", err)
    };

    println!("Number of Repositories in {org}: {num_repos}");

    let num_calls = ((num_repos/100) as i8) + 1;
    let num_last_call = num_repos % 100;

    let mut vector = vec![100; usize::try_from(num_calls).unwrap()];

    // This is crazy that it works.
    *vector.last_mut().unwrap() = num_last_call;

    vector
}

async fn get_page_of_repos(org: &String, page: &u32, token: &String) -> Vec<Repo> {

    let github_org_url = format!("{GH_URL}/orgs/{org}/repos?type=sources&per_page={page}");

    let response: Result<Option<Vec<Repo>>, HyperError> = get(
        github_org_url.as_str(),
        ContentType::Json,
        token.as_str(),
        None::<String>,
    ).await;

    match response {
        Ok(option) => match option {
            Some(value) => value,
            None => panic!("Nothing in here!"),
        },
        Err(err) => panic!("Error in the response(0): {}", err),
    }
}

pub struct GitHubProvider {}

impl GitHubProvider {

    async fn get_repos(org: String) -> AnyhowResult<Vec<Repo>> {

        // let mut pages = get_pages(&org).await;
        // TODO For development only, correct code above
        let mut pages = vec![5];

        let gh_fetch_token = match get_env_var("GH_FETCH_TOKEN") {
            Some(value) => value,
            None => panic!("Missing GitHub Token. export GH_FETCH_TOKEN=<token>")
        };

        let token: String = String::from("Bearer ") + &gh_fetch_token;

        let mut repo_vec: Vec<Repo> = Vec::new();

        for page in pages.iter_mut() {

            let mut gh_org_rsp = get_page_of_repos(&org, page, &token).await;

            for repo in gh_org_rsp.iter_mut() {

                let repo_name = repo.full_name.as_ref().unwrap();
                let default_branch = repo.default_branch.as_ref().unwrap();

                let github_last_commit_url = format!(
                    "{GH_URL}/repos/{repo_name}/commits/{default_branch}",
                );

                println!("Making call to {}", github_last_commit_url);

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

async fn get_mongo_db() -> Result<Database, GhProviderError> {

    let ctx = MongoContext {
        connection_uri: LocalContext::connection_string(),
        db_name: DB_IDENTIFIER.to_string(),
        key_name: KEY_NAME.to_string(),
    };

    let result = MongoClient::with_uri_str(ctx.connection_uri.clone()).await;
    let client = match result {
        Ok(client) => client,
        Err(err) => return Err(
            GhProviderError::MongoDb(
                format!("Unable to get the Mongo Client: {}", err)
            )
        )
    };

    // Get a handle to the database.
    Ok(client.database(&ctx.db_name))
}

async fn get_entities(harbor_config: HarborConfig) -> bool {

    let result = V1HarborClient::new(
        harbor_config.cf_domain,
        harbor_config.cognito_username,
        harbor_config.cognito_password
    ).await;

    let client = match result {
      Ok(client) => client,
      Err(err) => panic!("Unable to get the client: {}", err)
    };

    match client.get_team(harbor_config.cms_team_id).await {
        Ok(_team) => true,
        Err(_err) => false,
    }
}

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
            return Err(GhProviderError::EntityCreation(err_msg))
        }
    };

    // Each project will have only one codebase in this implementation
    let codebase = project.codebases.into_iter().nth(0);

    let mut test_map = HashMap::new();

    test_map.insert(
        String::from(TEAM_ID_KEY),
        harbor_config.cms_team_id.to_string(),
    );

    test_map.insert(
        String::from(PROJECT_ID_KEY),
        project.id
    );

    test_map.insert(
        String::from(CODEBASE_ID_KEY),
        codebase.unwrap().id
    );

    Ok(test_map)
}

fn authize(ssh_url: &String) -> Result<String, Error> {
    let token = match get_env_var(GH_FT_KEY) {
        Some(value) => value,
        None => panic!("GitHub token not in environment. Variable name: GH_FETCH_TOKEN")
    };

    let ssh_url_no_colon = ssh_url.replace(":", "/");
    let parts: Vec<&str> = ssh_url_no_colon.as_str().split("@").collect();
    Ok(format!("https://qtpeters:{}@{}", token, parts[1]))
}

async fn send_to_pilot(
    document: &GitHubCrawlerMongoDocument,
    harbor_config: &HarborConfig,
) -> Result<SBOMUploadResponse, GhProviderError> {

    /// Clones a repo, generates an SBOM, and then uploads to the Enrichment Engine.

    let clone_path = clone_path(&document.repo_url);
    let authed_url = match authize(&document.repo_url) {
        Ok(authed_url) => authed_url,
        Err(err) => panic!("Unable to fix the URL: {}", err)
    };

    match clone_repo(&clone_path, authed_url.as_str()) {
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

fn get_harbor_config() -> Result<HarborConfig, GhProviderError> {

    let cms_team_id = match get_env_var(V1_TEAM_ID_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team Id of V1 Team")
            )
        )
    };

    let cms_team_token = match get_env_var(V1_TEAM_TOKEN_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team token of V1 Team")
            )
        )
    };

    let cf_domain = match get_env_var(CF_DOMAIN_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_username = match get_env_var(V1_HARBOR_USERNAME_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_password = match get_env_var(V1_HARBOR_PASSWORD_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Password")
            )
        )
    };

    Ok(
        HarborConfig {
            cms_team_id,
            cms_team_token,
            cf_domain,
            cognito_username,
            cognito_password,
        }
    )
}

async fn update_last_hash_in_mongo(
    document: GitHubCrawlerMongoDocument,
    collection: Collection<GitHubCrawlerMongoDocument>,
    last_hash: String,
) {

    println!("Updating the last hash in Mongo!");

    // Create a filter to find the document we are looking for
    // by id.  Probably supposed to be using _id, but whatever for now.
    let filter = doc! {
        "id": document.id.to_string()
    };

    // This document is used by MongoDB to actually
    // set the new sha hash value on the record
    let update_document = doc!{
        "$set": {
            "last_hash": last_hash.clone()
        }
    };

    // Update the last hash in MongoDB
    match collection.update_one(filter, update_document, None).await {
        Ok(result) => result,
        Err(err) => panic!("Error attempting to insert a document: {}", err)
    };

    println!(
        "Updated EXISTING Document in MongoDB: {:#?}, with hash: {}",
        &document.id, last_hash
    );
}

async fn process_repo(repo: &Repo, harbor_config: &HarborConfig) {

    let url: &String = match &repo.ssh_url {
        Some(url) => url,
        None => panic!("No URL for Repository"),
    };

    let repo_name: &String = match &repo.full_name {
        Some(name) => name,
        None => panic!("No Full Name for Repository"),
    };

    let last_hash: &String = &repo.last_hash;

    println!("Will be processing {}@{}", repo_name, url);

    let db = match get_mongo_db().await {
        Ok(db) => db,
        Err(err) => panic!("Problem getting DB: {}", err)
    };

    let collection = db.collection::<GitHubCrawlerMongoDocument>(COLLECTION);
    let filter = doc! { "repo_url":  url.as_str() };
    let mut doc_option: Option<GitHubCrawlerMongoDocument> = match collection.find_one(filter, None).await {
        Ok(cursor) => cursor,
        Err(cursor_err) => panic!("Cursor - Error: {}", cursor_err)
    };

    match doc_option {
        Some(document) => {

            println!("Found document in mongo, comparing hashes");

            // This arm is executed when something is in the database
            // with the specified repo_url.

            if last_hash.to_string() != document.last_hash {

                println!("Hashes are not equal, sending to pilot");

                // Use the document to construct a request to Pilot
                match send_to_pilot(&document, &harbor_config).await {
                    Ok(upload_resp) => {

                        // Upload is OK, update Mongo

                        println!("One SBOM Down! {:#?}", upload_resp);
                        update_last_hash_in_mongo(document, collection, last_hash.clone());
                    },
                    Err(err) => println!("Error Uploading SBOM!! {}", err)
                }
            } else {
                println!("Hashes are equal, skipping pilot");
            }
        },
        None => { // Nothing is in Mongo

            println!("No Document found document in mongo. Creating entities in harbor");

            // This arm executes when nothing is found in the database associated
            // to the given repo_url.  This means we need to create the project and codebase
            // in Harbor before we can send SBOMs to that target.

            let language = match &repo.language {
                Some(language) => language.to_string(),
                None => String::from("None"),
            };

            let result = create_harbor_entities(
                url.clone(),
                harbor_config,
                language,
            ).await;

            let entities: HashMap<String, String> = match result {
                Ok(entities) => entities,
                Err(err) => panic!("Unable to create Harbor entities, {}", err),
            };

            let document = GitHubCrawlerMongoDocument {
                id: Uuid::new_v4().to_string(),
                repo_url: url.to_string(),
                last_hash: last_hash.to_string(),
                team_id: entities.get(TEAM_ID_KEY).unwrap().to_string(),
                project_id: entities.get(PROJECT_ID_KEY).unwrap().to_string(),
                codebase_id: entities.get(CODEBASE_ID_KEY).unwrap().to_string(),
            };

            match collection.insert_one(document.clone(), None).await {
                Ok(result) => result,
                Err(err) => panic!("Error attempting to insert a document: {}", err)
            };

            println!("Added NEW Document to MongoDB: {:#?}", document);

            // Use the document to construct a request to Pilot
            match send_to_pilot(&document, &harbor_config).await {
                Ok(upload_resp) => println!("One SBOM Down! {:#?}", upload_resp),
                Err(err) => println!("Error Uploading SBOM!! {}", err)
            }
        }
    };
}


#[async_trait]
impl Provider for GitHubProvider {

    async fn scan(&self) {

        let harbor_config = get_harbor_config().unwrap();

        println!("Scanning GitHub...");
        let org_name: String = String::from("cmsgov");
        let repos_result = GitHubProvider::get_repos(org_name).await;
        let repos: Vec<Repo> = match repos_result {
            Ok(value) => value,
            Err(err) => panic!("Panic trying to extract value from Result: {}", err),
        };

        for repo in repos.iter() {

            let name = repo.full_name.clone().unwrap();
            let url = repo.ssh_url.clone().unwrap();

            if !should_skip(repo, name, url) {
                process_repo(repo, &harbor_config).await;
            }
        }
    }
}

/// Represents all handled Errors for the GitHub Crawler.
///
#[derive(Error, Debug)]
enum GhProviderError {

    #[error("error getting database: {0}")]
    MongoDb(String),

    #[error("error getting collection: {0}")]
    MongoCollection(String),

    #[error("error creating Harbor v1 entities: {0}")]
    EntityCreation(String),

    #[error("error creating Harbor v1 entities: {0}")]
    Configuration(String),

    #[error("error running pilot: {0}")]
    Pilot(String),
}
