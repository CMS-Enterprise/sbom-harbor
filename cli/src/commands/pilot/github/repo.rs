use std::convert::TryFrom;

use anyhow::{anyhow, Result as AnyhowResult};
use serde;
use serde::{Deserialize, Serialize};
use platform::hyper::{ContentType, Error as HyperError, get};

use crate::commands::get_env_var;
use crate::commands::pilot::github::{Counter, GH_FT_KEY, GhProviderError};

const GH_URL: &str = "https://api.github.com";

pub fn get_last_commit_url(repo: &mut Repo) -> String {
    let repo_name = repo.full_name.as_ref().unwrap();
    let default_branch = repo.default_branch.as_ref().unwrap();
    format!("{GH_URL}/repos/{repo_name}/commits/{default_branch}")
}

/// Commit represents the returned Json from a commits
/// request from the GitHub API.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub(crate) sha: Option<String>
}

/// Org is used to extract the number of Public Repos
/// in a given GitHub Organization.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Org {
    /// The number of Public Repos in
    /// this organization
    public_repos: Option<u32>
}

/// Repo is used to extract several values from a Request for
/// the Repositories in a given GitHub Organization
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub(crate) full_name: Option<String>,
    pub(crate) ssh_url: Option<String>,
    pub(crate) default_branch: Option<String>,
    pub(crate) language: Option<String>,
    archived: Option<bool>,
    disabled: Option<bool>,

    #[serde(default = "default_bool")]
    empty: bool,

    #[serde(default = "empty_string")]
    pub(crate) last_hash: String,
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
    pub(crate) fn add_last_hash(&mut self, last_hash: String) {
        self.last_hash = last_hash.to_string();
    }

    /// Method to mark the Repository as empty
    ///
    pub(crate) fn mark_repo_empty(&mut self) {
        self.empty = true;
    }
}

/// Should skip determines if the repository is disabled or archived.
/// and if so, skips processing them.
///
pub fn should_skip(repo: &Repo, repo_name: String, url: String, counter: &mut Counter) -> bool {

    let mut skip: bool = false;

    match &repo.archived {
        Some(archived) => {
            if *archived {
                println!("{} at {} is archived, skipping", repo_name, url);
                counter.archived += 1;
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
                counter.disabled += 1;
                skip = true;
            }
        },
        None => {
            println!("No value to determine if the repo is disabled, processing");
        }
    }

    if repo.empty {
        skip = true;
        counter.empty += 1;
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

pub async fn get_pages(org: &String) -> Vec<u32> {

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

pub async fn get_page_of_repos(org: &String, page: usize, per_page: &u32, token: &String) -> Vec<Repo> {

    let github_org_url = format!("{GH_URL}/orgs/{org}/repos?type=sources&page={page}&per_page={per_page}");

    println!("Calling({})", github_org_url);

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
        Err(err) => panic!("Error in the response: {}", err),
    }
}

pub fn authize(ssh_url: &String) -> Result<String, GhProviderError> {
    let token = match get_env_var(GH_FT_KEY) {
        Some(value) => value,
        None => panic!("GitHub token not in environment. Variable name: GH_FETCH_TOKEN")
    };

    let ssh_url_no_colon = ssh_url.replace(":", "/");
    let parts: Vec<&str> = ssh_url_no_colon.as_str().split("@").collect();

    // TODO This should not be using qtpeters.  Need to fix this
    Ok(format!("https://qtpeters:{}@{}", token, parts[1]))
}