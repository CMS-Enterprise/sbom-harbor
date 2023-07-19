/// Errors for git Module
pub mod error;

use crate::filesystem::remove_directory;
use crate::git::error::Error;
use git2::{Repository, TreeWalkResult};
use std::path::Path;

/// Git Service
pub struct Service {
    /// This value is the url of the repository
    repo_url: String,
}

impl Service {
    /// Conventional Constructor
    pub fn new(repo_url: String) -> Self {
        Service { repo_url }
    }

    /// Clones a git repository to the specified clone path.
    pub fn clone_repo(&self, clone_path: &str) -> Result<(), Error> {
        println!("==> Cloning repo: {}", self.repo_url);

        let path = Path::new(clone_path);

        if path.is_dir() {
            // TODO Maybe add 'pull' functionality later to update the repo
            println!(
                "Repo ({}) has already been cloned ({})",
                self.repo_url, clone_path
            );
        } else {
            match Repository::clone(self.repo_url.as_str(), clone_path) {
                Err(err) => {
                    println!("==> Error cloning Repository");
                    return Err(Error::GitClientError(err));
                }
                _ => {
                    println!("==> Successfully cloned repo: {}", clone_path);
                }
            };
        }

        Ok(())
    }

    /// Removes a cloned repository from the filesystem.
    pub fn remove_clone(clone_path: &str) -> Result<(), Error> {
        if Path::new(&clone_path).is_dir() {
            return remove_directory(String::from(clone_path))
                .map_err(|e| Error::CloneError(e.to_string()));
        }

        Ok(())
    }

    /// Find files of a certain name in a git repo using the git2 crate
    pub fn find(&self, file_name: String, clone_path: String) -> Result<Vec<String>, Error> {
        let path = Path::new(clone_path.as_str());

        if !path.is_dir() {
            let err = format!("Repo ({}) has not been cloned", self.repo_url);
            println!("{}", err);
            Err(Error::CloneError(err))
        } else {
            let repo = Repository::open(path)?;
            let head = repo.head()?.peel_to_tree()?;

            let mut files = Vec::new();

            head.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
                if let Some(name) = entry.name() {
                    if name == file_name {
                        if root.ends_with('/') {
                            files.push(format!("{}{}", root, name));
                        } else {
                            files.push(format!("{}/{}", root, name));
                        }
                    }
                }

                TreeWalkResult::Ok
            })?;

            Ok(files)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::filesystem::get_tmp_location;
    use crate::git::Service as Git;
    use crate::str::get_random_string;
    use std::path::Path;

    #[tokio::test]
    async fn test_good_clone() {
        let repo_location = "/tmp/test-java-repo";

        let repo_url = "https://github.com/harbor-test-org/java-repo.git".to_string();
        let git_svc = Git::new(repo_url);
        git_svc
            .clone_repo(repo_location)
            .expect("TEST ERROR: Unable to clone repo");

        let path = Path::new(repo_location);
        assert!(path.is_dir());
        Git::remove_clone(repo_location).expect("Error removing clone");
    }

    #[tokio::test]
    async fn test_no_repo() {
        let repo_location = get_tmp_location();

        let rand_str: String = get_random_string();

        let repo_url = format!("https://github.com/harbor-test-org/{}.git", rand_str);
        let git_svc = Git::new(repo_url);
        match git_svc.clone_repo(repo_location.as_str()) {
            Ok(_) => panic!("Should not be able to clone repo"),
            _ => (),
        }
    }

    #[test]
    fn test_find() {
        let repo_location = get_tmp_location();

        let repo_url = "https://github.com/harbor-test-org/java-repo.git".to_string();
        let git_svc = Git::new(repo_url);

        git_svc
            .clone_repo(repo_location.as_str())
            .expect("Unable to clone Repo!");
        let result = git_svc.find("pom.xml".to_string(), repo_location.clone());
        Git::remove_clone(repo_location.as_str()).expect("Error removing clone");
        assert!(result.is_ok());

        let files = result.unwrap();
        assert!(files.iter().any(|file| file.contains("pom.xml")));
    }

    #[test]
    fn test_find_multi() {
        let repo_location = get_tmp_location();
        let build_target = String::from("pom.xml");

        // This repo has multiple build target files
        let repo_url = "https://github.com/harbor-test-org/java-multi-module.git".to_string();

        // Create an instance of Git Service
        let git_svc = Git::new(repo_url);

        // Clone the repo
        git_svc
            .clone_repo(repo_location.as_str())
            .expect("Unable to clone Repo!");

        // Find the build file
        let result = git_svc.find(build_target.clone(), repo_location.clone());
        assert!(result.is_ok());

        // Assert that *each* file path has a build target in it
        let paths = result.unwrap();
        paths.iter().for_each(|path| {
            assert!(path.contains(build_target.as_str()));
        });

        // Remove the clone for the next test
        Git::remove_clone(repo_location.as_str()).expect("Error removing clone");
    }
}
