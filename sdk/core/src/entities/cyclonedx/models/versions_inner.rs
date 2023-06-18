use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VersionsInner {
    /// A single version of a component or service.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// A single version of a component or service.
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::entities::cyclonedx::models::AffectedStatus>,
}

impl VersionsInner {
    pub fn new() -> VersionsInner {
        VersionsInner {
            version: None,
            range: None,
            status: None,
        }
    }
}