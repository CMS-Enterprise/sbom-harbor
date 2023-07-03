use std::convert::TryFrom;
use std::path::Path;

use git2::Repository;
use serde;
use serde::{Deserialize, Serialize};
use tracing::info;

use platform::hyper::{
    ContentType,
    Error as HyperError,
};

use platform::hyper::{Client as HttpClient};
use crate::services::github::Commit;

use crate::services::github::error::Error;

const GH_URL: &str = "https://api.github.com";

#[derive(Debug)]
/// GitHub Client for hitting teh HTTP API
pub struct Client {
    /// GitHub PAT
    token: String,
    http_client: HttpClient
}

impl Client {

    /* Private */

    /// Creates the URL one must use in an http request for
    /// acquiring the latest commit hash from a given branch
    fn get_last_commit_url(&self, repo: &mut Repo) -> String {
        let repo_name = repo.full_name.as_ref().unwrap();
        let default_branch = repo.default_branch.as_ref().unwrap();
        format!("{GH_URL}/repos/{repo_name}/commits/{default_branch}")
    }

    /// Function to get the number of public repos in the associated organization
    async fn get_num_pub_repos(&self, org: String) -> Result<Option<u32>, Error> {

        let org_url: String = format!("{GH_URL}/orgs/{org}");

        let response: Result<Option<Org>, HyperError> = self.http_client.get(
            org_url.as_str(),
            ContentType::Json,
            self.token.as_str(),
            None::<String>,
        ).await;

        match response {
            Ok(option) => match option {
                Some(value) => Ok(value.public_repo_count),
                None => Err(
                    Error::GitHubErrorResponse(
                        "Get request from GitHub had an empty response".to_string()
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
    pub async fn get_last_commit(&self, repo: &mut Repo)
        -> Result<Option<String>, Error> {

        let github_last_commit_url = self.get_last_commit_url(repo);

        let response: Result<Option<Commit>, HyperError> = self.http_client.get(
            github_last_commit_url.as_str(),
            ContentType::Json,
            self.token.as_str(),
            None::<String>,
        ).await;

        let gh_commits_rsp = match response {
            Ok(option) => match option {
                Some(last_hash) => last_hash,
                None => return Err(
                    Error::GitHubErrorResponse(
                        "==> Last hash is missing:".to_string()
                    )
                )
            },
            Err(err) => {
                return if let HyperError::Remote(status, msg) = err {
                    Err(Error::LastCommitHashError(status, msg))
                } else {
                    Err(Error::GitHubErrorResponse(format!("{}", err)))
                }
            },
        };

        match gh_commits_rsp.last_hash {
            Some(val) => Ok(Some(val)),
            None => Ok(None),
        }
    }

    /// Conventional Constructor.
    pub fn new(pat: String) -> Self {
        let token = format!("Bearer {}", pat);
        let http_client = HttpClient::new();
        Client {
            token,
            http_client
        }
    }

    /// Function to get the number of repos per page
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
    pub async fn get_page_of_repos(
        &self,
        org: &String,
        page: usize,
        per_page: &u32,
    ) -> Result<Vec<Repo>, Error> {

        let github_org_url = format!("{GH_URL}/orgs/{org}/repos?type=sources&page={page}&per_page={per_page}");

        println!("Calling({})", github_org_url);

        let response: Result<Option<Vec<Repo>>, HyperError> = self.http_client.get(
            github_org_url.as_str(),
            ContentType::Json,
            self.token.as_str(),
            None::<String>,
        ).await;

        match response {
            Ok(option) => match option {
                Some(value) => Ok(value),
                None => Err(
                    Error::GitHubErrorResponse(
                        "Get request from GitHub had an empty response".to_string()
                    )
                ),
            },
            Err(err) => Err(
                Error::GitHubResponse(err)
            ),
        }
    }

    /// Clones a git repository to the specified clone path.
    pub fn clone_repo(&self, clone_path: &str, url: &str) -> Result<String, Error> {

        println!("==> Cloning repo: {}", url);

        match Repository::clone(url, clone_path) {
            Err(err) => return Err(
                Error::GitHubErrorResponse(
                    format!("==> error cloning repository from {}: {}", url, err)
                )
            ),
            _ => info!("Successfully cloned repo"),
        };

        Ok(clone_path.to_string())
    }

    /// Removes a cloned repository from the filesystem.
    pub fn remove_clone(&self, clone_path: &str) -> std::io::Result<()> {
        if Path::new(&clone_path).is_dir() {
            return std::fs::remove_dir_all(clone_path);
        }

        Ok(())
    }
}

/// Org is used to extract the number of Public Repos
/// in a given GitHub Organization.
#[derive(Debug, Serialize, Deserialize)]
pub struct Org {
    /// The number of Public Repos in
    /// this organization
    public_repo_count: Option<u32>
}

/// Repo is used to extract several values from a Request for
/// the Repositories in a given GitHub Organization
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
    pub(crate) archived: Option<bool>,
    /// Is this repo disabled?
    pub(crate) disabled: Option<bool>,
    /// Is this repo empty?
    #[serde(default = "default_bool")]
    pub(crate) empty: bool,
    /// This is the most recent commit
    /// hash of the default branch
    #[serde(default = "empty_string")]
    pub last_hash: String,

}

/// Little function to define default booleans
/// for struct values that are to be used to collect Json
fn default_bool() -> bool {
    false
}

/// Little function to define default Strings
/// for struct values that are to be used to collect Json
fn empty_string() -> String {
    "".to_string()
}

/// Repo impl
impl Repo {

    /// This method allows us to add the last hash to a
    /// Repo if it is newer that what is already in Mongo
    ///
    pub fn add_last_hash(&mut self, last_hash: String) {
        self.last_hash = last_hash;
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