use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use async_trait::async_trait;
use platform::config::{from_env, sdk_config_from_env};
use platform::mongodb::{Context, Store};
use platform::persistence::s3;
use tracing::debug;

use crate::entities::cyclonedx::models::{Bom, Component};
use crate::entities::packages::{Dependency, Finding, Package, Purl, Unsupported};
use crate::entities::sboms::{CdxFormat, Spec};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::findings::FindingService;
use crate::services::snyk::adapters::{Issue, Organization, Project};
use crate::Error;

use crate::services::sboms::SbomService;
use crate::services::snyk::client::client::Client;
use crate::services::snyk::client::models::CommonIssueModel;
use crate::services::snyk::{IssueSnyk, SbomFormat, SnykRef};
use crate::services::xrefs::XrefService;

/// Provides Snyk related data retrieval and analytics capabilities. This service is used by
/// other domain specific services when they need to access the Snyk API. The Snyk API should not
/// be exposed directly.
#[derive(Debug)]
pub struct SnykService {
    /// The Snyk API Client instance.
    client: Client,
    /// The datastore connection context.
    cx: Context,
}

impl SnykService {
    /// Factory method to create new instance of type.
    pub fn new(token: String, cx: Context) -> Self {
        let client = Client::new(token);
        Self { client, cx }
    }

    /// Retrieves orgs from the Snyk API.
    pub(in crate::services) async fn orgs(&self) -> Result<Vec<Organization>, Error> {
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

    /// Gathers all projects across all orgs so that index can be analyzed linearly.
    pub(in crate::services) async fn projects(&self) -> Result<Vec<Project>, Error> {
        let mut projects = vec![];

        let mut orgs = match self.orgs().await {
            Ok(o) => o,
            Err(e) => {
                let msg = format!("gather_projects::orgs::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg));
            }
        };

        // Get projects for each org.
        for org in orgs.iter() {
            // Get the projects for the org.
            match self.projects_by_org(org).await {
                Ok(p) => {
                    projects.extend(p.into_iter());
                }
                Err(e) => {
                    debug!("gather_projects::projects::{}", e);
                    continue;
                }
            }
        }

        Ok(projects)
    }

    /// Retrieves [Projects] for an [Organization] from the Snyk API.
    pub(in crate::services) async fn projects_by_org(
        &self,
        org: &Organization,
    ) -> Result<Vec<Project>, Error> {
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
                    results.push(Project::new(
                        org.group().id.clone(),
                        org.group().name.clone(),
                        org.id.clone(),
                        org.name.clone(),
                        inner.clone(),
                    ));
                });

                Ok(results)
            }
        }
    }

    /// Retrieves raw native Snyk [Sbom] JSON from the Snyk API.
    pub(in crate::services) async fn sbom_raw(&self, snyk_ref: &SnykRef) -> Result<String, Error> {
        let raw = match self
            .client
            .sbom_raw(
                snyk_ref.org_id.as_str(),
                snyk_ref.project_id.as_str(),
                SbomFormat::CycloneDxJson,
            )
            .await
        {
            Ok(raw) => raw,
            Err(e) => {
                let msg = format!("sbom_raw::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg));
            }
        };

        let raw = match raw {
            None => {
                // TODO: Emit Metric.
                let msg = "sbom_raw::sbom::none";
                debug!(msg);
                return Err(Error::Snyk(msg.to_string()));
            }
            Some(raw) => {
                if raw.is_empty() {
                    let msg = "sbom_raw::sbom::empty";
                    debug!(msg);
                    return Err(Error::Snyk(msg.to_string()));
                }
                raw
            }
        };

        Ok(raw)
    }

    /// Get findings for a Package URL.
    pub(in crate::services) async fn findings(
        &self,
        org_id: &str,
        purl: &str,
        xrefs: Vec<Xref>,
    ) -> Result<Option<Vec<Finding>>, Error> {
        let issues = match self.issues(org_id, purl).await {
            Ok(issues) => issues,
            Err(e) => {
                return Err(Error::Snyk(e.to_string()));
            }
        };

        let issues = match issues {
            None => {
                return Ok(None);
            }
            Some(issues) => issues,
        };

        if issues.is_empty() {
            return Ok(None);
        }

        let mut results = vec![];

        issues.iter().for_each(|issue| {
            results.push(issue.to_finding(purl.to_string(), xrefs.clone()));
        });

        Ok(Some(results))
    }

    /// Get native Snyk issues. External callers should most likely use [findings].
    pub(in crate::services::snyk) async fn issues(
        &self,
        org_id: &str,
        purl: &str,
    ) -> Result<Option<Vec<IssueSnyk>>, Error> {
        let issues = match self.client.get_issues(org_id, purl).await {
            Ok(issues) => issues,
            Err(e) => {
                return Err(Error::Snyk(format!("snyk::issues::{}", e)));
            }
        };

        let issues = match issues {
            None => {
                return Ok(None);
            }
            Some(issues) => issues,
        };

        if issues.is_empty() {
            return Ok(None);
        }

        Ok(Some(issues))
    }
}
