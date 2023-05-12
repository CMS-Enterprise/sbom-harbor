use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::entities::cyclonedx::Component;
use crate::entities::enrichments::Vulnerability;
use crate::entities::tasks::{Task, TaskRef};
use crate::entities::xrefs::Xref;
use crate::Error;

/// Purl is a derived type that facilitates analysis of a Package across the entire enterprise.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Purl {
    /// Unique identifier for the Package URL.
    pub id: String,

    /// The package manager for the [Purl].
    pub package_manager: Option<String>,

    /// The raw Package URL.
    pub purl: String,

    /// The package name.
    pub name: String,

    /// The package version.
    pub version: Option<String>,

    /// Source of the Purl.
    pub component_kind: ComponentKind,

    /// Reference to each [Task] that was performed against this [Purl].
    pub task_refs: Vec<TaskRef>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,

    /// The list of vulnerability associated with this Purl.
    pub vulnerabilities: Option<Vec<Vulnerability>>,
}

impl Purl {
    #[allow(dead_code)]
    pub(crate) fn decode(purl: &str) -> Result<String, Error> {
        let result = platform::encoding::url_decode(purl)
            .map_err(|e| Error::Entity(format!("purl::decode::{}", e)))?;
        Ok(result)
    }

    /// Generates a path safe file name from a Package URL.
    pub(crate) fn format_file_name(purl: &str) -> String {
        let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
        re.replace_all(purl, "-").to_string()
    }

    pub(crate) fn from_component(
        component: &Component,
        component_kind: ComponentKind,
        task: &Task,
        iteration: u32,
        xref: Xref,
    ) -> Result<Self, Error> {
        let purl = match &component.purl {
            None => {
                return Err(Error::Entity("component_purl_none".to_string()));
            }
            Some(p) => p,
        };

        let task_ref = TaskRef::new(task, purl.clone(), iteration);

        Ok(Self {
            id: "".to_string(),
            package_manager: None,
            purl: purl.clone(),
            name: component.name.clone(),
            version: component.version.clone(),
            component_kind,
            task_refs: vec![task_ref],
            vulnerabilities: None,
            xrefs: vec![xref],
        })
    }

    /// Sets up a reference between the [Purl] and the [Task].
    pub fn init_scan(&mut self, task: &Task) -> Result<TaskRef, Error> {
        if task.id.is_empty() {
            return Err(Error::Entity("task_id_required".to_string()));
        }

        let mut task_ref = TaskRef::new(task, self.purl.clone(), 0);

        task_ref.iteration = match self.task_refs.iter().max_by_key(|s| s.iteration) {
            Some(s) => s.iteration + 1,
            _ => 1,
        };

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

    /// Appends Vulnerabilities to the Purl.
    pub fn vulnerabilities(&mut self, new: &Vec<Vulnerability>) {
        if new.is_empty() {
            return;
        }

        let mut current = match &self.vulnerabilities {
            None => {
                self.vulnerabilities = Some(new.clone());
                return;
            }
            Some(existing) => existing.clone(),
        };

        if current.is_empty() {
            self.vulnerabilities = Some(new.clone());
            return;
        }

        for new_vulnerability in new.iter() {
            match current.iter().any(|existing| {
                existing.cve == new_vulnerability.cve
                    && existing.provider == new_vulnerability.provider
            }) {
                true => {}
                false => {
                    current.push(new_vulnerability.clone());
                }
            }
        }

        self.vulnerabilities = Some(current);
    }
}

/// Discriminator that indicates whether the Purl was extracted from a [Package] or a [Dependency].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ComponentKind {
    /// The Purl was extracted from a Package.
    Package,
    /// The Purl was extracted from a Dependency.
    Dependency,
}

impl ToString for ComponentKind {
    fn to_string(&self) -> String {
        match self {
            ComponentKind::Package => "package".to_string(),
            ComponentKind::Dependency => "dependency".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::enrichments::{Vulnerability, VulnerabilityProviderKind};
    use crate::entities::packages::{ComponentKind, Purl};
    use crate::Error;

    #[async_std::test]
    #[ignore = "used to load projects from Snyk to local Mongo for debugging"]
    async fn can_prevent_duplicate_issues_from_snyk() -> Result<(), Error> {
        struct TestCase {
            expected_count: usize,
            existing: Purl,
            new: Purl,
        }

        let test_cases = vec![
            // Unique by CVE & Provider
            TestCase {
                expected_count: 1,
                existing: Purl {
                    id: "".to_string(),
                    package_manager: None,
                    purl: "pkg:npm/xml2js@0.4.19".to_string(), // Known to have issues
                    name: "".to_string(),
                    version: None,
                    component_kind: ComponentKind::Package,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: Some(vec![Vulnerability {
                        provider: VulnerabilityProviderKind::Snyk,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                    }]),
                },
                new: Purl {
                    id: "".to_string(),
                    package_manager: None,
                    purl: "pkg:npm/xml2js@0.4.19".to_string(), // Known to have issues
                    name: "".to_string(),
                    version: None,
                    component_kind: ComponentKind::Package,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: Some(vec![Vulnerability {
                        provider: VulnerabilityProviderKind::Snyk,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                    }]),
                },
            },
            // Multiple by CVE with different provider
            TestCase {
                expected_count: 2,
                existing: Purl {
                    id: "".to_string(),
                    package_manager: None,
                    purl: "pkg:npm/xml2js@0.4.19".to_string(), // Known to have issues
                    name: "".to_string(),
                    version: None,
                    component_kind: ComponentKind::Package,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: Some(vec![Vulnerability {
                        provider: VulnerabilityProviderKind::Snyk,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                    }]),
                },
                new: Purl {
                    id: "".to_string(),
                    package_manager: None,
                    purl: "pkg:npm/xml2js@0.4.19".to_string(), // Known to have issues
                    name: "".to_string(),
                    version: None,
                    component_kind: ComponentKind::Package,
                    task_refs: vec![],
                    xrefs: vec![],
                    vulnerabilities: Some(vec![Vulnerability {
                        provider: VulnerabilityProviderKind::IonChannel,
                        severity: None,
                        cve: Some("CVE-2023-0842".to_string()),
                        description: None,
                        cvss: None,
                        cwes: None,
                        remediation: None,
                        raw: None,
                    }]),
                },
            },
        ];

        for mut case in test_cases {
            case.new
                .vulnerabilities(&case.existing.vulnerabilities.unwrap());
            assert_eq!(case.expected_count, case.new.vulnerabilities.unwrap().len());
        }

        Ok(())
    }
}
