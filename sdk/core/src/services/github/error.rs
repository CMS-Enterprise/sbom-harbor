use thiserror::Error;
use platform::{Error as PlatformError};
use platform::hyper::{Error as PlatformHttpError};
use crate::{Error as CoreError};

/// Represents all handled Errors for the GitHub Crawler.
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
    GitHubEmptyResponse(String),
    /// Error thrown only if the client is having trouble getting the last
    /// commit has from a given GetHub Repo
    #[error("error getting last hash from Github: {0}")]
    LastCommitHashError(u16, String),
}
