use crate::entities::enrichments::cvss::{Cvss as HarborCvss, Score, Version};
use crate::entities::enrichments::{
    Remediation, Severity, Vulnerability as HarborVulnerability, VulnerabilityProviderKind,
};
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;

const NVD: &str = "nvd:cpe";

/// Match is a single item for the JSON array reported
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Match {
    pub vulnerability: Option<Vulnerability>,
    pub related_vulnerabilities: Option<Vec<VulnerabilityMetadata>>,
    pub match_details: Option<Vec<MatchDetails>>,
    pub artifact: Option<Package>,
}

impl Match {
    /// Converts a Grype Vulnerability model to the Harbor model.
    pub fn to_vulnerability(&self) -> Result<HarborVulnerability, Error> {
        let purl = match &self.purl() {
            None => {
                return Err(Error::Entity("grype_package_none".to_string()));
            }
            Some(purl) => purl.to_string(),
        };

        let raw = match serde_json::to_string(&self).map_err(|e| Error::Serde(e.to_string())) {
            Ok(raw) => Some(raw),
            Err(_) => None,
        };

        // Returning an option here is an ergonomic decision.
        Ok(HarborVulnerability {
            id: "".to_string(),
            purl,
            provider: VulnerabilityProviderKind::Grype,
            severity: self.severity(),
            cve: self.cve(),
            description: self.description(),
            epss_score: None,
            cvss: self.cvss(),
            cwes: None,
            remediation: self.remediation(),
            raw,
            task_refs: vec![],
        })
    }

    fn purl(&self) -> Option<String> {
        let artifact = self.artifact.clone();
        artifact?.purl
    }

    fn cve(&self) -> Option<String> {
        let vuln = self.vulnerability.clone()?;
        // Check if the primary vulnerability ID is a CVE ID.
        match vuln.namespace {
            None => {}
            Some(namespace) => {
                if namespace == NVD {
                    return vuln.id.clone();
                }
            }
        };

        // If the primary is not a CVE ID, check related vulnerabilities.
        let related = match &self.related_vulnerabilities {
            None => {
                return None;
            }
            Some(related) => related,
        };

        // Return the first found related vulnerability that is a CVE ID.
        for r in related.iter() {
            match &r.namespace {
                None => {}
                Some(namespace) => {
                    if namespace == NVD {
                        return r.id.clone();
                    }
                }
            };
        }

        None
    }

    fn severity(&self) -> Option<Severity> {
        let vuln = self.vulnerability.clone()?;
        let severity = match vuln.severity {
            None => {
                return Some(Severity::Unknown);
            }
            Some(severity) => severity,
        };

        match Severity::from_str(severity.as_str()) {
            Ok(severity) => Some(severity),
            Err(_) => {
                println!("severity_conversion_error");
                Some(Severity::Unknown)
            }
        }
    }

    fn description(&self) -> Option<String> {
        let vuln = self.vulnerability.clone()?;
        vuln.description
    }

    fn cvss(&self) -> Option<HarborCvss> {
        let vuln = self.vulnerability.clone()?;
        // CVSS should be available on the primary if the Primary is sourced from the NVD.
        // Otherwise, we have to check each related vulnerability until we find the one from NVD
        // and then attempt to get CVSS from that one.
        let cvss = match vuln.namespace? == NVD {
            true => vuln.cvss?,
            false => {
                let related_vulns = self.related_vulnerabilities.clone()?;
                related_vulns
                    .iter()
                    .find(|related| {
                        let namespace = related.namespace.clone();
                        namespace.is_some() && namespace.unwrap() == NVD
                    })?
                    .cvss
                    .clone()?
            }
        };

        let mut scores = vec![];
        for score in cvss.iter() {
            let base_score = match score.metrics.clone()?.base_score {
                None => {
                    continue;
                }
                Some(base_score) => base_score,
            };

            let version = match &score.version {
                None => None,
                Some(version) => match Version::from_str(version) {
                    Ok(version) => Some(version),
                    Err(_) => Some(Version::Unknown),
                },
            };

            scores.push(Score {
                score: base_score as f32,
                source: score.source.clone(),
                version,
                vector: score.vector.clone(),
            })
        }

        if scores.is_empty() {
            return None;
        }

        Some(HarborCvss {
            maturity: None,
            mean_score: None,
            median_score: None,
            mode_score: None,
            scores: Some(scores),
        })
    }

