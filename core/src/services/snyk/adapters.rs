use serde::{Deserialize, Serialize};

use crate::clients::snyk::models::{
    CommonIssueModel, ListOrgProjects200ResponseDataInner, OrgV1, ProjectStatus, Severity,
};
use crate::entities::cyclonedx::{Bom, Severity};
use crate::entities::packages::{PackageXRef, SnykXRef, Unsupported};
use crate::models::cyclonedx::Bom;
use crate::services::cyclonedx::models::Bom;

/// Adapter over a native Snyk Group.
pub(in crate::services::snyk) struct Group {
    pub id: String,
    pub name: String,
}

impl Group {
    pub fn new(inner: crate::clients::snyk::models::Group) -> Self {
        let id = inner.id.clone().unwrap_or("group id not set".to_string());
        let name = inner
            .name
            .clone()
            .unwrap_or("group name not set".to_string());

        Self { id, name }
    }
}

/// Adapter over a native Snyk Org.
pub(in crate::services::snyk) struct Organization {
    pub id: String,
    pub name: String,
    pub(crate) inner: OrgV1,
}

impl Organization {
    pub fn new(inner: OrgV1) -> Self {
        let id = inner
            .id
            .clone()
            .unwrap_or("org id not set".to_string())
            .clone();
        let name = inner
            .name
            .clone()
            .unwrap_or("org name not set".to_string())
            .clone();
        Self { id, name, inner }
    }
}

/// Adapter over a native Snyk Project.
pub(in crate::services::snyk) struct Project {
    pub id: String,
    pub name: String,
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
        org_id: String,
        org_name: String,
        inner: ListOrgProjects200ResponseDataInner,
    ) -> Self {
        let id = inner.id.clone().to_string();
        let mut name = "project name not set".to_string();
        let mut target_file = "".to_string();
        let mut target_reference = "".to_string();
        let mut status = ProjectStatus::default();
        let mut package_manager = "unknown".to_string();

        match inner.clone().attributes {
            None => {}
            Some(attrs) => {
                name = attrs.name.clone();
                target_file = attrs.target_file.clone();
                target_reference = attrs.target_reference.clone();
                status = attrs.status.clone();
                package_manager = attrs.r#type.clone();
            }
        }

        Self {
            id,
            name,
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
            name: self.name.clone(),
            package_manager: self.package_manager.clone(),
            snyk_refs: vec![self.to_snyk_xref()],
        }
    }

    pub fn to_snyk_xref(&self) -> SnykXRef {
        SnykXRef {
            id: "".to_string(),
            active: self.status == ProjectStatus::Active,
            org_id: self.org_id.clone(),
            org_name: self.org_name.clone(),
            group_id: org.group().id,
            group_name: org.group().name,
            project_id: self.id.clone(),
            project_name: self.name.clone(),
        }
    }
}

/// Adapter over a native Snyk Issue.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::services::snyk) struct Issue {
    pub id: String,
    pub purl: String,
    pub title: String,
    pub description: String,
    pub versions: Vec<String>,
    pub effective_security_level: String,
    pub severities: Vec<Severity>,
    pub(crate) inner: CommonIssueModel,
}

impl Issue {
    pub fn new(purl: String, inner: CommonIssueModel) -> Self {
        let id = inner
            .id
            .clone()
            .unwrap_or("issue id not set".to_string())
            .clone();
        let mut title = "".to_string(); // from attributes.title
        let mut description = "".to_string(); // from attributes
        let mut versions = vec![]; // from attributes.coordinates.representation
        let mut effective_security_level = "".to_string(); // attributes.effective_security_level
        let mut severities = vec![]; // from attributes.severities

        match inner.clone().attributes {
            None => {}
            Some(attrs) => {
                let attrs = *attrs;

                match attrs.title.clone() {
                    None => {}
                    Some(t) => title = t,
                }

                match attrs.description.clone() {
                    None => {}
                    Some(d) => description = d,
                }

                match attrs.coordinates.clone() {
                    None => {}
                    Some(c) => {
                        c.iter()
                            .for_each(|coord| match coord.clone().representation {
                                None => {}
                                Some(r) => {
                                    r.iter().for_each(|r| versions.push(r.to_string()));
                                }
                            });
                    }
                }

                match attrs.effective_severity_level {
                    None => {}
                    Some(efs) => effective_security_level = format!("{:?}", efs),
                }

                match attrs.severities {
                    None => {}
                    Some(sevs) => sevs.iter().for_each(|sev| severities.push(sev.clone())),
                }
            }
        }

        Self {
            id,
            purl,
            title,
            description,
            versions,
            effective_security_level,
            severities,
            inner,
        }
    }
}
