use git2::Error as GitError;
use thiserror::Error;

/// Errors specific to Git operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Raised when there is an error while attempting to clone a Repository
    #[error(transparent)]
    GitClientError(#[from] GitError),
    /// Specific Error for problems centered around the repo being cloned.
    #[error("clone error: {0}")]
    CloneError(String),
}
