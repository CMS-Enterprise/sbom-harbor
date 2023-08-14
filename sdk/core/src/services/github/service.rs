use crate::services::github::client::{Client as GitHubClient, Repo};
use crate::services::github::error::Error;
use crate::services::github::Commit;
use async_trait::async_trait;
use platform::git::Service as Git;
use platform::persistence::mongodb::Service as MongoService;
use platform::persistence::mongodb::Store;
use platform::str::get_random_string;
use platform::Error as PlatformError;
use std::sync::Arc;

/// Definition of the GitHubProvider
#[derive(Debug)]
pub struct GitHubService {
    org: String,
    client: GitHubClient,
    store: Arc<Store>,
}

/// Impl for GitHubSbomProvider
impl GitHubService {
    /// new method sets the organization for the struct
    pub fn new(org: String, pat: String, store: Arc<Store>) -> Self {
        let client = GitHubClient::new(pat);
        Self { org, client, store }
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
                    Ok(last_hash) => repo.add_last_hash(last_hash),
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
    pub fn find_build_targets(
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

#[async_trait]
impl MongoService<Commit> for GitHubService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }

    /// Insert a document into a [Collection].
    async fn insert<'a>(&self, doc: &mut Commit) -> Result<(), PlatformError> {
        if doc.id.is_empty() {
            return Err(PlatformError::Mongo(String::from("==> commit::id::empty")));
        }

        if doc.url.is_empty() {
            return Err(PlatformError::Mongo(String::from("==> commit::url::empty")));
        }

        self.store.insert(doc).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::dev_context;
    use crate::services::github::error::Error;
    use crate::services::github::service::GitHubService;
    use platform::config::from_env;
    use platform::persistence::mongodb::Store;
    use std::sync::Arc;

    async fn test_store() -> Arc<Store> {
        let ctx = dev_context(None).unwrap();
        let store = Store::new(&ctx).await.unwrap();
        Arc::new(store)
    }

    /// Ensure version exists in repos
    /// Requires GITHUB_PAT environment variable!
    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_ensure_release_not_empty() -> Result<(), Error> {
        let test_pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No TEST_PAT in environment"), // test panic
        };

        let service = GitHubService::new(
            String::from("harbor-test-org"),
            test_pat,
            test_store().await,
        );

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

        let service = GitHubService::new(
            String::from("harbor-test-org"),
            test_pat,
            test_store().await,
        );

        let repos = service.get_repos().await;
        println!("Repos: {:#?}", repos);
        assert!(repos.is_ok());
        let _repos = repos.unwrap();
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_clone_repo() {
        let test_pat = match from_env("GITHUB_PAT") {
            Some(v) => v,
            None => panic!("No GITHUB_PAT in environment"), // test panic
        };

        let repo = "https://github.com/harbor-test-org/java-repo.git";

        let service = GitHubService::new(
            String::from("harbor-test-org"),
            test_pat.clone(),
            test_store().await,
        );

        let clone_path = match service.clone_repo(repo, Some(test_pat)) {
            Ok(clone_path) => clone_path,
            Err(err) => panic!("{:#?}", err),
        };

        service
            .remove_clone(clone_path.as_str())
            .unwrap_or_else(|_| panic!("Unable to remove clone path: {}", clone_path));
    }
}
