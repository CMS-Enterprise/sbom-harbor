use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::entities::packages::Unsupported;
use crate::entities::sboms::SbomProviderKind;
use crate::entities::xrefs::Xref;
use crate::services::snyk::client::models::{
    EffectiveSeverityLevel, ListOrgProjects200ResponseDataInner, OrgV1, ProjectStatus,
};
use crate::services::snyk::{IssueSnyk, SnykRef};

use crate::entities::enrichments::cvss::{Cvss, Maturity, Score, Version};
use crate::entities::enrichments::{
    Cwe, Remediation, Severity, Vulnerability, VulnerabilityProviderKind,
};
use crate::Error;

/// Adapter over a native Snyk Group.
pub(crate) struct Group {
    pub id: String,
    pub name: String,
}

impl Group {
    pub fn new(inner: crate::services::snyk::client::models::Group) -> Self {
        let id = inner.id.clone().unwrap_or("group id not set".to_string());
        let name = inner.name.unwrap_or("group name not set".to_string());

        Self { id, name }
    }
}

/// Adapter over a native Snyk Org.
pub(crate) struct Organization {
    pub id: String,
    pub name: String,
    pub(crate) inner: OrgV1,
}

impl Organization {
    pub fn new(inner: OrgV1) -> Self {
        let id = inner.id.clone().unwrap_or("org id not set".to_string());

        let name = inner.name.clone().unwrap_or("org name not set".to_string());
        Self { id, name, inner }
    }

    pub fn group(&self) -> Group {
        Group::new(self.inner.group.clone().unwrap_or_default())
    }
}

/// Adapter over a native Snyk Project.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Project {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub group_id: String,
    pub group_name: String,
    pub org_id: String,
    pub org_name: String,
    pub package_manager: String,

    /// Path within the target to identify a specific file/directory/image etc. when scanning just part of the target, and not the entity.
    pub target_file: String,
    /// The additional information required to resolve which revision of the resource should be scanned.
    pub target_reference: String,
    /// Describes if a project is currently monitored or it is de-activated. Useful for skipping inactive projects.
    pub status: ProjectStatus,
    pub inner: ListOrgProjects200ResponseDataInner,
}

impl Project {
    pub fn new(
        group_id: String,
        group_name: String,
        org_id: String,
        org_name: String,
        inner: ListOrgProjects200ResponseDataInner,
    ) -> Self {
        let id = "".to_string();
        let project_id = inner.id.clone().to_string();
        let mut project_name = "project name not set".to_string();
        let mut target_file = "".to_string();
        let mut target_reference = "".to_string();
        let mut status = ProjectStatus::default();
        let mut package_manager = "unknown".to_string();

        match inner.clone().attributes {
            None => {}
            Some(attrs) => {
                project_name = attrs.name.clone();
                target_file = attrs.target_file.clone();
                target_reference = attrs.target_reference.clone();
                status = attrs.status;
                package_manager = attrs.r#type.clone();
            }
        }

        Self {
            id,
            project_id,
            project_name,
            group_id,
            group_name,
            org_id,
            org_name,
            target_file,
            target_reference,
            status,
            package_manager,
            inner,
        }
    }

    pub fn to_unsupported(&self) -> Unsupported {
        Unsupported {
            id: "".to_string(),
            external_id: self.project_id.clone(),
            name: self.project_name.clone(),
            package_manager: Some(self.package_manager.clone()),
            provider: SbomProviderKind::Snyk,
            xrefs: vec![Xref::from(self.to_snyk_ref())],
        }
    }

    pub fn to_snyk_ref(&self) -> SnykRef {
        SnykRef {
            org_id: self.org_id.clone(),
            org_name: self.org_name.clone(),
            group_id: self.group_id.clone(),
            group_name: self.group_name.clone(),
            project_id: self.project_id.clone(),
            project_name: self.project_name.clone(),
        }
    }
}

/// Adapter over a native Snyk Issue.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Issue {}

impl IssueSnyk {
    pub(crate) fn to_vulnerability(&self, purl: &str) -> Vulnerability {
        let raw = match serde_json::to_string(&self).map_err(|e| Error::Serde(e.to_string())) {
            Ok(raw) => Some(raw),
            Err(_) => None,
        };

        let severity = self.severity();
        let purl = purl.to_string();

        Vulnerability {
            id: "".to_string(),
            purl,
            provider: VulnerabilityProviderKind::Snyk,
            severity,
            cve: self.cve(),
            epss_score: None,
            description: self.description(),
            cvss: self.cvss(),
            cwes: self.cwes(),
            remediation: self.remediation(),
            raw,
            task_refs: vec![],
        }
    }

