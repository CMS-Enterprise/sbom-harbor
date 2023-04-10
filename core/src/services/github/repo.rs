use std::convert::TryFrom;

use serde;
use serde::{Deserialize, Serialize};
use platform::hyper::{ContentType, Error as HyperError, get};

use crate::commands::get_env_var;
use crate::commands::github::{Counter, GH_FT_KEY, GhProviderError};
use crate::config::GH_FT_KEY;
use crate::services::github::{Counter, GhProviderError};

const GH_URL: &str = "https://api.github.com";

/// Creates the URL one must use in an http request for
/// acquiring the latest commit hash from a given branch
///
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

    /// The name of the repo.  Getting the Full name
    /// is nice because it has teh "user" in it: <USER>>/<REPO>.git
    /// Ex: CMSgov/design-system.git
    pub(crate) full_name: Option<String>,

    /// Url of the repository.
    pub(crate) html_url: Option<String>,

    /// The default branch of the repo, usually master or main
    pub(crate) default_branch: Option<String>,

    /// The language of the code in the repo.
    pub(crate) language: Option<String>,

    /// Is this repo archived?
    archived: Option<bool>,

    /// Is this repo disabled?
    disabled: Option<bool>,

    /// Is this repo empty?
    #[serde(default = "default_bool")]
    empty: bool,

    /// This is the most recent commit
    /// hash of the default branch
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
pub fn should_skip(
    repo: &Repo,
    repo_name: String,
    url: String,
    counter: &mut Counter
) -> bool {

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

/// Function to get the number of public
/// repos in the associated organization
///
async fn get_num_pub_repos(org: String) -> Result<Option<u32>, GhProviderError> {

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

    return match response {
        Ok(option) => match option {
            Some(value) => Ok(value.public_repos),
            None => Err(
                GhProviderError::GitHubRequest(
                    format!("Get request from GitHub had an empty response")
                )
            ),
        },
        Err(err) => Err(
            GhProviderError::GitHubRequest(
                format!("Error in the response: {}", err)
            )
        ),
    }
}

/// Function to get the number of repos per page
///
pub async fn get_pages(org: &String) -> Result<Vec<u32>, GhProviderError> {

    let num_repos = match get_num_pub_repos(org.to_string()).await {
        Ok(option) => match option {
            Some(num) => num,
            None => return Err(
                GhProviderError::GitHubRequest(
                    format!("There are no repositories in {}, something is wrong", org)
                )
            ),
        },
        Err(err) => return Err(
            GhProviderError::GitHubRequest(
                format!("Error Attempting to get num Repos: {}", err)
            )
        ),
    };

    println!("Number of Repositories in {org}: {num_repos}");

    let num_calls = ((num_repos/100) as i8) + 1;
    let num_last_call = num_repos % 100;

    let mut vector = vec![100; usize::try_from(num_calls).unwrap()];

    // This is crazy that it works.
    *vector.last_mut().unwrap() = num_last_call;

    Ok(vector)
}

/// Function to get the data for a page of repos
///
pub async fn get_page_of_repos(
    org: &String,
    page: usize,
    per_page: &u32,
    token: &String
) -> Result<Vec<Repo>, GhProviderError> {

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
            Some(value) => Ok(value),
            None => return Err(
                GhProviderError::GitHubRequest(
                    format!("Get request from GitHub had an empty response")
                )
            ),
        },
        Err(err) => return Err(
            GhProviderError::GitHubRequest(
                format!("Error in the response: {}", err)
            )
        ),
    }
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

    let mut test_counter = Counter {
        archived: 0,
        disabled: 0,
        empty: 0,
        processed: 0,
        hash_matched: 0,
        upload_errors: 0,
    };

    if !should_skip(&test_repo, test_name, test_url, &mut test_counter) {
        panic!("should_skip should be true for an archived repo");
    } else {
        if test_counter.archived != 1 {
            panic!("Counter did not count the repo as archived");
        }
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

    let mut test_counter = Counter {
        archived: 0,
        disabled: 0,
        empty: 0,
        processed: 0,
        hash_matched: 0,
        upload_errors: 0,
    };

    if !should_skip(&test_repo, test_name, test_url, &mut test_counter) {
        panic!("should_skip should be true for an disabled repo");
    } else {
        if test_counter.disabled != 1 {
            panic!("Counter did not count the repo as disabled");
        }
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

    let mut test_counter = Counter {
        archived: 0,
        disabled: 0,
        empty: 0,
        processed: 0,
        hash_matched: 0,
        upload_errors: 0,
    };

    if !should_skip(&test_repo, test_name, test_url, &mut test_counter) {
        panic!("should_skip should be true for an empty repo");
    } else {
        if test_counter.empty != 1 {
            panic!("Counter did not count the repo as empty");
        }
    }
}