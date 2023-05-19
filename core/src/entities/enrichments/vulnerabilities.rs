use crate::entities::enrichments::cvss::Summary;
use crate::entities::tasks::{Task, TaskRef};
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};

/// Alias for the native CycloneDx Severity enum.
pub type Severity = crate::entities::cyclonedx::Severity;

/// Identified security issue for a [Package].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Vulnerability {
    /// Unique identifier for instance.
    pub id: String,

    /// Package URL that the [Vulnerability] pertains to.
    pub purl: String,

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

    /// Reference to each [Task] that was performed against this [Vulnerability].
    pub task_refs: Vec<TaskRef>,
}

impl Vulnerability {
    /// Sets up a reference between the [Vulnerability] and a [Task].
    pub fn join_task(&mut self, task: &Task) -> Result<TaskRef, Error> {
        if task.id.is_empty() {
            return Err(Error::Entity("task_id_required".to_string()));
        }

        let task_ref = TaskRef::new(task, self.purl.clone());

        let result = task_ref.clone();
        self.task_refs.push(task_ref);

        Ok(result)
    }

    /// Add a [TaskRef] to the [Purl].
    pub fn task_refs(&mut self, task_ref: &TaskRef) {
        if !self.task_refs.iter().any(|s| s.task_id == task_ref.task_id) {
            self.task_refs.push(task_ref.clone());
        }
    }
}

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

#[cfg(test)]
mod tests {
    use crate::entities::enrichments::{Vulnerability, VulnerabilityProviderKind};
    use crate::entities::packages::{Package, PackageKind};
    use crate::Error;

    #[async_std::test]
    #[ignore = "used to load projects from Snyk to local Mongo for debugging"]
    async fn can_prevent_duplicate_issues_from_snyk() -> Result<(), Error> {
        struct TestCase {
            expected_count: usize,
            existing: Package,
            new: Package,
        }

        let test_cases = vec![
            // Unique by CVE & Provider
            TestCase {
                expected_count: 1,
                existing: Package {
                    id: "".to_string(),
                    kind: PackageKind::Dependency,
                    package_manager: None,
                    purl: Some("pkg:npm/xml2js@0.4.19".to_string()), // Known to have issues
                    version: None,
                    cpe: None,
                    cdx: None,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: vec![Vulnerability {
                        id: "".to_string(),
                        purl: "pkg:npm/xml2js@0.4.19".to_string(),
                        provider: VulnerabilityProviderKind::Snyk,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                        task_refs: vec![],
                    }],
                    dependency_refs: None,
                    dependencies: vec![],
                },
                new: Package {
                    id: "".to_string(),
                    kind: PackageKind::Dependency,
                    package_manager: None,
                    purl: Some("pkg:npm/xml2js@0.4.19".to_string()), // Known to have issues
                    version: None,
                    cpe: None,
                    cdx: None,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: vec![Vulnerability {
                        id: "".to_string(),
                        purl: "pkg:npm/xml2js@0.4.19".to_string(),
                        provider: VulnerabilityProviderKind::Snyk,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                        task_refs: vec![],
                    }],
                    dependency_refs: None,
                    dependencies: vec![],
                },
            },
            // Multiple by CVE with different provider
            TestCase {
                expected_count: 2,
                existing: Package {
                    id: "".to_string(),
                    kind: PackageKind::Dependency,
                    package_manager: None,
                    purl: Some("pkg:npm/xml2js@0.4.19".to_string()), // Known to have issues
                    version: None,
                    cpe: None,
                    cdx: None,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: vec![Vulnerability {
                        id: "".to_string(),
                        purl: "pkg:npm/xml2js@0.4.19".to_string(),
                        provider: VulnerabilityProviderKind::Snyk,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                        task_refs: vec![],
                    }],
                    dependency_refs: None,
                    dependencies: vec![],
                },
                new: Package {
                    id: "".to_string(),
                    kind: PackageKind::Dependency,
                    package_manager: None,
                    purl: Some("pkg:npm/xml2js@0.4.19".to_string()), // Known to have issues
                    version: None,
                    cpe: None,
                    cdx: None,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: vec![Vulnerability {
                        id: "".to_string(),
                        purl: "pkg:npm/xml2js@0.4.19".to_string(),
                        provider: VulnerabilityProviderKind::IonChannel,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                        task_refs: vec![],
                    }],
                    dependency_refs: None,
                    dependencies: vec![],
                },
            },
        ];

        for mut case in test_cases {
            case.new.vulnerabilities(&case.existing.vulnerabilities[0]);
            assert_eq!(case.expected_count, case.new.vulnerabilities.len());
        }

        Ok(())
    }
}
