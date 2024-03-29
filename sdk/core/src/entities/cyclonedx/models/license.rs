use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::entities::cyclonedx::models::SpdxPeriodSchema>,
    /// If SPDX does not define the license used, this field may be used to provide the license name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<crate::entities::cyclonedx::models::Attachment>>,
    /// The URL to the license file. If specified, a 'license' externalReference should also be specified for completeness
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl License {
    pub fn new() -> License {
        License {
            id: None,
            name: None,
            text: None,
            url: None,
        }
    }
}
