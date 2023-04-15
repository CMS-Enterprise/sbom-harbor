use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use crate::entities::cyclonedx::{Bom, Vulnerability};
use async_trait::async_trait;
use platform::config::{from_env, sdk_config_from_env};
use platform::mongodb::{Context, Store};
use platform::persistence::s3;
use tracing::debug;

use crate::entities::packages::Source::Dependency;
use crate::entities::packages::{
    CdxComponent, Dependency, DependencyKind, Package, Purl, Registry, SnykXRef, Source,
    Unsupported, Vulnerability,
};
use crate::entities::sboms::{CdxFormat, Spec};
use crate::services::cyclonedx::models::{Bom, Component};
use crate::services::cyclonedx::{bom_eq, bom_parse, bom_purl, extract_purls};
use crate::services::sbom::{SbomProvider, SbomService};
use crate::services::snyk::adapters::{Issue, Organization, Project};
use crate::services::{SbomProvider, SbomService};
use crate::Error;

use crate::models::sbom::Spec;
use crate::services::sboms::{SbomProvider, SbomService};
use crate::services::snyk::client::{Client, SbomFormat};
use crate::services::snyk::service::adapters::Organization;

// TODO: Lazy Static or OnceCell this.
pub(in crate::services::snyk) const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
    "npm",
    "swift",
    "maven",
    "cocoapods",
    "composer",
    "gem",
    "nuget",
    "pypi",
    "hex",
    "cargo",
    "generic",
];

/// Provides Snyk related data retrieval and analytics capabilities.
#[derive(Debug)]
pub struct SnykService {
    pub(in crate::service::snyk) client: Client,
    pub(in crate::service::snyk) cx: Context,
}

impl SnykService {
    /// Factory method to create new instance of type.
    pub fn new(token: String, cx: Context) -> Self {
        let client = Client::new(token);
        Self { client, cx }
    }

    /// Retrieves orgs from the Snyk API.
    pub async fn orgs(&self) -> Result<Vec<Organization>, Error> {
        let orgs = match self.client.orgs().await {
            Ok(orgs) => orgs,
            Err(e) => {
                return Err(Error::Snyk(e.to_string()));
            }
        };

        match orgs {
            None => {
                return Err(Error::Snyk("orgs_not_found".to_string()));
            }
            Some(orgs) => {
                if orgs.is_empty() {
                    return Err(Error::Snyk("orgs_empty".to_string()));
                }

                let mut results = vec![];

                orgs.into_iter().for_each(|inner| {
                    results.push(Organization::new(inner));
                });

                Ok(results)
            }
        }
    }

    /// Retrieves [Projects] for an [Organization] from the Snyk API.
    pub async fn projects(&self, org: &Organization) -> Result<Vec<Project>, Error> {
        let projects = match self.client.projects(org.id.as_str()).await {
            Ok(projects) => projects,
            Err(e) => {
                return Err(Error::Snyk(format!(
                    "projects::org_name::{}::org_id::{}::{}",
                    org.name, org.id, e
                )));
            }
        };

        match projects {
            None => {
                return Err(Error::Snyk(format!(
                    "projects::org_name::{}::org_id::{}::projects_not_found",
                    org.name, org.id
                )));
            }
            Some(projects) => {
                if projects.is_empty() {
                    return Err(Error::Snyk(format!(
                        "projects::org_name::{}::org_id::{}::projects_empty",
                        org.name, org.id
                    )));
                }
                let mut results = vec![];

                projects.iter().for_each(|inner| {
                    result.push(Project::new(inner.clone()));
                });

                Ok(results)
            }
        }
    }
}
