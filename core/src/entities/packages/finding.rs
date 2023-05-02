use crate::entities::cyclonedx::Issue;
use crate::entities::xrefs::Xref;
use crate::services::snyk::IssueSnyk;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Identified security issue for a [Package].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
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
    pub fn functionally_eq(&self, other: &Finding) -> bool {
        self.purl.eq(&other.purl) && self.provider == other.provider && self.xrefs.eq(&other.xrefs)
    }
}

/// Discriminator used to indicate what enrichment provider identified a [Finding]. Implementers
/// that want to develop their own enrichment sources and don't intend to contribute them back upstream can
/// use the Custom variant without having to hard fork.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FindingProviderKind {
    /// Dependency Track provider.
    DependencyTrack,
    /// Ion Channel provider.
    IonChannel,
    /// Snyk provider.
    Snyk,
    /// Custom provider.
    Custom(String),
}

/// A subset of the full CycloneDx Issue suitable for tracking findings.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCdx {
    /// The denormalized raw Package URL from the CdxComponent.
    pub purl: String,

    /// The CycloneDx native model for the Issue.
    pub inner: Issue,
}
