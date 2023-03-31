use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::fmt::Error;
use std::ops::Deref;
use std::pin::Pin;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use harbcore::services::{clone_path, clone_repo, remove_clone, syft};
use harbor_client::client::{Client as V1HarborClient, SBOMUploadResponse};
use platform::hyper::{ContentType, Error as HyperError, get};

use crate::commands::{get_env_var};

mod mongo;
mod repo;
pub mod provider;

pub const DB_IDENTIFIER: &str = "harbor";
pub const KEY_NAME: &str = "id";
pub const COLLECTION: &str = "pilot";

pub const TEAM_ID_KEY: &str = "team_id";
pub const CF_DOMAIN_KEY: &str = "CF_DOMAIN";
pub const PROJECT_ID_KEY: &str = "project_id";
pub const CODEBASE_ID_KEY: &str = "codebase_id";
pub const GH_FT_KEY: &str = "GH_FETCH_TOKEN";
pub const V1_TEAM_ID_KEY: &str = "V1_CMS_TEAM_ID";
pub const V1_TEAM_TOKEN_KEY: &str = "V1_CMS_TEAM_TOKEN";
pub const V1_HARBOR_USERNAME_KEY: &str = "V1_HARBOR_USERNAME";
pub const V1_HARBOR_PASSWORD_KEY: &str = "V1_HARBOR_PASSWORD";

/// Configuration from the environment for V1 Harbor
///
struct HarborConfig {
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

/// Snag a bunch of environment variables and put them into a struct
///
fn get_harbor_config() -> Result<HarborConfig, GhProviderError> {

    let cms_team_id = match get_env_var(V1_TEAM_ID_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team Id of V1 Team")
            )
        )
    };

    let cms_team_token = match get_env_var(V1_TEAM_TOKEN_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Team token of V1 Team")
            )
        )
    };

    let cf_domain = match get_env_var(CF_DOMAIN_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_username = match get_env_var(V1_HARBOR_USERNAME_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Username")
            )
        )
    };

    let cognito_password = match get_env_var(V1_HARBOR_PASSWORD_KEY) {
        Some(value) => value,
        None => return Err(
            GhProviderError::Configuration(
                String::from("Missing Cognito Password")
            )
        )
    };

    Ok(
        HarborConfig {
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

    /// This value is incremented
    /// if the Repo is archived
    archived: i32,

    /// This value is incremented
    /// if the Repo is disabled
    disabled: i32,

    /// This value is incremented
    /// if the Repo is empty
    empty: i32,

    /// This value is incremented
    /// if the Repo is processed successfully
    processed: i32,

    /// This value is incremented
    /// if the last commit hash of
    /// the repo is in the database
    /// already. This happens when
    /// there has been no change in
    /// the repo since last run
    hash_matched: i32,

    /// This value is incremented if there is an
    /// error when trying to upload the SBOM.
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

    /// This a raised if there is a problem when getting the
    /// collection from MongoDB
    #[error("error getting collection: {0}")]
    MongoCollection(String),

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
