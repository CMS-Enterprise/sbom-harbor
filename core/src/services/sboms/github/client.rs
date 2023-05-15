use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use chrono::Utc;
use git2::Repository;
use serde;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use platform::config::from_env;
use platform::hyper::{ContentType, Error as HyperError, get};
use crate::config::get_gh_token;

use crate::services::providers::github::counter::Counter;
use crate::services::providers::github::error::Error;
use crate::services::sboms::github::counter::Counter;
use crate::services::sboms::github::error::Error;

const GH_URL: &str = "https://api.github.com";

// TODO: Make an enum with all possible formats, then make
// TODO: this a config option with default.
const CYCLONEDX_JSON_FORMAT: &str = "cyclonedx-json";

pub struct Client {}

impl Client {

    /* Private */

    /// Creates the URL one must use in an http request for
    /// acquiring the latest commit hash from a given branch
    ///
    fn get_last_commit_url(&self, repo: &mut Repo) -> String {
        let repo_name = repo.full_name.as_ref().unwrap();
        let default_branch = repo.default_branch.as_ref().unwrap();
        format!("{GH_URL}/repos/{repo_name}/commits/{default_branch}")
    }

    /// Function to get the number of public
    /// repos in the associated organization
    ///
    async fn get_num_pub_repos(&self, org: String) -> Result<Option<u32>, Error> {

        let token = match get_gh_token() {
            Ok(value) => value,
            Err(err) => return Err(
                Error::Configuration(err)
            )
        };

        let bearer_token: String = format!("Bearer {}", &token);
        let org_url: String = format!("{GH_URL}/orgs/{org}");

        let response: Result<Option<Org>, HyperError> = get(
            org_url.as_str(),
            ContentType::Json,
            bearer_token.as_str(),
            None::<String>,
        ).await;

        return match response {
            Ok(option) => match option {
                Some(value) => Ok(value.public_repos),
                None => Err(
                    Error::GitHubEmptyResponse(
                        format!("Get request from GitHub had an empty response")
                    )
                ),
            },
            Err(err) => Err(
                Error::GitHubResponse(err)
            ),
        }
    }

    /* Public */

    /// Get the last commit for a given Repo
    ///
    pub async fn get_last_commit(&self, token: &String, repo: &mut Repo)
        -> Result<Option<String>, Error> {

        let github_last_commit_url = self.get_last_commit_url(repo);

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
                if let HyperError::Remote(status, msg) = err {
                    return Err(
                        Error::LastCommitHashError(status, msg)
                    )
                }

                Commit {
                    sha: None,
                }
            },
        };

        match gh_commits_rsp.sha {
            Some(val) => Ok(Some(val)),
            None => Ok(None),
        }
    }

    /// Conventional Constructor.
    ///
    pub fn new() -> Self {
        Client {}
    }

    /// Should skip determines if the repository is disabled or archived.
    /// and if so, skips processing them.
    ///
    pub fn should_skip(
        &self,
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

    /// Function to get the number of repos per page
    ///
    pub async fn get_pages(&self, org: &String) -> Result<Vec<u32>, Error> {

        let num_repos = self.get_num_pub_repos(org.to_string()).await?.unwrap_or(0);

        println!("Number of Repositories in {org}: {:#?}", num_repos);

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
        &self,
        org: &String,
        page: usize,
        per_page: &u32,
        token: &String,
    ) -> Result<Vec<Repo>, Error> {

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
                    Error::GitHubEmptyResponse(
                        format!("Get request from GitHub had an empty response")
                    )
                ),
            },
            Err(err) => return Err(
                Error::GitHubResponse(err)
            ),
        }
    }

    /// Generates a unique clone path for a repository.
    ///
    pub fn clone_path(&self, url: &str) -> String {
        // add a unique element to the path to prevent collisions.
        let timestamp = Utc::now().to_rfc3339();

        let repo_name = url
            .split('/')
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .replace(".git", "");

        format!("/tmp/{}/{}", timestamp, repo_name)
    }

    /// Clones a git repository to the specified clone path.
    ///
    pub fn clone_repo(&self, clone_path: &str, url: &str) -> Result<(), Error> {
        info!("Cloning repo: {}", url);

        match Repository::clone(url, clone_path) {
            Err(err) => {
                panic!("error cloning repository from {}: {}", url, err);
            }
            _ => info!("Successfully cloned repo"),
        };

        Ok(())
    }

    /// Invokes the syft CLI against the cloned repository to generate an SBOM.
    ///
    pub fn syft(&self, source_path: &str) -> Result<HashMap<String, Value>, Error> {
        let output = match Command::new("syft")
            .arg("--output")
            .arg(CYCLONEDX_JSON_FORMAT)
            .arg(source_path)
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                panic!("error executing syft cli: {}", err);
            }
        };

        // Handle error generated by syft.
        if !&output.status.success() {
            match String::from_utf8(output.stderr) {
                Ok(stderr) => {
                    panic!("error generating SBOM: {}", &stderr);
                }
                Err(err) => {
                    panic!("error formatting syft stderr: {}", &err);
                }
            };
        }

        if output.stdout.is_empty() {
            panic!("syft generated empty SBOM");
        };

        match serde_json::from_slice::<HashMap<String, Value>>(output.stdout.as_slice()) {
            Ok(result) => Ok(result),
            Err(err) => {
                panic!("error serializing SBOM to hash map: {}", err);
            }
        }
    }

    /// Removes a cloned repository from the filesystem.
    pub fn remove_clone(&self, clone_path: &str) -> std::io::Result<()> {
        if Path::new(&clone_path).is_dir() {
            return std::fs::remove_dir_all(clone_path);
        }

        Ok(())
    }
}

