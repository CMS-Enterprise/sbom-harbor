pub(crate) mod adapters;
mod service;

/// ## Overview
/// This module provides a lightweight partial OpenAPI client for the Snyk API. It is primarily used for
///
/// - Using Snyk as an SBOM Provider.
/// - Using Snyk as Enrichment Provider.
///
/// ## SBOM Provider
///
/// Example
/// ```rust
///
/// ```
///
/// ## Enrichment Provider
///
/// Example
/// ```rust
///
/// ```
/// A lightweight Snyk OpenAPI client.
pub(in crate::services::snyk) mod client;

/// Alias for the native Snyk status of a [Project].
pub type ProjectStatus = client::models::ProjectStatus;
/// Alias for a native Snyk issue. Analogous to a [Vulnerability] in Harbor.
pub type IssueSnyk = client::models::CommonIssueModel;

use crate::entities::xrefs::{Xref, XrefKind};
use serde::{Deserialize, Serialize};
pub use service::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// The version of the Snyk API referenced at build time.
pub const API_VERSION: &str = "2023-03-08~beta"; // "2023-03-29"

/// Constant used to represent Snyk in string text.
pub const SNYK_DISCRIMINATOR: &str = "snyk";

// TODO: Lazy Static or OnceCell this.
/// List of package managers that the Snyk API supports generating SBOMs from.
pub const SUPPORTED_SBOM_PROJECT_TYPES: &[&str] = &[
    "npm",
    "swift",
    "maven",
    "cocoapods",
    "composer",
    "gem",
    "nuget",
    "pypi",
    "hex",
    "cargo",
    "generic",
];

/// Set of formats that the Snyk API supports generating an [Sbom] in.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SbomFormat {
    /// CycloneDX JSON format.
    CycloneDxJson,
    /// CycloneDX XML format.
    CycloneDxXml,
    /// Spdx JSON format.
    SpdxJson,
}

impl Display for SbomFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SbomFormat::CycloneDxJson => write!(f, "cyclonedx+json"),
            SbomFormat::CycloneDxXml => write!(f, "cyclonedx+xml"),
            SbomFormat::SpdxJson => write!(f, "spdx2.3+json"),
        }
    }
}

/// Typed Xref for entities created by interacting with the Snyk API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnykRef {
    /// Organization ID that relates to the entity.
    pub org_id: String,
    /// Organization name that relates to the entity.
    pub org_name: String,
    /// Group ID that relates to the entity.
    pub group_id: String,
    /// Group name that relates to the entity.
    pub group_name: String,
    /// Project ID that relates to the entity.
    pub project_id: String,
    /// Project name that relates to the entity.
    pub project_name: String,
}

impl From<SnykRef> for Xref {
    fn from(snyk_ref: SnykRef) -> Self {
        Xref {
            kind: XrefKind::External(SNYK_DISCRIMINATOR.to_string()),
            map: HashMap::from([
                ("groupId".to_string(), snyk_ref.group_id.clone()),
                ("groupName".to_string(), snyk_ref.group_name.clone()),
                ("orgId".to_string(), snyk_ref.org_id.clone()),
                ("orgName".to_string(), snyk_ref.org_name.clone()),
                ("projectId".to_string(), snyk_ref.project_id.clone()),
                ("projectName".to_string(), snyk_ref.project_name),
            ]),
        }
    }
}

impl SnykRef {
    /// Compares two [SnykRef] instances for ID equality.
    pub fn id_eq(&self, xref: &SnykRef) -> bool {
        self.org_id == xref.org_id
            && self.group_id == xref.group_id
            && self.project_id == xref.project_id
    }
}
