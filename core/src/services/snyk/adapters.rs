use serde::{Deserialize, Serialize};

use crate::entities::packages::{Finding, FindingProviderKind, Unsupported};
use crate::entities::sboms::SbomProviderKind;
use crate::entities::xrefs::Xref;
use crate::services::snyk::client::models::{
    ListOrgProjects200ResponseDataInner, OrgV1, ProjectStatus,
};
use crate::services::snyk::{IssueSnyk, SnykRef};

use crate::services::snyk::API_VERSION;
use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Project);

/// Adapter over a native Snyk Group.
pub(crate) struct Group {
    pub id: String,
    pub name: String,
}

impl Group {
    pub fn new(inner: crate::services::snyk::client::models::Group) -> Self {
        let id = inner.id.clone().unwrap_or("group id not set".to_string());
        let name = inner
            .name
            .unwrap_or("group name not set".to_string());

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
        let id = inner
            .id
            .clone()
            .unwrap_or("org id not set".to_string());

        let name = inner
            .name
            .clone()
            .unwrap_or("org name not set".to_string());
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
            provider: SbomProviderKind::Snyk {
                api_version: API_VERSION.to_string(),
            },
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
    pub(crate) fn to_finding(&self, purl: String, xrefs: Vec<Xref>) -> Finding {
        Finding {
            provider: FindingProviderKind::Snyk,
            purl: Some(purl),
            cdx: None,
            snyk_issue: Some(self.clone()),
            xrefs,
        }
    }
}
