use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Bom {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub dollar_schema: Option<DollarSchema>,
    /// Specifies the format of the BOM. This helps to identify the file as CycloneDX since BOMs do not have a filename convention nor does JSON schema support namespaces. This value MUST be \"CycloneDX\".
    #[serde(rename = "bomFormat")]
    pub bom_format: BomFormat,
    /// The version of the CycloneDX specification a BOM conforms to (starting at version 1.2).
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    /// Every BOM generated SHOULD have a unique serial number, even if the contents of the BOM have not changed over time. If specified, the serial number MUST conform to RFC-4122. Use of serial numbers are RECOMMENDED.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Whenever an existing BOM is modified, either manually or through automated processes, the version of the BOM SHOULD be incremented by 1. When a system is presented with multiple BOMs with identical serial numbers, the system SHOULD use the most recent version of the BOM. The default version is '1'.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::entities::cyclonedx::Metadata>>,
    /// A list of software and hardware components.
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::entities::cyclonedx::models::Component>>,
    /// A list of services. This may include microservices, function-as-a-service, and other types of network or intra-process services.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::entities::cyclonedx::models::Service>>,
    /// External references provide a way to document systems, sites, and information that may be relevant but which are not included with the BOM.
    #[serde(rename = "externalReferences", skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<crate::entities::cyclonedx::models::ExternalReference>>,
    /// Provides the ability to document dependency relationships.
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::entities::cyclonedx::models::Dependency>>,
    /// Compositions describe constituent parts (including components, services, and dependency relationships) and their completeness.
    #[serde(rename = "compositions", skip_serializing_if = "Option::is_none")]
    pub compositions: Option<Vec<crate::entities::cyclonedx::models::Compositions>>,
    /// Vulnerabilities identified in components or services.
    #[serde(rename = "vulnerabilities", skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<crate::entities::cyclonedx::models::Vulnerability>>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<serde_json::Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DollarSchema {
    #[serde(rename = "http://cyclonedx.org/schema/bom-1.4.schema.json")]
    Version1_4,
}

impl Default for DollarSchema {
    fn default() -> DollarSchema {
        Self::Version1_4
    }
}
/// Specifies the format of the BOM. This helps to identify the file as CycloneDX since BOMs do not have a filename convention nor does JSON schema support namespaces. This value MUST be \"CycloneDX\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BomFormat {
    #[serde(rename = "CycloneDX")]
    CycloneDx,
}

impl Default for BomFormat {
    fn default() -> BomFormat {
        Self::CycloneDx
    }
}
