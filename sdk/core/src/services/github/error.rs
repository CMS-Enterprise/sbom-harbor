use crate::Error as CoreError;
use platform::hyper::Error as PlatformHttpError;
use platform::Error as PlatformError;
use thiserror::Error;

/// Represents all handled Errors for the GitHub Provider.
///
#[derive(Error, Debug)]
pub enum Error {
    /// Raised when we have a generic MongoDB Error
    #[error(transparent)]
    MongoDb(#[from] PlatformError),
    /// This is raised when there is a problem getting
    /// configuration from the environment.
    #[error(transparent)]
    Configuration(#[from] CoreError),
    /// This error is raised when there is a problem communicating
    /// with GitHub over HTTP.
    #[error(transparent)]
    GitHubResponse(#[from] PlatformHttpError),
    /// This error is raised when there is a problem communicating
    /// with GitHub over HTTP.
    #[error("empty response from Github: {0}")]
    GitHubErrorResponse(String),
    /// Error thrown only if the client is having trouble getting the last
    /// commit has from a given GetHub Repo
    #[error("error getting last hash from Github: {0}")]
    LastCommitHashError(u16, String),
}
