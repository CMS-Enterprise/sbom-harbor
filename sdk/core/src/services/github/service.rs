use crate::services::github::client::{Client as GitHubClient, Repo};
use crate::services::github::error::Error;
use std::path::Path;

/// Definition of the GitHubProvider
#[derive(Debug)]
pub struct GitHubService {
    org: String,
    client: GitHubClient,
}

/// Impl for GitHubSbomProvider
///
impl GitHubService {
    /// new method sets the organization for the struct
    ///
    pub fn new(org: String, pat: String) -> Self {
        let client = GitHubClient::new(pat);
        GitHubService { org, client }
    }

    /// Use the GitHub Client to get all of the repositories
    /// from the GitHub Organization
    pub(crate) async fn get_repos(&self) -> Result<Vec<Repo>, Error> {
        let mut pages = match self.client.get_pages(&self.org).await {
            Ok(pages) => pages,
            Err(err) => {
                return Err(Error::GitHubErrorResponse(format!(
                    "Unable to get pages of repos from GitHub: {:#?}",
                    err
                )))
            }
        };

        let mut repo_vec: Vec<Repo> = Vec::new();
        for (page, per_page) in pages.iter_mut().enumerate() {
            let mut gh_org_rsp = self
                .client
                .get_page_of_repos(&self.org, page + 1, per_page)
                .await?;

            for repo in gh_org_rsp.iter_mut() {
                let result = self.client.get_last_commit(repo).await;
                let repo_name = repo.full_name.clone().unwrap();

                match result {
                    Ok(option) => match option {
                        Some(last_hash) => repo.add_last_hash(last_hash),
                        None => println!("==> No last commit has found for Repo: {}", &repo_name),
                    },
                    Err(err) => {
                        if let Error::LastCommitHashError(status, _msg) = err {
                            if status == 409 {
                                repo.mark_repo_empty();
                            }
                        } else {
                            println!("Unexpected error: {:#?}", err);
                            continue;
                        }
                    }
                }
            }

            repo_vec.extend(gh_org_rsp);
        }

        Ok(repo_vec)
    }

    /// Clones a git repository to the specified clone path.
    pub fn clone_repo(&self, url: &str, last_hash: &str) -> Result<String, Error> {
        let clone_path = self.clone_path(url, &last_hash.to_string())?;
        self.client.clone_repo(clone_path.as_str(), url)
    }

    /// Generates a unique clone path for a repository.
    pub fn clone_path(&self, url: &str, hash: &String) -> Result<String, Error> {
        let repo_name = url
            .split('/')
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .replace(".git", "");

        Ok(format!("/tmp/harbor-debug/{}/{}", hash, repo_name))
    }

    /// Removes a cloned repository from the filesystem.
    pub fn remove_clone(&self, clone_path: &str) -> std::io::Result<()> {
        if Path::new(&clone_path).is_dir() {
            return std::fs::remove_dir_all(clone_path);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::services::github::client::Client;
    use crate::services::github::service::GitHubService;
    use platform::config::from_env;

    /// For this test to work, one must be in the harbor-test-org
    /// and have a test PAT in their environment with permissions
    /// to list repositories
    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_repos() {
        let test_pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No TEST_PAT in environment"), // test panic
        };

        let client = Client::new(test_pat);
        let service = GitHubService {
            org: "harbor-test-org".to_string(),
            client,
        };

        let repos = service.get_repos().await;
        println!("Repos: {:#?}", repos);
        assert!(repos.is_ok());
        let _repos = repos.unwrap();
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    fn test_clone_repo() {
        let test_pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No GITHUB_PAT in environment"), // test panic
        };

        let last_hash = "BSLASTHASH";
        let repo = "https://github.com/harbor-test-org/java-repo.git";

        let client = Client::new(test_pat);
        let service = GitHubService {
            org: "harbor-test-org".to_string(),
            client,
        };

        let clone_path = match service.clone_repo(repo, last_hash) {
            Ok(clone_path) => clone_path,
            Err(err) => panic!("{}", err),
        };

        println!("THE FUCKING RESULT: {:#?}", clone_path);

        service
            .remove_clone(clone_path.as_str())
            .expect(format!("Unable to remove clone path: {}", last_hash).as_str());
    }
}