/// An HTTP request that contains the necessary configuration and authorization
/// to auto-generate and upload an SBOM.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {

    /// The team id for the request.
    #[serde(rename = "teamId")]
    pub team_id: String,

    /// The project id for the request.
    #[serde(rename = "projectId")]
    pub project_id: String,

    /// The codebase id for the request.
    #[serde(rename = "codebaseId")]
    pub codebase_id: String,

    /// The CloudFront Domain for the Harbor instance.
    #[serde(rename = "cloudFrontDomain")]
    pub cloud_front_domain: String,

    /// The API Gateway URL for the Harbor instance.
    #[serde(rename = "apiGatewayUrl")]
    pub api_gateway_url: Option<String>,

    /// A valid SBOM upload token for Harbor.
    #[serde(rename = "harborToken")]
    pub token: String,

    // TODO: Rename to clone_url.
    /// The HTTPS git URL for the repository.
    #[serde(rename = "gitHubUrl")]
    pub github_url: String,
}

impl Request {

    /// Validates a Request
    ///
    pub fn validate(&self) -> Result<(), Error> {
        let mut errors: String = String::from("");

        if self.team_id.is_empty() {
            errors.push_str("teamId required\n");
        }

        if self.project_id.is_empty() {
            errors.push_str("projectId required\n");
        }

        if self.codebase_id.is_empty() {
            errors.push_str("codebaseId required\n");
        }

        if self.cloud_front_domain.is_empty() {
            errors.push_str("cloudFrontDomain required\n");
        }

        if self.token.is_empty() {
            errors.push_str("harborToken required\n");
        }

        if self.github_url.is_empty() {
            errors.push_str("gitHubUrl required\n");
        }

        if !errors.is_empty() {
            panic!("Invalid request parameters: {}", errors);
        }

        Ok(())
    }
}

/// Commit represents the returned Json from a commits
/// request from the GitHub API.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: Option<String>
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
    pub full_name: Option<String>,

    /// Url of the repository.
    pub html_url: Option<String>,

    /// The default branch of the repo, usually master or main
    pub default_branch: Option<String>,

    /// The language of the code in the repo.
    pub language: Option<String>,

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
    pub last_hash: String,

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
    pub fn add_last_hash(&mut self, last_hash: String) {
        self.last_hash = last_hash.to_string();
    }

    /// Method to mark the Repository as empty
    ///
    pub fn mark_repo_empty(&mut self) {
        self.empty = true;
    }
}

#[test]
fn test_remove_clone() {
    let dir = std::env::temp_dir();
    println!("Temp Directory: {:#?}", dir)
}

#[tokio::test]
async fn test_should_skip_archived() {

    let client = Client::new();

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

    let mut test_counter = Counter::default();

    if !client.should_skip(&test_repo, test_name, test_url, &mut test_counter) {
        panic!("should_skip should be true for an archived repo");
    } else {
        if test_counter.archived != 1 {
            panic!("Counter did not count the repo as archived");
        }
    }
}

#[tokio::test]
async fn test_should_skip_disabled() {

    let client = Client::new();

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

    let mut test_counter = Counter::default();

    if !client.should_skip(&test_repo, test_name, test_url, &mut test_counter) {
        panic!("should_skip should be true for an disabled repo");
    } else {
        if test_counter.disabled != 1 {
            panic!("Counter did not count the repo as disabled");
        }
    }
}

#[tokio::test]
async fn test_should_skip_empty() {

    let client = Client::new();

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

    let mut test_counter = Counter::default();

    if !client.should_skip(&test_repo, test_name, test_url, &mut test_counter) {
        panic!("should_skip should be true for an empty repo");
    } else {
        if test_counter.empty != 1 {
            panic!("Counter did not count the repo as empty");
        }
    }
}