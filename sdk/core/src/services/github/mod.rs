use platform::persistence::mongodb::{mongo_doc, MongoDocument};
use serde_derive::{Deserialize, Serialize};

/// Publish the service module
pub mod service;

/// Error module for the GitHub service
pub(crate) mod error;

/// GitHub Client. Used to for interacting with the GitHub API.
pub mod client;

/// Struct to define a GitHub Commit
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commit {
    /// This is the last commit of the "default" branch in the repo
    #[serde(alias = "sha")]
    pub id: String,
    /// Url of the GitHub Repository
    pub url: String,
}
mongo_doc!(Commit);
