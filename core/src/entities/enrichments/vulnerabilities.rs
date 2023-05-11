use crate::entities::enrichments::cvss::Summary;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};

///
pub type Severity = crate::entities::cyclonedx::Severity;

/// Identified security issue for a [Package].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Vulnerability {
    /// Indicates which enrichment provider reported the vulnerability.
    pub provider: VulnerabilityProviderKind,

    /// Indicates the severity of the [Vulnerability].
    pub severity: Option<Severity>,

    /// The CVE ID of the [Vulnerability].
    pub cve: Option<String>,

    /// The CVE description of the [Vulnerability].
    pub description: Option<String>,

    /// Optional CVSS Detail from the enrichment provider.
    pub cvss: Option<Summary>,

    /// Optional list of identified CWEs for the Vulnerability.
    pub cwes: Option<Vec<Cwe>>,

    /// Optional advice from the enrichment provider on how to mitigate the [Vulnerability].
    pub remediation: Option<Remediation>,

    /// Stores the original enrichment provider raw result.
    pub raw: Option<String>,
}

impl Vulnerability {}

/// Discriminator used to indicate what enrichment provider identified a [Vulnerability]. Implementers
/// that want to develop their own enrichment sources and don't intend to contribute them back upstream can
/// use the Custom variant without having to hard fork.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VulnerabilityProviderKind {
    /// Dependency Track provider.
    DependencyTrack,
    /// Ion Channel provider.
    IonChannel,
    /// Snyk provider.
    Snyk,
    /// Custom provider.
    Custom(String),
}

impl Display for VulnerabilityProviderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VulnerabilityProviderKind::DependencyTrack => write!(f, "dependency-track"),
            VulnerabilityProviderKind::IonChannel => write!(f, "ion-channel"),
            VulnerabilityProviderKind::Snyk => write!(f, "snyk"),
            VulnerabilityProviderKind::Custom(name) => write!(f, "custom-{}", name),
        }
    }
}

/// From [cwe.mitre.org](https://cwe.mitre.org): "CWE™ is a community-developed list of
/// software and hardware weakness types. It serves as a common language, a measuring stick for
/// security tools, and as a baseline for weakness identification, mitigation, and prevention efforts.
///
/// This model is based on the JSON schema specified by [OpenCVE](https://docs.opencve.io/api/cwe/).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Cwe {
    /// The CWE identifier.
    pub id: String,
    /// The summary description.
    pub name: Option<String>,
    /// The extended description.
    pub description: Option<String>,
}

/// Advice from the enrichment provider on how to mitigate the [Vulnerability].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Remediation {
    /// Advice on how to mitigate the [Vulnerability].
    pub description: String,
}
