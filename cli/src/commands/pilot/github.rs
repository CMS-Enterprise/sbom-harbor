use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::ops::Deref;
use std::pin::Pin;

use anyhow::{anyhow, Result as AnyhowResult};
use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::TryStreamExt;
use hyper::StatusCode;
use mongodb::{Client as MongoClient, Cursor, Database};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use platform::hyper::{ContentType, Error as HyperError, get};
use platform::mongodb::{Context as MongoContext, mongo_doc, MongoDocument};
use platform::mongodb::service::Service;
use uuid::Uuid;
use crate::commands::{get_env_var, Provider};

pub const DB_IDENTIFIER: &str = "harbor";
pub const KEY_NAME: &str = "id";
pub const COLLECTION: &str = "pilot";

pub const TEAM_ID_KEY: &str = "team_id";
pub const PROJECT_ID_KEY: &str = "project_id";
pub const CODEBASE_ID_KEY: &str = "codebase_id";

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
fn should_skip(repo: &Repo, repo_name: &String, url: &String) -> bool {

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

    let token = match get_env_var("GH_FETCH_TOKEN") {
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
        Err(err) => panic!("Error in the response(1): {}", err),
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

        let mut pages = get_pages(&org).await;
        let token: String = String::from("Bearer ") + &get_gh_token();
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

async fn get_mongo_db() -> Result<Database, GhCrawlerError> {

    let ctx = MongoContext {
        connection_uri: LocalContext::connection_string(),
        db_name: DB_IDENTIFIER.to_string(),
        key_name: KEY_NAME.to_string(),
    };

    let result = MongoClient::with_uri_str(ctx.connection_uri.clone()).await;
    let client = match result {
        Ok(client) => client,
        Err(err) => return Err(
            GhCrawlerError::MongoDb(
                format!("Unable to get the Mongo Client: {}", err)
            )
        )
    };

    // Get a handle to the database.
    Ok(client.database(&ctx.db_name))
}

async fn create_harbor_entities() -> Result<HashMap<String, String>, GhCrawlerError> {

    // TODO - STUB!! Please Implement...
    // TODO Error for this method: GhCrawlerError::EntityCreation

    let cms_team_id = match get_env_var("V1_CMS_TEAM_ID") {
        Some(value) => value,
        None => panic!("Missing Team Id of V1 Team")
    };

    let cms_team_token = match get_env_var("V1_CMS_TEAM_TOKEN") {
        Some(value) => value,
        None => panic!("Missing Team token of V1 Team")
    };

    let mut test_map = HashMap::new();

    test_map.insert(
        String::from(TEAM_ID_KEY),
        cms_team_id,
    );

    test_map.insert(
        String::from(PROJECT_ID_KEY),
        Uuid::new_v4().to_string()
    );

    test_map.insert(
        String::from(CODEBASE_ID_KEY),
        Uuid::new_v4().to_string()
    );

    Ok(test_map)
}

async fn send_to_pilot(document: &GitHubCrawlerMongoDocument) {
    // TODO - STUB!! -> NOOP
    println!("Using this document to construct a request to Pilot: {:#?}", document)
}

async fn process_repo(url: &String, repo_name: &String, last_hash: &String) {

    println!("Will be processing {}@{}", repo_name, url);

    let db = match get_mongo_db().await {
        Ok(db) => db,
        Err(err) => panic!("Problem getting DB: {}", err)
    };

    let collection = db.collection::<GitHubCrawlerMongoDocument>(COLLECTION);

    let filter = doc! { "repo_url": url.to_string() };
    let mut cursor = match collection.find(filter, None).await {
        Ok(cursor) => cursor,
        Err(cursor_err) => panic!("Cursor - Error: {}", cursor_err)
    };

    match cursor.next().await {
        Some(mongo_result) => match mongo_result {
            Ok(document) => {

                if last_hash.to_string() != document.last_hash {

                    // Use the document to construct a request to Pilot
                    send_to_pilot(&document);

                    // Create a filter to find the document we are looking for
                    // by id.  Probably supposed to be using _id, but whatever for now.
                    let filter = doc! {
                        "id": document.id.to_string()
                    };

                    // This document is used by MongoDB to actually
                    // set the new sha hash value on the record
                    let update_document = doc!{
                        "$set": {
                            "last_hash": last_hash.to_string()
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
            }
            Err(err) => panic!("Mongo Result Error. Result exists, but data is missing: {}", err)
        },
        None => {

            let entities: HashMap<String, String> = match create_harbor_entities().await {
                Ok(entities) => entities,
                Err(err) => panic!("Unable to create Harbor entities: {}", err),
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
        }
    };
}


#[async_trait]
impl Provider for GitHubProvider {

    async fn scan(&self) {

        println!("Scanning GitHub...");
        let org_name: String = String::from("cmsgov");
        let repos_result = GitHubProvider::get_repos(org_name).await;
        let repos: Vec<Repo> = match repos_result {
            Ok(value) => value,
            Err(err) => panic!("Panic trying to extract value from Result: {}", err),
        };

        for repo in repos.iter() {

            let url: &String = match &repo.ssh_url {
                Some(url) => url,
                None => panic!("No URL for Repository"),
            };

            let repo_name: &String = match &repo.full_name {
                Some(name) => name,
                None => panic!("No Full Name for Repository"),
            };

            if !should_skip(repo, url, repo_name) {
                process_repo(url, repo_name, &repo.last_hash).await;
            }
        }
    }
}

/// Represents all handled Errors for the GitHub Crawler.
///
#[derive(Error, Debug)]
enum GhCrawlerError {

    #[error("error getting database: {0}")]
    MongoDb(String),

    #[error("error getting collection: {0}")]
    MongoCollection(String),

    #[error("error creating Harbor v1 entities: {0}")]
    EntityCreation(String),
}