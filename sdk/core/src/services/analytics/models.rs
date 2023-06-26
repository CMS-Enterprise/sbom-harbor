use crate::entities::enrichments::cvss::Cvss;
use crate::entities::enrichments::{Cwe, Severity, Vulnerability, VulnerabilityProviderKind};
use crate::entities::packages::Package;
use crate::entities::sboms::{Author, Sbom, SbomProviderKind};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Subset of the full [Sbom] entity most useful for analytics and reporting.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct SbomSummary {
    /// The unique identifier for the Sbom.
    pub id: String,

    /// Designation assigned to a unit of software defined by the original supplier as specified by
    /// the NTIA SBOM Minimum Elements.
    pub name: Option<String>,

    /// Identifier used by the supplier to specify a change in software from a previously identified
    /// version as specified by the NTIA SBOM Minimum Elements.
    pub version: Option<String>,

    /// The package manager for the [Sbom].
    pub package_manager: Option<String>,

    /// The Package URL for the package that the Sbom was generated from.
    pub purl: Option<String>,

    /// The system or tool that generated the [Sbom] if known. Should satisfy the Author
    /// element as specified by the NTIA SBOM Minimum Elements.
    pub author: Author,

    /// Denormalized from Author. Allows partitioning SBOMs by the tool that was used to generate
    /// them.
    pub provider: Option<SbomProviderKind>,

    /// The name of an entity that creates, defines, and identifies the component that this [Sbom]
    /// pertains to. Part of the NTIA SBOM Minimum Elements.
    pub supplier_name: Option<String>,

    /// Denormalized list of dependency refs as specified by the NTIA SBOM Minimum Elements. Used to
    /// hydrate package summaries for the dependencies field.
    #[serde(skip)]
    pub(crate) dependency_refs: Vec<String>,

    /// List of dependencies identified in the SBOM.
    pub dependencies: Vec<PackageSummary>,
}

impl SbomSummary {
    /// Converts an entity to a summarized format for reporting.
    pub fn from_entity(source: &Sbom) -> SbomSummary {
        SbomSummary {
            id: source.id.clone(),
            name: source.component_name.clone(),
            version: source.version.clone(),
            package_manager: source.package_manager.clone(),
            purl: source.purl.clone(),
            author: source.author.clone(),
            provider: source.provider.clone(),
            supplier_name: source.supplier_name.clone(),
            dependencies: vec![],
            dependency_refs: match &source.dependency_refs {
                None => vec![],
                Some(refs) => refs.clone(),
            },
        }
    }
}

/// Subset of the full [Package] entity most useful for analytics and reporting.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct PackageSummary {
    /// The unique identifier for the Package.
    pub id: String,

    /// Optional Package URL if known.
    pub purl: Option<String>,

    /// The package version.
    pub version: Option<String>,

    /// Optional Common Platform Enumeration (CPE) identifier if known.
    pub cpe: Option<String>,

    /// Characterizing the relationship that an upstream component X is included in software Y.
    pub dependency_refs: Option<Vec<String>>,

    /// Dependencies represented as packages summaries.
    pub dependencies: Vec<PackageSummary>,

    /// Vulnerabilities associated with this [Package].
    pub vulnerabilities: Vec<VulnerabilitySummary>,
}

impl Default for PackageSummary {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            purl: None,
            version: None,
            cpe: None,
            dependency_refs: None,
            dependencies: vec![],
            vulnerabilities: vec![],
        }
    }
}

impl PackageSummary {
    /// Converts an entity to a summarized format for reporting.
    pub fn from_entity(source: &Package) -> PackageSummary {
        PackageSummary {
            id: source.id.clone(),
            purl: source.purl.clone(),
            version: source.version.clone(),
            cpe: source.cpe.clone(),
            dependency_refs: source.dependency_refs.clone(),
            dependencies: vec![],
            vulnerabilities: vec![],
        }
    }
}

/// Identified security issue for a [Package]. Subset of the full [Vulnerability] entity most useful
/// for analytics and reporting.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct VulnerabilitySummary {
    /// Unique identifier for instance.
    pub id: String,

    /// The Package URL that the [Vulnerability] pertains to.
    pub purl: String,

    /// Indicates which enrichment provider reported the vulnerability.
    pub provider: VulnerabilityProviderKind,

    /// Indicates the severity of the [Vulnerability].
    pub severity: Option<Severity>,

    /// The CVE ID of the [Vulnerability].
    pub cve: Option<String>,

    /// The CVE description of the [Vulnerability].
    pub description: Option<String>,

    /// The EPSS Score for the CVE ID.
    pub epss_score: Option<f32>,

    /// Optional CVSS Detail from the enrichment provider.
    pub cvss: Option<Cvss>,

    /// Optional list of identified CWEs for the Vulnerability.
    pub cwes: Option<Vec<Cwe>>,
}

impl Default for VulnerabilitySummary {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            purl: "".to_string(),
            provider: VulnerabilityProviderKind::Grype,
            severity: None,
            cve: None,
            description: None,
            epss_score: None,
            cvss: None,
            cwes: None,
        }
    }
}

impl VulnerabilitySummary {
    /// Converts an entity to a summarized format for reporting.
    pub fn from_entity(source: &Vulnerability) -> VulnerabilitySummary {
        Self {
            id: source.id.clone(),
            purl: source.purl.clone(),
            provider: source.provider.clone(),
            severity: source.severity,
            cve: source.cve.clone(),
            description: source.description.clone(),
            epss_score: source.epss_score,
            cvss: source.cvss.clone(),
            cwes: source.cwes.clone(),
        }
    }
}
