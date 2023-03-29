use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use aws_config::environment::EnvironmentVariableRegionProvider;
use aws_config::meta::region::RegionProviderChain;
use aws_config as sdk;
use aws_sdk_secretsmanager::{Client, Error as awsError};

use crate::Error;

pub async fn get_secret(secret_name: &str) -> Result<Option<String>, awsError> {
    println!("Getting Secret {} from AWS", secret_name);

    let region_provider = RegionProviderChain::default_provider()
        .or_else(EnvironmentVariableRegionProvider::new());

    let shared_config = sdk::from_env()
        .region(region_provider)
        .load()
        .await;
    
    let client = Client::new(&shared_config);
    let result = client.get_secret_value().secret_id(secret_name).send().await?;
    if let Some(secret) = result.secret_string() {
        return Ok(Some(secret.to_string()));
    }
    Ok(None)
}

/// Used to authorize whether a [User] can perform an [Action] against a [Resource].
#[async_trait]
pub trait Authorizer {
    /// Determines the authorization [Effect] for the [Resource] (e.g. [Allow], [Deny]).
    async fn assert(&self, user: User, resource: Resource, action: Action) -> Result<Effect, Error>;
}

/// Uniquely identifiable user of the system.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    /// Unique id of the [User].
    pub id: String,
    /// Email address of the [User].
    pub email: String,
}

/// Association between a set of [Users] and [Roles].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    /// Unique id of the [Group].
    pub id: String,
    /// Human readable unique name of the [Group].
    pub name: String,
    /// References to [Users] that are members of the [Group].
    pub users: Vec<String>,
    /// References to [Roles] associated with the [Group].
    pub roles: Vec<String>,
}

/// Association between one or more [Policies] (i.e. capabilities) that are related to a category of [User].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
    /// Unique id of the [Role].
    pub id: String,
    /// Human readable unique name of the [Role].
    pub name: String,
    /// References to [Policies] associated with the [Role].
    pub policies: Vec<String>,
}

/// Union of a [Resource], [Action], and [Effect]. This is effectively a permission or capability (e.g. `group::create::deny`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Policy {
    /// Unique id fo the [Policy].
    pub id: String,
    /// Human readable unique name of the [Policy].
    pub name: String,
    /// Reference to the [Resource] to which this [Policy] applies.
    pub resource_id: String,
    /// Reference to the [Action] which this [Policy] applies (e.g. Create, Read).
    pub action: Action,
    /// Reference to the [Effect] this [Policy] asserts (e.g. Allow, Deny).
    pub effect: Effect,
}

/// Resources are specific to consuming crates, however, a generic auth system needs a conventional
/// way to apply an effect to all resources for explicit deny/allow semantics. Crates that use this
/// auth system are expected to create a ResourceKind enum and map the Any member to this value.
/// # Examples
///
/// ```
/// use std::fmt::{Display, Formatter};
/// pub enum ResourceKind {
///     Any,
/// }
///
/// impl Display for ResourceKind {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///        match self {
///             ResourceKind::Any => write!(f, "{}", platform::auth::ANY_RESOURCE_KIND),
///         }
///     }
/// }
/// ```
pub const ANY_RESOURCE_KIND: &str = "*";

/// A secured asset.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resource {
    /// Unique id of the [Resource].
    pub id: String,
    /// Human readable unique name of the [Resource].
    pub name: String,
    /// Discriminator field. Useful for categorizing [Resources].
    pub kind: String,
}

/// The outcome of a [Policy] assertion.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Effect {
    /// [User] [Action] is allowed for the [Resource].
    Allow,
    /// [User] [Action] is denied for the [Resource].
    Deny
}

/// List of actions a user can perform against a [Resource].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Action {
    /// Union of all actions. Useful for deny-all or full-control policies.
    Any,
    /// Create a new [Resource].
    Create,
    /// Read access for a [Resource].
    Read,
    /// Mutate access for an existing [Resource].
    Update,
    /// Delete an existing [Resource].
    Delete,
}
