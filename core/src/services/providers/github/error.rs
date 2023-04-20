use thiserror::Error;
use platform::hyper;

/// Represents all handled Errors for the GitHub Crawler.
///
#[derive(Error, Debug)]
pub enum Error {

    /// Raised when we have a generic MongoDB Error
    ///
    #[error("error getting database: {0}")]
    MongoDb(String),

    /// This is raised when there is an issue creating entities
    ///
    #[error("error creating Harbor v1 entities: {0}")]
    EntityCreation(String),

    /// This is raised when there is a problem getting
    /// configuration from the environment.
    #[error("error creating Harbor v1 entities: {0}")]
    Configuration(String),

    /// This is raised when we are unable to upload to v1
    ///
    #[error("error uploading to v1: {0}")]
    SbomUpload(String),

    /// This error is raised when there is a problem communicating
    /// with GitHub over HTTP.
    ///
    #[error("error requesting from Github: {0}")]
    GitHubRequest(String),

    /// Error thrown only if the client is having trouble getting the last
    /// commit has from a given GetHub Repo
    ///
    #[error("error getting last hash from Github: {0}")]
    LastCommitHashError(u16, String),
}

impl From<platform::Error> for Error {
    fn from(err: platform::Error) -> Self {
        Error::MongoDb(
            err.to_string()
        )
    }
}
