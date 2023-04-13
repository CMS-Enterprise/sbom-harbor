use thiserror::Error;

/// Represents all handled Errors for the GitHub Crawler.
///
#[derive(Error, Debug)]
pub enum Error {

    /// Raised when we have a generic MongoDB Error
    #[error("error getting database: {0}")]
    MongoDb(String),

    /// This is raised when there is an issue creating entities
    #[error("error creating Harbor v1 entities: {0}")]
    EntityCreation(String),

    /// This is raised when there is a problem getting
    /// configuration from the environment.
    #[error("error creating Harbor v1 entities: {0}")]
    Configuration(String),

    /// This is raised when we are unable to upload to v1
    #[error("error running pilot: {0}")]
    SbomUpload(String),

    /// This error is raised when there is a problem communicating
    /// with GitHub over HTTP.
    #[error("error running pilot: {0}")]
    GitHubRequest(String),
}