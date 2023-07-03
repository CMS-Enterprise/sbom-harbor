use platform::persistence::mongodb::{mongo_doc, MongoDocument};
use serde_derive::{Deserialize, Serialize};

/// Publish the service module
pub mod service;

/// Error module for the GitHub service
pub(crate) mod error;

/// GitHub Client. Used to for interacting with the GitHub API.
pub mod client;

/// Mongo Service implementation for the GitHub Provider
pub mod mongo;

/// Struct to define a GitHub Commit
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commit {
    /// Url of the GitHub Repository
    #[serde(default = "empty_string")]
    pub id: String,
    /// Last commit hash of the repository
    #[serde(alias = "sha")]
    pub last_hash: Option<String>,
}
mongo_doc!(Commit);

/// Little function to define default Strings
/// for struct values that are to be used to collect Json
fn empty_string() -> String {
    "".to_string()
}
