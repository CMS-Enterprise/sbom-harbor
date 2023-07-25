use harbcore::entities::teams::Token;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Error;

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct TokenInsert {
    /// The name of the token.
    pub name: Option<String>,

    /// Flag indicating whether the token is enabled.
    pub enabled: Option<bool>,

    // TODO: Change to a timestamp.
    /// The string representation of the expiration date of the token.
    pub expires: Option<String>,

    /// The unique identifier for the [Team] associated with the token.
    pub team_id: Option<String>,

    /// The unique identifier for the [Vendor] associated with the token.
    pub vendor_id: Option<String>,
}

impl TokenInsert {
    /// Validates insert type and converts to entity.
    pub fn to_entity(&self) -> Result<Token, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::InvalidParameters("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        let expires = match &self.expires {
            None => {
                return Err(Error::InvalidParameters("expires required".to_string()));
            }
            Some(expires) => expires.clone(),
        };

        Token::new(
            name,
            expires,
            self.enabled.unwrap_or(false),
            self.team_id.clone(),
            self.vendor_id.clone(),
        )
        .map_err(|e| Error::InvalidParameters(e.to_string()))
    }
}