    fn remediation(&self) -> Option<Remediation> {
        let vuln = self.vulnerability.clone()?;
        let fix = match vuln.fix {
            None => {
                return None;
            }
            Some(fix) => fix,
        };

        if fix.state != "fixed" {
            return None;
        }

        let versions = match fix.versions {
            None => {
                return None;
            }
            Some(versions) => {
                if versions.is_empty() {
                    return None;
                }
                versions
            }
        };

        let description = format!("fixed in version(s): {}", versions.join(", "));
        Some(Remediation { description })
    }
}

/// MatchDetails contains all data that indicates how the result match was found
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct MatchDetails {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub matcher: Option<String>,
    pub searched_by: Option<serde_json::Value>,
    pub found: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Vulnerability {
    pub id: Option<String>,
    pub data_source: Option<String>,
    pub namespace: Option<String>,
    pub severity: Option<String>,
    pub urls: Option<Vec<String>>,
    pub description: Option<String>,
    pub cvss: Option<Vec<Cvss>>,
    pub fix: Option<Fix>,
    pub advisories: Option<Vec<Advisory>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Fix {
    pub versions: Option<Vec<String>>,
    pub state: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Advisory {
    pub id: Option<String>,
    pub link: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct VulnerabilityMetadata {
    pub id: Option<String>,
    pub data_source: Option<String>,
    pub namespace: Option<String>,
    pub severity: Option<String>,
    pub urls: Option<Vec<String>>,
    pub description: Option<String>,
    pub cvss: Option<Vec<Cvss>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Package {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub r#type: Option<String>,
    pub locations: Option<Vec<Coordinates>>,
    pub language: Option<String>,
    pub licenses: Option<Vec<String>>,
    pub cpes: Option<Vec<String>>,
    pub purl: Option<String>,
    pub upstreams: Option<Vec<UpstreamPackage>>,
    pub metadata_type: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpstreamPackage {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Cvss {
    pub source: Option<String>,
    pub r#type: Option<String>,
    pub version: Option<String>,
    pub vector: Option<String>,
    pub metrics: Option<CvssMetrics>,
    pub vendor_metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct CvssMetrics {
    pub base_score: Option<f64>,
    pub exploitability_score: Option<f64>,
    pub impact_score: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Coordinates {
    pub real_path: Option<String>, // The path where all path ancestors have no hardlinks / symlinks
    pub file_system_id: Option<String>, // An ID representing the filesystem. For container images, this is a layer digest. For directories or a root filesystem, this is blank.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::grype::GrypeOutput;

    #[test]
    fn can_adapt_grype_vulnerabilities() -> Result<(), Error> {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("cannot access CARGO_MANIFEST_DIR");

        let manifest_dir = manifest_dir.replace("sdk/core", "tests/fixtures/core/services/grype");

        let fixture_path = format!("{}/grype-output.json", manifest_dir);

        let raw = std::fs::read_to_string(fixture_path)
            .map_err(|e| Error::Runtime(format!("error reading test fixture: {}", e)))?;

        let grype_output: GrypeOutput =
            serde_json::from_str(raw.as_str()).map_err(|e| Error::Vulnerability(e.to_string()))?;

        assert_eq!(8, grype_output.matches.len());

        let mut harbor_vulns = vec![];
        for grype_vuln in grype_output.matches.iter() {
            match grype_vuln.to_vulnerability() {
                Ok(vuln) => harbor_vulns.push(vuln),
                Err(_) => {}
            }
        }

        assert_eq!(8, harbor_vulns.len());

        let cve_count = harbor_vulns.iter().filter(|r| r.cve.is_some()).count();
        assert_eq!(5, cve_count, "cve count");

        let remediation_count = harbor_vulns
            .iter()
            .filter(|r| r.remediation.is_some())
            .count();

        assert_eq!(7, remediation_count, "remediation count");

        for vuln in harbor_vulns {
            // All should have a purl.
            assert!(!vuln.purl.is_empty());

            match vuln.cve {
                None => {}
                Some(cve) => {
                    // All CVEs that have a CVE ID should start with this prefix.
                    assert!(cve.starts_with("CVE-"));
                }
            }

            match vuln.cvss {
                None => {}
                Some(cvss) => {
                    // Maturity should be None for all that have a CVSS.
                    assert!(cvss.maturity.is_none());
                }
            }
        }

        Ok(())
    }
}
