use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::{Discriminator, Team};
use crate::Error;

/// A Token is an entity that represents a string used to authorize sending
/// SBOMs into the system.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Token {
    /// The DynamoDB partition key for the item entry.
    #[serde(rename = "TeamId")]
    pub partition_key: String,
    #[serde(rename = "EntityKey")]
    /// The DynamoDB sort key for the item entry.
    pub sort_key: String,
    /// The id of the Team that owns the Token.
    #[serde(rename = "parentId")]
    pub parent_id: String,

    /// The unique identifier for the Token.
    pub id: String,
    /// The name of the token.
    pub name: String,
    /// The secret token value.
    pub token: String,
    /// Flag indicating whether the token is enabled.
    pub enabled: bool,
    /// The string representation of the expiration date of the token.
    pub expires: String,
}

impl Token {
    pub fn new(parent: Team, name: String, expires: String, enabled: Option<bool>) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            partition_key: parent.partition_key.clone(),
            sort_key: Discriminator::Token.to_sort_key(&id).unwrap(),
            parent_id: parent.id.clone(),
            id,
            name,
            token: Uuid::new_v4().to_string(),
            enabled: enabled.unwrap_or(false),
            expires,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn expired(&self) -> Result<bool, Error> {
        if self.expires.is_empty() {
            return Ok(false);
        }

        match DateTime::parse_from_rfc3339(&self.expires) {
            Ok(expiry) => Ok(Utc::now() >= expiry),
            Err(err) => Err(Error::Parse(format!("error parsing token expires: {}", err.to_string()))),
        }
    }
}
