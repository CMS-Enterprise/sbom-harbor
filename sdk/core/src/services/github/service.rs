
use crate::services::github::error::Error;
use crate::services::github::client::{
    Client as GitHubClient,
    Repo,
};

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
        GitHubService {
            org,
            client,
        }
    }

    /// Use the GitHub Client to get all of the repositories
    /// from the GitHub Organization
    pub(crate) async fn get_repos(&self) -> Result<Vec<Repo>, Error> {

        let mut pages = match self.client.get_pages(&self.org).await {
            Ok(pages) => pages,
            Err(err) => return Err(
                Error::GitHubErrorResponse(
                    format!("Unable to get pages of repos from GitHub: {:#?}", err)
                )
            )
        };

        let mut repo_vec: Vec<Repo> = Vec::new();
        for (page, per_page) in pages.iter_mut().enumerate() {

            let mut gh_org_rsp = self.client.get_page_of_repos(&self.org, page + 1, per_page).await?;

            for repo in gh_org_rsp.iter_mut() {

                let result = self.client.get_last_commit(repo).await;
                let repo_name = repo.full_name.clone().unwrap();

                match result {
                    Ok(option) => match option {
                        Some(last_hash) => repo.add_last_hash(last_hash),
                        None => println!("==> No last commit has found for Repo: {}", &repo_name)
                    },
                    Err(err) => {
                        if let Error::LastCommitHashError(status, _msg) = err {
                            if status == 409 {
                                repo.mark_repo_empty();
                            }
                        } else {
                            println!("Unexpected error: {:#?}", err);
                            continue
                        }
                    }
                }
            }

            repo_vec.extend(gh_org_rsp);
        }

        Ok(repo_vec)
    }

    /// Clones a git repository to the specified clone path.
    pub fn clone_repo(&self, clone_path: &str, url: &str) -> Result<(), Error> {
        self.client.clone_repo(clone_path, url)
    }

    /// Generates a unique clone path for a repository.
    pub fn clone_path(&self, url: &str, hash: &String) -> String {
        self.client.clone_path(url, hash)
    }

    /// Removes a cloned repository from the filesystem.
    pub fn remove_clone(&self, clone_path: &str) -> std::io::Result<()> {
        self.client.remove_clone(clone_path)
    }
}


#[cfg(test)]
mod tests {
    use platform::config::from_env;
    use crate::services::github::service::GitHubService;
    use crate::services::github::client::Client;

    /// For this test to work, one must be in the harbor-test-org
    /// and have a test PAT in their environment with permissions
    /// to list repositories
    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_repos() {

        let test_pat = match from_env("TEST_PAT") {
            Some(v) => v,
            None => panic!("No TEST_PAT in environment") // test panic
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

    #[test]
    fn test_clone_repo() {

        let clone_path = "/tmp/clone-test";
        let eslint_repo = "https://github.com/eslint/eslint.git";

        let client = Client::new("test_pat".to_string());
        let service = GitHubService {
            org: "test_org".to_string(),
            client,
        };

        let result = service.clone_repo(clone_path, eslint_repo);
        assert!(result.is_ok());
        service.remove_clone(clone_path).expect(
            format!("Unable to remove clone path: {}", clone_path).as_str()
        );
    }
}
