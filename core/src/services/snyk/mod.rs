pub(crate) mod adapters;
mod service;

pub(in crate::services::snyk) mod client;
pub(in crate::services::snyk) use client::*;

pub type ProjectStatus = models::ProjectStatus;
pub type IssueSnyk = models::CommonIssueModel;

use crate::entities::xrefs::{Xref, XrefKind};
use crate::Error;
use serde::{Deserialize, Serialize};
pub use service::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub const API_VERSION: &str = "2023-03-08~beta"; // "2023-03-29"
pub const SNYK_DISCRIMINATOR: &str = "snyk";
// TODO: Lazy Static or OnceCell this.
pub const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SbomFormat {
    CycloneDxJson,
    CycloneDxXml,
    SpdxJson,
}

impl Display for SbomFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SbomFormat::CycloneDxJson => write!(f, "{}", "cyclonedx+json"),
            SbomFormat::CycloneDxXml => write!(f, "{}", "cyclonedx+xml"),
            SbomFormat::SpdxJson => write!(f, "{}", "spdx2.3+json"),
        }
    }
}

#[allow(missing_docs)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnykRef {
    pub org_id: String,
    pub org_name: String,
    pub group_id: String,
    pub group_name: String,
    pub project_id: String,
    pub project_name: String,
}

impl From<SnykRef> for Xref {
    fn from(snyk_ref: SnykRef) -> Self {
        Xref {
            kind: XrefKind::External(SNYK_DISCRIMINATOR.to_string()),
            map: HashMap::from([
                ("group_id".to_string(), snyk_ref.group_id.clone()),
                ("group_name".to_string(), snyk_ref.group_name.clone()),
                ("org_id".to_string(), snyk_ref.org_id.clone()),
                ("org_name".to_string(), snyk_ref.org_name.clone()),
                ("project_id".to_string(), snyk_ref.project_id.clone()),
                ("project_name".to_string(), snyk_ref.project_name.clone()),
            ]),
        }
    }
}

impl SnykRef {
    pub fn eq(&self, xref: &SnykRef) -> bool {
        self.org_id == xref.org_id
            && self.group_id == xref.group_id
            && self.project_id == xref.project_id
    }

    pub fn from(xrefs: &Option<HashMap<XrefKind, Xref>>) -> Option<&Xref> {
        match xrefs {
            None => None,
            Some(xrefs) => xrefs.get(&XrefKind::External(SNYK_DISCRIMINATOR.to_string())),
        }
    }
}
