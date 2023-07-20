use crate::services::github::client::{Client as GitHubClient, Repo};
use crate::services::github::error::Error;
use platform::git::Service as Git;
use platform::str::get_random_string;

/// Definition of the GitHubProvider
#[derive(Debug)]
pub struct GitHubService {
    org: String,
    client: GitHubClient,
}

/// Impl for GitHubSbomProvider
impl GitHubService {
    /// new method sets the organization for the struct
    pub fn new(org: String, pat: String) -> Self {
        let github_client = GitHubClient::new(pat);
        GitHubService {
            org,
            client: github_client,
        }
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

            println!("==> processing page {}. There are {} repos", page, per_page);

            for repo in gh_org_rsp.iter_mut() {
                let version = self.client.get_latest_release_tag(repo).await?;
                repo.add_version(version);

                let last_hash_result = self.client.get_last_commit(repo).await;
                match last_hash_result {
                    Ok(option) => match option {
                        Some(last_hash) => repo.add_last_hash(last_hash),
                        None => println!(
                            "==> No last commit has found for Repo: {}",
                            repo.full_name.clone()
                        ),
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

    /// Find files of a certain name in a git repo using the git2 crate
    pub fn find(
        &self,
        url: String,
        file_name: String,
        clone_path: String,
    ) -> Result<Vec<String>, Error> {
        Git::new(url)
            .find(file_name, clone_path)
            .map_err(|_err| Error::Find())
    }

    /// Clones a git repository to the specified clone path.
    pub fn clone_repo(&self, url: &str, pat: Option<String>) -> Result<String, Error> {
        let clone_path = self.clone_path(url, &get_random_string())?;
        let git_service = Git::new(String::from(url));
        git_service.clone_repo(clone_path.as_str(), pat)?;
        Ok(clone_path)
    }

    /// Generates a unique clone path for a repository.
    fn clone_path(&self, url: &str, rand: &String) -> Result<String, Error> {
        let repo_name = url
            .split('/')
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .replace(".git", "");

        Ok(format!("/tmp/harbor/{}/{}", rand, repo_name))
    }

    /// Removes a cloned repository from the filesystem.
    pub fn remove_clone(&self, clone_path: &str) -> Result<(), Error> {
        // Clippy insight made this very clean. Lets focus on using thiserror like this.
        Git::remove_clone(clone_path).map_err(Error::CloneRepo)
    }
}

#[cfg(test)]
mod tests {
    use crate::services::github::error::Error;
    use crate::services::github::service::GitHubService;
    use platform::config::from_env;

    /// Ensure version exists in repos
    /// Requires GITHUB_PAT environment variable!
    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_ensure_release_not_empty() -> Result<(), Error> {
        let test_pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No TEST_PAT in environment"), // test panic
        };

        let service = GitHubService::new(String::from("harbor-test-org"), test_pat);

        service
            .get_repos()
            .await?
            .iter()
            .for_each(|repo| assert!(!repo.version.is_empty()));

        Ok(())
    }

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

        let service = GitHubService::new(String::from("harbor-test-org"), test_pat);

        let repos = service.get_repos().await;
        println!("Repos: {:#?}", repos);
        assert!(repos.is_ok());
        let _repos = repos.unwrap();
    }

    #[test]
    #[ignore = "debug manual only"]
    fn test_clone_repo() {
        let test_pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No GITHUB_PAT in environment"), // test panic
        };

        let repo = "https://github.com/harbor-test-org/java-repo.git";

        let service = GitHubService::new("harbor-test-org".to_string(), test_pat.clone());

        let clone_path = match service.clone_repo(repo, Some(test_pat)) {
            Ok(clone_path) => clone_path,
            Err(err) => panic!("{:#?}", err),
        };

        service
            .remove_clone(clone_path.as_str())
            .expect(format!("Unable to remove clone path: {}", clone_path).as_str());
    }
}
