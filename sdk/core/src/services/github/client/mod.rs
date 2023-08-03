use std::convert::TryFrom;
use std::path::Path;

use serde;
use serde::{Deserialize, Serialize};

use platform::hyper::{ContentType, Error as HyperError};

use crate::services::github::Commit;
use platform::hyper::Client as HttpClient;

use crate::services::github::error::Error;

const GH_URL: &str = "https://api.github.com";

#[derive(Debug)]
/// GitHub Client for hitting the HTTP API
pub struct Client {
    /// GitHub PAT
    token: String,
    http_client: HttpClient,
}

impl Client {
    /* Private */

    /// Creates the URL one must use in an http request for
    /// acquiring the latest commit hash from a given branch
    fn get_last_commit_url(&self, repo: &Repo) -> String {
        let full_name = repo.full_name.clone();
        let default_branch = repo.default_branch.as_ref().unwrap();
        format!("{GH_URL}/repos/{full_name}/commits/{default_branch}")
    }

    /// Creates the URL one must use in an http request for
    /// acquiring the latest commit hash from a given branch
    fn get_latest_release_url(&self, repo: &Repo) -> String {
        let full_name = repo.full_name.clone();
        format!("{GH_URL}/repos/{full_name}/releases/latest")
    }

    /// Function to get the number of repos in the associated organization
    /// gets public, private and forked repos.
    async fn get_num_repos(&self, org: &str) -> Result<Option<u32>, Error> {
        let org_url: String = format!("{GH_URL}/orgs/{org}");

        println!("==> Getting repositories from org, url: {} ", org_url);

        let option: Option<Org> = self
            .http_client
            .get(
                org_url.as_str(),
                ContentType::Json,
                self.token.as_str(),
                None::<String>,
            )
            .await
            .map_err(Error::GitHubResponse)?;

        match option {
            Some(value) => {
                let num_public_repos = value.public_repo_count;
                let num_private_repos = value.private_repo_count;

                println!(
                    "Found {} public repos and {} private repos",
                    num_public_repos, num_private_repos
                );

                Ok(Some(num_public_repos + num_private_repos))
            }

            None => Err(Error::GitHubErrorResponse(String::from(
                "==> Get request from GitHub had an empty response",
            ))),
        }
    }

    /* Public */

    /// Get the last commit for a given Repo
    pub async fn get_last_commit(&self, repo: &Repo) -> Result<Option<String>, Error> {
        let github_last_commit_url = self.get_last_commit_url(repo);

        println!(
            "==> getting last commit for repo : {:#?}",
            repo.full_name.clone()
        );

        let response: Result<Option<Commit>, HyperError> = self
            .http_client
            .get(
                github_last_commit_url.as_str(),
                ContentType::Json,
                self.token.as_str(),
                None::<String>,
            )
            .await;

        let gh_commits_rsp = match response {
            Ok(option) => match option {
                Some(last_hash) => last_hash,
                None => {
                    return Err(Error::GitHubErrorResponse(
                        "==> Last hash is missing:".to_string(),
                    ))
                }
            },
            Err(err) => {
                return if let HyperError::Remote(status, msg) = err {
                    Err(Error::LastCommitHashError(status, msg))
                } else {
                    Err(Error::GitHubErrorResponse(format!("{}", err)))
                }
            }
        };

        Ok(gh_commits_rsp.last_hash)
    }

    /// Gets the latest release tag name from the GitHub repo
    pub async fn get_latest_release_tag(&self, repo: &Repo) -> Result<String, Error> {
        let latest_release_url = self.get_latest_release_url(repo);

        println!(
            "==> getting latest release for repo : {:#?}",
            repo.full_name.clone()
        );

        let response: Result<Option<Release>, Error> = self
            .http_client
            .get(
                latest_release_url.as_str(),
                ContentType::Json,
                self.token.as_str(),
                None::<String>,
            )
            .await
            .map_err(|err| Error::GitHubErrorResponse(format!("{}", err)));

        let option = response.unwrap_or(Some(Release::new()));
        Ok(option.unwrap_or(Release::new()).tag_name)
    }

    /// Conventional Constructor.
    pub fn new(pat: String) -> Self {
        let token = format!("Bearer {}", pat);
        let http_client = HttpClient::new();
        Client { token, http_client }
    }

