use crate::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A Token is an entity that represents a string used to authorize sending
/// SBOMs into the system.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
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
    /// Constructor function for creating new [Token] instances.
    pub fn new(name: String, expires: String, enabled: Option<bool>) -> Self {
        Self {
            id: "".to_string(),
            name,
            token: Uuid::new_v4().to_string(),
            enabled: enabled.unwrap_or(false),
            expires,
        }
    }

    /// Determines whether a token is expired.
    #[allow(dead_code)]
    pub(crate) fn expired(&self) -> Result<bool, Error> {
        if self.expires.is_empty() {
            return Ok(false);
        }

        match DateTime::parse_from_rfc3339(&self.expires) {
            Ok(expiry) => Ok(Utc::now() >= expiry),
            Err(e) => Err(Error::InvalidFormat(format!(
                "error parsing token expires: {}",
                e
            ))),
        }
    }
}
