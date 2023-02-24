use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{Error, models};

/// A Token is an entity that represents a string used to authorize sending
/// SBOMs into the system.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Token {
    /// The unique identifier for the Token.
    #[serde(rename = "_id")]
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
    pub fn new(name: String, expires: String, enabled: Option<bool>) -> Self {
        Self {
            id: "".to_string(),
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
            Err(err) => Err(Error::Format(format!("error parsing token expires: {}", err.to_string()))),
        }
    }
}

impl From<models::Token> for Token {
    fn from(value: models::Token) -> Self {
        Self{
            id: value.id,
            name: value.name,
            token: value.token,
            enabled: value.enabled,
            expires: value.expires,
        }
    }
}