    fn severity(&self) -> Option<Severity> {
        let mut result = Severity::Unknown;

        if let Some(severity) = self.attributes.as_deref()?.effective_severity_level {
            result = match severity {
                EffectiveSeverityLevel::Info => Severity::Info,
                EffectiveSeverityLevel::Low => Severity::Low,
                EffectiveSeverityLevel::Medium => Severity::Medium,
                EffectiveSeverityLevel::High => Severity::High,
                EffectiveSeverityLevel::Critical => Severity::Critical,
            };
        }

        Some(result)
    }

    fn cve(&self) -> Option<String> {
        match &self.attributes.as_deref()?.problems {
            None => None,
            Some(problems) => problems
                .iter()
                .find(|problem| problem.source == "CVE")
                .map(|problem| problem.id.clone()),
        }
    }

    fn cwes(&self) -> Option<Vec<Cwe>> {
        match &self.attributes.as_deref()?.problems {
            None => None,
            Some(problems) => {
                let cwes: Vec<Cwe> = problems
                    .iter()
                    .filter(|problem| problem.source == "CWE")
                    .map(|problem| Cwe {
                        id: problem.id.clone(),
                        name: None,
                        description: None,
                    })
                    .collect();

                if cwes.is_empty() {
                    return None;
                }

                Some(cwes)
            }
        }
    }

    fn description(&self) -> Option<String> {
        self.attributes.as_deref()?.description.as_ref().cloned()
    }

    fn cvss(&self) -> Option<Cvss> {
        let mut detail = Cvss {
            maturity: self.resolve_maturity(),
            mean_score: None,
            median_score: None,
            mode_score: None,
            scores: self.resolve_scores(),
        };

        detail.calculate_scores();

        Some(detail)
    }

    fn resolve_scores(&self) -> Option<Vec<Score>> {
        let severities = match &self.attributes.as_deref()?.severities {
            None => vec![],
            Some(severities) => severities.to_vec(),
        };

        let mut scores = vec![];
        for severity in severities.into_iter() {
            match severity.score? {
                None => {}
                Some(score) => {
                    let vector = severity.vector?.clone();
                    scores.push(Score {
                        score,
                        source: severity.source.clone(),
                        version: version_from_vector(vector.clone()),
                        vector,
                    });
                }
            };
        }

        if scores.is_empty() {
            return None;
        }

        Some(scores)
    }

    fn resolve_maturity(&self) -> Option<Maturity> {
        if let Some(exploit) = &self
            .attributes
            .as_deref()?
            .slots
            .as_deref()?
            .exploit
            .clone()
        {
            let exploit_maturity = match ExploitMaturity::from_str(exploit.as_str()) {
                Ok(exploit_maturity) => exploit_maturity,
                Err(_) => ExploitMaturity::NotDefined,
            };

            return Some(Maturity::from(exploit_maturity));
        }

        None
    }

    fn remediation(&self) -> Option<Remediation> {
        let mut description = vec![];
        let coordinates = self.attributes.as_deref()?.coordinates.clone()?;

        for coordinate in coordinates {
            for remedy in coordinate.remedies? {
                match remedy.description {
                    None => {}
                    Some(remedy) => description.push(remedy),
                }
            }
        }

        Some(Remediation {
            description: description.join("\n"),
        })
    }
}

enum ExploitMaturity {
    NoData,
    NotDefined,
    Unproven,
    ProofOfConcept,
    Functional,
    High,
}

impl FromStr for ExploitMaturity {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "No Data" => Ok(Self::NoData),
            "Not Defined" => Ok(Self::NotDefined),
            "Unproven" => Ok(Self::Unproven),
            "Proof of Concept" => Ok(Self::ProofOfConcept),
            "Functional" => Ok(Self::Functional),
            "High" => Ok(Self::High),
            _ => Err(Error::Snyk(format!("invalid_exploit_maturity::{}", input))),
        }
    }
}

impl From<ExploitMaturity> for Maturity {
    fn from(value: ExploitMaturity) -> Self {
        match value {
            ExploitMaturity::NoData => Maturity::NotDefined,
            ExploitMaturity::NotDefined => Maturity::NotDefined,
            ExploitMaturity::Unproven => Maturity::Unproven,
            ExploitMaturity::ProofOfConcept => Maturity::ProofOfConcept,
            ExploitMaturity::Functional => Maturity::Functional,
            ExploitMaturity::High => Maturity::High,
        }
    }
}

fn version_from_vector(vector: Option<String>) -> Option<Version> {
    let vector = match vector {
        None => {
            return None;
        }
        Some(vector) => vector,
    };

    // TODO: This needs hardening.
    if vector.starts_with("CVSS:1") {
        return Some(Version::V1);
    } else if vector.starts_with("CVSS:2") {
        return Some(Version::V2);
    } else if vector.starts_with("CVSS:3.0") {
        return Some(Version::V3);
    } else if vector.starts_with("CVSS:3.1") {
        return Some(Version::V3_1);
    } else if vector.starts_with("CVSS:4") {
        return Some(Version::V4);
    }

    None
}
