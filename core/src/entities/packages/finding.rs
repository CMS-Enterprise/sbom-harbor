use crate::entities::cyclonedx::{Dependency, Issue};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::snyk::IssueSnyk;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::Display;

/// Identified security issue for a [Package].
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Finding {
    /// Indicates which enrichment provider reported the vulnerability.
    pub provider: FindingProviderKind,

    /// The Package URL for the finding.
    pub purl: Option<String>,

    /// Encapsulates CycloneDx specific model.
    pub cdx: Option<IssueCdx>,

    // TODO: Hard-coding an external dependency like this is a code-smell.
    /// Encapsulates Snyk specific model.
    pub snyk_issue: Option<IssueSnyk>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,
}

impl Finding {
    /// Compares the current finding with another to determine if they are functionally equal.
    /// Not an instance equality comparator.
    pub fn eq(&self, other: &Finding) -> bool {
        self.purl.eq(&other.purl) && self.provider == other.provider && self.xrefs.eq(&other.xrefs)
    }
}

/// Discriminator used to indicate what enrichment source identified a [Finding]. Implementers
/// that want to develop their own enrichment sources and don't intend to contribute them back upstream can
/// use the Custom variant without having to hard fork.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FindingProviderKind {
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

    /// The CycloneDx native model for the Issue.
    pub inner: Issue,
}

impl IssueCdx {
    fn from(purl: String, issue: Issue) -> Self {
        IssueCdx { purl, inner: issue }
    }
}
