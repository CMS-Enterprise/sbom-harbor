use std::convert::TryFrom;
use std::env;
use std::ops::Deref;
use std::pin::Pin;

use anyhow::{anyhow, Result as AnyhowResult};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Client as MongoClient;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use futures::TryStreamExt;

use platform::mongodb::{Context as MongoContext, mongo_doc, MongoDocument};
use platform::mongodb::service::Service;

use crate::commands::{get_env_var, Provider};
use crate::http::{ContentType, get};

pub const DB_IDENTIFIER: &str = "harbor";
pub const KEY_NAME: &str = "id";
pub const COLLECTION: &str = "pilot";

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

    #[serde(default = "empty_string")]
    last_hash: String,
}

impl Repo {
    fn add_last_hash(&mut self, last_hash: String) {
        self.last_hash = last_hash.to_string();
    }
}

fn empty_string() -> String {
    "".to_string()
}

/// Extracts teh GitHub token from the environment
///
fn get_gh_token() -> String {
    return match env::var("GH_FETCH_TOKEN") {
        Ok(v) => v,
        Err(e) => panic!("$GH_FETCH_TOKEN is not set ({})", e),
    };
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

    return skip;
}

async fn get_num_pub_repos(org: String) -> AnyhowResult<Option<u32>> {
    let token: String = String::from("Bearer ") + &get_env_var("GH_FETCH_TOKEN");
    let org_url: String = format!("https://api.github.com/orgs/{org}");

    let response: AnyhowResult<Option<Org>> = get(
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

    let response: AnyhowResult<Option<Vec<Repo>>> = get(
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

                let response: AnyhowResult<Option<Commit>> = get(
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
                        // This error response means that there was some issue making the call
                        // If it is a "409 Conflict", then the repo was empty
                        panic!("Error in the response(Empty Repo): {}", err);
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

            let url = match &repo.ssh_url {
                Some(url) => url,
                None => panic!("No URL for Repository"),
            };

            let repo_name = match &repo.full_name {
                Some(name) => name,
                None => panic!("No Full Name for Repository"),
            };

            if !should_skip(repo, url, repo_name) {

                println!("Will be processing {}@{}", repo_name, url);

                let ctx = MongoContext {
                    connection_uri: LocalContext::connection_string(),
                    db_name: DB_IDENTIFIER.to_string(),
                    key_name: KEY_NAME.to_string(),
                };

                let result = MongoClient::with_uri_str(ctx.connection_uri.clone()).await;
                let client = match result {
                    Ok(client) => client,
                    Err(err) => panic!("Unable to get the Mongo Client: {}", err),
                };

                // Get a handle to the database.
                let db = client.database(&ctx.db_name);

                // Get the collection
                let collection = db.collection::<GitHubCrawlerMongoDocument>(COLLECTION);

                // Query the books in the collection with a filter and an option.
                let filter = doc! { "repo_url": url.to_string() };
                let mut cursor = match collection.find(filter, None).await {
                    Ok(cursor) => cursor,
                    Err(cursor_err) => panic!("Cursor - Error: {}", cursor_err)
                };

                let mongo_result = match cursor.next().await {
                    Some(mr) => match mr {
                        Ok(mr) => mr,
                        Err(err) => panic!("Err matching on mr: {}", err)
                    },
                    None => panic!("No value here BUDDY")
                };

                println!("Cursor from Mongo: {:#?}", mongo_result);

                // // Iterate over the results of the cursor.
                // let result = match cursor.next().await {
                //     Ok(mdb_result) => mdb_result,
                //     Err(mdb_err) => panic!("MDB - Error getting value: {}", mdb_err)
                // };
                //
                // println!("Result from Mongo: {:#?}", result);

                let test_doc = GitHubCrawlerMongoDocument {
                    id: String::from("test-document"),
                    repo_url: url.to_string(),
                    last_hash: String::from(""),
                };

                match collection.insert_one(test_doc, None).await {
                    Ok(result) => result,
                    Err(err) => panic!("Error attempting to insert a document: {}", err)
                };
            }
        }
    }
}

