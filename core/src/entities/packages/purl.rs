use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::entities::cyclonedx::Component;
use crate::entities::packages::{Finding, FindingProviderKind};
use crate::entities::scans::{Scan, ScanRef};
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

    /// Reference to each [Scan] that was performed against this [Purl].
    pub scan_refs: Vec<ScanRef>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,

    /// The list of vulnerability findings associated with this Purl.
    pub findings: Option<Vec<Finding>>,
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
        scan: &Scan,
        iteration: u32,
        xref: Xref,
    ) -> Result<Self, Error> {
        let purl = match &component.purl {
            None => {
                return Err(Error::Entity("component_purl_none".to_string()));
            }
            Some(p) => p,
        };

        let scan_ref = ScanRef::new(scan, purl.clone(), iteration);

        Ok(Self {
            id: "".to_string(),
            package_manager: None,
            purl: purl.clone(),
            name: component.name.clone(),
            version: component.version.clone(),
            component_kind,
            scan_refs: vec![scan_ref],
            findings: None,
            xrefs: vec![xref],
        })
    }

    /// Sets up a reference between the [Purl] and the [Scan]. This is has been renamed to
    /// `join_scan` in the concurrency branch.
    pub fn init_scan(&mut self, scan: &Scan) -> Result<ScanRef, Error> {
        if scan.id.is_empty() {
            return Err(Error::Entity("scan_id_required".to_string()));
        }

        let mut scan_ref = ScanRef::new(scan, self.purl.clone(), 0);

        scan_ref.iteration = match self.scan_refs.iter().max_by_key(|s| s.iteration) {
            Some(s) => s.iteration + 1,
            _ => 1,
        };

        let result = scan_ref.clone();
        self.scan_refs.push(scan_ref);

        Ok(result)
    }

    /// Log an error ta the [ScanRef] that matches the [Scan].
    pub fn scan_err(&mut self, scan: &Scan, err: Option<String>) -> Result<(), Error> {
        return match self.scan_refs.iter_mut().find(|e| e.scan_id == scan.id) {
            None => Err(Error::Entity("scan_ref_none".to_string())),
            Some(scan_ref) => {
                scan_ref.err = err;
                Ok(())
            }
        };
    }

    /// Add a [ScanRef] to the [Purl].
    pub fn scan_refs(&mut self, scan_ref: &ScanRef) {
        if !self.scan_refs.iter().any(|s| s.scan_id == scan_ref.scan_id) {
            self.scan_refs.push(scan_ref.clone());
        }
    }

    /// Appends Findings to the Purl.
    pub fn findings(&mut self, new: &Vec<Finding>) {
        if new.is_empty() {
            return;
        }

        let mut current = match &self.findings {
            None => {
                self.findings = Some(new.clone());
                return;
            }
            Some(existing) => existing.clone(),
        };

        if current.is_empty() {
            self.findings = Some(new.clone());
            return;
        }

        for new_finding in new {
            match new_finding.provider {
                FindingProviderKind::DependencyTrack => {}
                FindingProviderKind::IonChannel => {}
                FindingProviderKind::Custom(_) => {}
                FindingProviderKind::Snyk => {
                    // TODO: Inlining this is a code smell and couples the domain to providers.
                    // This needs to ultimately be handled differently and is confirmation that
                    // putting the snyk_issue ref here was a bad design.
                    if !current.iter().any(|existing_finding| {
                        if existing_finding.provider != FindingProviderKind::Snyk {
                            return false;
                        }

                        if let Some(existing_issue) = &existing_finding.snyk_issue {
                            if let Some(existing_issue_id) = &existing_issue.id {
                                if let Some(new_issue) = &new_finding.snyk_issue {
                                    if let Some(new_issue_id) = &new_issue.id {
                                        return existing_issue_id.eq(new_issue_id);
                                    }
                                }
                            }
                        }

                        false
                    }) {
                        current.push(new_finding.clone());
                    }
                }
            }
        }

        self.findings = Some(current);
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
    use crate::entities::packages::{ComponentKind, Finding, FindingProviderKind, Purl};
    use crate::services::snyk::IssueSnyk;
    use crate::Error;

    #[async_std::test]
    #[ignore = "used to load projects from Snyk to local Mongo for debugging"]
    async fn can_prevent_duplicate_issues_from_snyk() -> Result<(), Error> {
        struct TestCase {
            existing: Purl,
        }

        let test_cases = vec![
            // Snyk Case
            TestCase {
                existing: Purl {
                    id: "".to_string(),
                    package_manager: None,
                    purl: "pkg:npm/xml2js@0.4.19".to_string(), // Known to have issues
                    name: "".to_string(),
                    version: None,
                    component_kind: ComponentKind::Package,
                    scan_refs: vec![],
                    xrefs: vec![],
                    findings: Some(vec![Finding {
                        provider: FindingProviderKind::Snyk,
                        purl: None,
                        cdx: None,
                        snyk_issue: Some(IssueSnyk {
                            attributes: None,
                            id: Some("duplicate_id".to_string()),
                            r#type: None,
                        }),
                        xrefs: vec![],
                    }]),
                },
            },
            // TODO: Add more as other providers are added.
        ];

        for case in test_cases {
            let mut new = case.existing.clone();
            new.findings(&case.existing.findings.unwrap());
            assert_eq!(1, new.findings.unwrap().len());
        }

        Ok(())
    }
}
