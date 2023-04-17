use crate::entities::cyclonedx::Issue;
use crate::entities::xrefs::{Xref, XrefKind};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::Display;

/// Identified security issue for a [Package].
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Finding {
    /// Unique identifier for the Finding.
    pub id: String,

    /// Indicates which enrichment source reported the vulnerability.
    pub source: FindingSource,

    /// The Package URL for the finding.
    pub purl: Option<String>,

    /// Encapsulates CycloneDx specific attributes.
    pub cdx: Option<IssueCdx>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Option<HashMap<XrefKind, Xref>>,
}

impl Finding {
    /// Compares the current finding with another to determine if they are functionally equal.
    /// Not an instance equality comparator.
    pub fn eq(&self, other: &Finding) -> bool {
        self.purl.eq(&other.purl) && self.source == other.source && self.xrefs.eq(&other.xrefs)
    }
}

/// Discriminator used to indicate what enrichment source identified a [Finding]. Implementers
/// that want to develop their own enrichment sources and don't intend to contribute them back upstream can
/// use the Custom variant without having to hard fork.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FindingSource {
    DependencyTrack,
    IonChannel,
    Snyk,
    Custom(String),
}

/// A subset of the full CycloneDx Issue suitable for tracking findings.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssueCdx {
    /// The denormalized raw Package URL from the CdxComponent.
    pub purl: String,

    pub issue: Issue,
}

impl IssueCdx {
    fn from(purl: String, issue: Issue) -> Self {
        IssueCdx { purl, issue }
    }
}
