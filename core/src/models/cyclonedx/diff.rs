use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

/// Diff : The patch file (or diff) that show changes. Refer to https://en.wikipedia.org/wiki/Diff
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Diff {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<crate::models::cyclonedx::Attachment>>,
    /// Specifies the URL to the diff
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Diff {
    /// The patch file (or diff) that show changes. Refer to https://en.wikipedia.org/wiki/Diff
    pub fn new() -> Diff {
        Diff {
            text: None,
            url: None,
        }
    }
}

