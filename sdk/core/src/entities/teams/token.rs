use crate::Error;
use chrono::{DateTime, Utc};
use platform::cryptography::argon2;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

/// A Token is an entity that represents a string used to authorize sending
/// SBOMs into the system.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Token {
    /// The unique identifier for the Token.
    pub id: String,

    /// The name of the token.
    pub name: String,

    /// The hashed token secret value.
    pub hash: String,

    /// Flag indicating whether the token is enabled.
    pub enabled: bool,

    // TODO: Change to a timestamp.
    /// The string representation of the expiration date of the token.
    pub expires: String,

    /// The unique identifier for the [Team] associated with the token.
    pub team_id: Option<String>,

    /// The unique identifier for the [Vendor] associated with the token.
    pub vendor_id: Option<String>,
}

impl Token {
    /// Constructor function for creating new [Token] instances.
    pub fn new(
        name: String,
        expires: String,
        enabled: bool,
        team_id: Option<String>,
        vendor_id: Option<String>,
    ) -> Result<Token, Error> {
        if name.is_empty() {
            return Err(Error::Entity("token name required".to_string()));
        }

        if name.len() < 2 {
            return Err(Error::Entity(
                "token name must be at least 2 characters in length".to_string(),
            ));
        }

        match (team_id.clone(), vendor_id.clone()) {
            (None, None) | (Some(_), Some(_)) => {
                return Err(Error::Entity(
                    "token can only belong to 1 entity".to_string(),
                ));
            }
            _ => {}
        };

        match DateTime::parse_from_rfc3339(expires.as_str()) {
            Ok(expiry) => {
                if Utc::now() <= expiry {
                    return Err(Error::Entity(
                        "expiration must be in the future".to_string(),
                    ));
                }
            }
            Err(e) => {
                return Err(Error::InvalidFormat(format!(
                    "error parsing token expires: {}",
                    e
                )));
            }
        }

        let token = Uuid::new_v4().to_string();
        let hash = argon2::hash_string(token.as_str()).map_err(Error::from)?;

        Ok(Token {
            id: "".to_string(),
            name,
            hash,
            enabled,
            expires,
            team_id,
            vendor_id,
        })
    }

    /// Determines whether a token is expired.
    #[allow(dead_code)]
    pub(crate) fn expired(&self) -> Result<bool, Error> {
        if self.expires.is_empty() {
            return Ok(true);
        }

        match DateTime::parse_from_rfc3339(self.expires.as_str()) {
            Ok(expiry) => Ok(Utc::now() >= expiry),
            Err(e) => Err(Error::InvalidFormat(format!(
                "error parsing token expires: {}",
                e
            ))),
        }
    }

    /// Verifies whether the hashed value for this token matches the input value.
    #[allow(dead_code)]
    pub(crate) fn verify(&self, input: &str) -> Result<(), Error> {
        argon2::verify(self.hash.as_str(), input).map_err(Error::from)
    }
}
