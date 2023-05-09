use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

/// IdentifiableAction : Specifies an individual commit
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentifiableAction {
    /// The timestamp in which the action occurred
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The name of the individual who performed the action
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email address of the individual who performed the action
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl IdentifiableAction {
    /// Specifies an individual commit
    pub fn new() -> IdentifiableAction {
        IdentifiableAction {
            timestamp: None,
            name: None,
            email: None,
        }
    }
}
