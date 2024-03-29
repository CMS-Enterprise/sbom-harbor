use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// The date and time (timestamp) when the BOM was created.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The tool(s) used in the creation of the BOM.
    #[serde(rename = "tools", skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<crate::entities::cyclonedx::models::Tool>>,
    /// The person(s) who created the BOM. Authors are common in BOMs created through manual processes. BOMs created through automated means may not have authors.
    #[serde(rename = "authors", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<crate::entities::cyclonedx::models::OrganizationalContact>>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<Box<crate::entities::cyclonedx::models::Component>>,
    #[serde(rename = "manufacture", skip_serializing_if = "Option::is_none")]
    pub manufacture: Option<Box<crate::entities::cyclonedx::models::OrganizationalEntity>>,
    #[serde(rename = "supplier", skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Box<crate::entities::cyclonedx::models::OrganizationalEntity>>,
    #[serde(rename = "licenses", skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<crate::entities::cyclonedx::models::LicenseChoice>>,
    /// Provides the ability to document properties in a name-value store. This provides flexibility to include data not officially supported in the standard without having to use additional namespaces or create extensions. Unlike key-value stores, properties support duplicate names, each potentially having different values. Property names of interest to the general public are encouraged to be registered in the [CycloneDX Property Taxonomy](https://github.com/CycloneDX/cyclonedx-property-taxonomy). Formal registration is OPTIONAL.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::entities::cyclonedx::models::Property>>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            timestamp: None,
            tools: None,
            authors: None,
            component: None,
            manufacture: None,
            supplier: None,
            licenses: None,
            properties: None,
        }
    }
}