    /// Function to get the number of repos per page
    pub async fn get_pages(&self, org: &String) -> Result<Vec<u32>, Error> {
        let num_repos = self.get_num_repos(org.as_str()).await?.unwrap_or(0);

        println!("==> Number of Repositories in {}: {}", org, num_repos);

        let num_calls = ((num_repos / 100) as i8) + 1;
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
        let github_org_url =
            format!("{GH_URL}/orgs/{org}/repos?type=all&page={page}&per_page={per_page}");

        println!("==> calling: {}", github_org_url);

        let response: Result<Option<Vec<Repo>>, HyperError> = self
            .http_client
            .get(
                github_org_url.as_str(),
                ContentType::Json,
                self.token.as_str(),
                None::<String>,
            )
            .await;

        match response {
            Ok(option) => match option {
                Some(value) => {
                    println!("==> response is ok, {} repos returned", value.len());
                    Ok(value)
                }
                None => Err(Error::GitHubErrorResponse(
                    "Get request from GitHub had an empty response".to_string(),
                )),
            },
            Err(err) => Err(Error::GitHubResponse(err)),
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

/// Org is used to extract the number of Public Repos
/// in a given GitHub Organization.
#[derive(Debug, Serialize, Deserialize)]
pub struct Org {
    /// The number of Public Repos in
    /// this organization
    #[serde(default = "default_num_repos")]
    #[serde(alias = "public_repos")]
    public_repo_count: u32,

    /// Number of Private repos
    #[serde(default = "default_num_repos")]
    #[serde(alias = "total_private_repos")]
    private_repo_count: u32,
}

/// Function to provide a default  for the repo counts
fn default_num_repos() -> u32 {
    0
}

/// Repo is used to extract several values from a Request for
/// the Repositories in a given GitHub Organization
#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    /// The name of the repo.  Getting the Full name
    /// is nice because it has teh "user" in it: <USER>>/<REPO>.git
    /// Ex: CMSgov/design-system.git
    pub full_name: String,
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
    /// Latest release tag name
    #[serde(default = "empty_string")]
    pub version: String,
}

/// Repo impl
impl Repo {
    /// Conventional Constructor
    pub fn new() -> Self {
        Repo {
            full_name: String::from(""),
            html_url: None,
            default_branch: None,
            language: None,
            archived: None,
            disabled: None,
            empty: default_bool(),
            last_hash: empty_string(),
            version: empty_string(),
        }
    }

    /// This method allows us to add the last hash to a
    /// Repo if it is newer that what is already in Mongo
    pub fn add_last_hash(&mut self, last_hash: String) {
        self.last_hash = last_hash;
    }

    /// Method to add a version
    pub fn add_version(&mut self, version: String) {
        self.version = version;
    }

    /// Method to mark the Repository as empty
    pub fn mark_repo_empty(&mut self) {
        self.empty = true;
    }
}

/// Default Implementation
impl Default for Repo {
    fn default() -> Self {
        Repo::new()
    }
}

/// The Release struct is used to collect data from a response
/// from the /repos/{org}/{repo_name}/releases/latest endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    /// The tag name associated to the release
    #[serde(default = "default_version")]
    pub tag_name: String,
}

impl Release {
    fn new() -> Self {
        Release {
            tag_name: default_version(),
        }
    }
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

/// Function used to define a default version for Harbor generated purls.
/// The GitHub Provider currently requests the release tags from each repository
/// and selects the latest tag name to use as the purl version.  If no tag exists,
/// then this version is used instead.  The 'd' in the version is used to flag that
/// the version is the default version that came from this function while the form
/// of a semantic version "0.0.0" is used to easily recognize that the string is a
/// version.
pub fn default_version() -> String {
    String::from("d0.0.0")
}

#[cfg(test)]
mod test {
    use crate::services::github::client::{Client, Repo};
    use crate::services::github::error::Error;
    use platform::config::from_env;

    /// Requires GITHUB_PAT environment variable
    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_last_release() -> Result<(), Error> {
        let pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No GITHUB_PAT in environment"), // test panic
        };

        let client = Client::new(pat);

        let mut repo: Repo = Repo::default();
        repo.full_name = String::from("harbor-test-org/java-multi-module");

        let release = client.get_latest_release_tag(&repo).await?;

        assert_eq!("0.0.2", release);

        Ok(())
    }

    /// This test must be run with a GitHub PAT that has the correct permissions
    /// Fine grained PATs do not work, the token must be a classic PAT with these perms:
    /// - repo (all)
    /// - admin:public_key
    /// - admin:org, only read:org
    /// This is a good place to test a PAT from GItHub
    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_page_repos() -> Result<(), Error> {
        // https://github.com/orgs/harbor-test-org/repositories
        let total_repos_in_harbor_test_org = 11;

        let pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No GITHUB_PAT in environment"), // test panic
        };

        let page_num = 1;
        let num_per_page = 100 as u32;

        let client = Client::new(pat);
        let repo_page = client
            .get_page_of_repos(&String::from("harbor-test-org"), page_num, &num_per_page)
            .await;

        match repo_page {
            Ok(repos) => {
                assert_eq!(repos.len(), total_repos_in_harbor_test_org);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
