use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use mongodb::bson::doc;
use platform::mongodb::{Service, Store};

use crate::commands::get_env_var;
use crate::config::*;
use crate::services::github::mongo::GitHubProviderDocument;
use crate::services::providers::github::mongo::GitHubProviderDocument;

mod mongo;
mod repo;
pub mod provider;

/// Args for generating one ore more SBOMs from a GitHub Organization.
#[derive(Clone, Debug, Parser)]
pub struct GitHubProviderConfig {

    /// This is the GUID that is in DynamoDB that
    /// belongs to the team we are using.
    cms_team_id: String,

    /// This is the token from that team
    cms_team_token: String,

    /// This is the Cloudfront Domain of the API endpoints
    cf_domain: String,

    /// The username we use to get the JWT and make API calls
    cognito_username: String,

    /// The password we use to get the JWT and make API calls
    cognito_password: String,
}

#[derive(Debug)]
pub struct GitHubProviderService {
    store: Arc<Store>,
}

impl GitHubProviderService {
    /// Factory method to create new instances of a [TeamService].
    pub fn new(store: Arc<Store>) -> GitHubProviderService {
        GitHubProviderService { store }
    }
}

impl Service<GitHubProviderDocument> for GitHubProviderService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

/// Snag a bunch of environment variables and put them into a struct
///
fn get_harbor_config() -> Result<GitHubProviderConfig, GhProviderError> {

    let cms_team_id = match get_cms_team_id() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team Id of V1 Team")
            )
        )
    };

    let cms_team_token = match get_cms_team_token() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team token of V1 Team")
            )
        )
    };

    let cf_domain = match get_cf_domain() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_username = match get_v1_username() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_password = match get_v1_password() {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Password")
            )
        )
    };

    Ok(
        GitHubProviderConfig {
            cms_team_id,
            cms_team_token,
            cf_domain,
            cognito_username,
            cognito_password,
        }
    )
}

/// The Counter struct is used to keep track of
/// what happened to an attempt to submit an SBOM.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Counter {

    /// This value is incremented if the Repo is archived
    archived: i32,

    /// This value is incremented if the Repo is disabled
    disabled: i32,

    /// This value is incremented if the Repo is empty
    empty: i32,

    /// This value is incremented if the Repo is processed successfully
    processed: i32,

    /// This value is incremented if the last commit hash of
    /// the repo is in the database already. This happens when
    /// there has been no change in the repo since last run
    hash_matched: i32,

    /// This value is incremented if there is an error when trying to upload the SBOM.
    upload_errors: i32,
}

/// Default, completely 0'd out default Counter
///
impl Default for Counter {
    fn default() -> Self {
        Self {
            archived: 0,
            disabled: 0,
            empty: 0,
            processed: 0,
            hash_matched: 0,
            upload_errors: 0,
        }
    }
}

/// Represents all handled Errors for the GitHub Crawler.
///
#[derive(Error, Debug)]
pub enum GhProviderError {

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

    /// This is Raised when the Pilot has issues doing its job
    #[error("error running pilot: {0}")]
    Pilot(String),

    /// This error is raised when there is a problem communicating
    /// with GitHub over HTTP.
    #[error("error running pilot: {0}")]
    GitHubRequest(String),
}
