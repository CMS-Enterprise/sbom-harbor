use std::borrow::BorrowMut;
use std::ops::DerefMut;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::clients::snyk::client::Client;
use crate::clients::snyk::models::{CommonIssueModel, CommonIssueModelAttributes, Coordinate, EffectiveSeverityLevel, ListOrgProjects200ResponseDataInner, OrgV1, ProjectAttributes, ProjectStatus, Severity};
use crate::Error;

/// Provides Snyk related data retrieval and analytics capabilities.
pub struct SnykService {
    client: Client,
}

/// Provides access to the full set of organization data available for the provided Snyk Token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Registry {
    pub group_count: u32,
    pub groups: Vec<Group>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self { group_count: 0, groups: vec![] }
    }

    /// Add a Group to the Vector. Because Groups are embedded as references, this particular
    /// vector add function uses the local group model rather than the Snyk API model since that
    /// has already been adapted by the controller function.
    pub(crate) fn groups(&mut self, group: Group) {
        self.groups.push(group);
        self.group_count = self.groups.len() as u32;
    }
}

/// Adapter over a native Snyk Group.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub org_count: u32,
    pub(crate) inner: crate::clients::snyk::models::Group,
    pub orgs: Vec<Organization>,
}

impl Group {
    pub(crate) fn new(inner: crate::clients::snyk::models::Group) -> Self {
        let id = inner.id.clone().unwrap_or("group id not set".to_string());
        let name = inner.name.clone().unwrap_or("group name not set".to_string());

        Self {
            inner,
            id,
            name,
            orgs: vec![],
            org_count: 0,
        }
    }

    pub(crate) fn orgs(&mut self, org: Organization) {
        self.orgs.push(org);
        self.org_count = self.orgs.len() as u32;
    }
}

/// Adapter over a native Snyk Org.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub project_count: u32,
    pub issue_count: u32,
    pub(crate) inner: OrgV1,
    pub projects: Vec<Project>,
    pub issues: Vec<Issue>,
}

impl Organization {
    pub(crate) fn new(inner: OrgV1) -> Self {
        let id = inner.id.clone().unwrap_or("org id not set".to_string()).clone();
        let name = inner.name.clone().unwrap_or("org name not set".to_string()).clone();
        Self {
            id,
            name,
            inner,
            issue_count: 0,
            issues: vec![],
            projects: vec![],
            project_count: 0,
        }
    }

    pub(crate) fn issues(&mut self, inner: CommonIssueModel) {
        let issue = Issue::new(inner);
        self.issues.push(issue);
        self.issue_count = self.issues.len() as u32;
    }

    pub(crate) fn projects(&mut self, inner: ListOrgProjects200ResponseDataInner) {
        let project = Project::new(inner);
        self.projects.push(project);
        self.project_count = self.projects.len() as u32;
    }
}

/// Adapter over a native Snyk Project.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    /// Path within the target to identify a specific file/directory/image etc. when scanning just part of the target, and not the entity.
    pub(crate) target_file: String,
    /// The additional information required to resolve which revision of the resource should be scanned.
    pub(crate) target_reference: String,
    /// Describes if a project is currently monitored or it is de-activated.
    pub(crate) status: ProjectStatus, // Useful for skipping inactive projects.
    pub(crate) package_manager: String,
    pub(crate) inner: ListOrgProjects200ResponseDataInner,
}

impl Project {
    pub fn new(inner: ListOrgProjects200ResponseDataInner) -> Self {
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
            target_file,
            target_reference,
            status,
            package_manager,
            inner
        }
    }
}

/// Adapter over a native Snyk Issue.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub description: String,
    pub versions: Vec<String>,
    pub effective_security_level: String,
    pub severities: Vec<Severity>,
    pub(crate) inner: CommonIssueModel,
}

impl Issue {
    pub fn new(inner: CommonIssueModel) -> Self {
        let id = inner.id.clone().unwrap_or("issue id not set".to_string()).clone();
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
                    Some(t) => title = t
                }

                match attrs.description.clone() {
                    None => {}
                    Some(d) => description = d
                }

                match attrs.coordinates.clone() {
                    None => {}
                    Some(c) => {
                        c.iter().for_each(|coord| {
                            match coord.clone().representation {
                                None => {}
                                Some(r) => {
                                    r.iter().for_each(|r| versions.push(r.to_string()));
                                }
                            }
                        });
                    }
                }

                match attrs.effective_severity_level {
                    None => {}
                    Some(efs) => effective_security_level = format!("{:?}", efs)
                }

                match attrs.severities {
                    None => {}
                    Some(sevs) => {
                        sevs.iter().for_each(|sev| severities.push(sev.clone()))
                    }
                }
            }
        }

        Self {
            id,
            title,
            description,
            versions,
            effective_security_level,
            severities,
            inner
        }
    }
}

impl SnykService {
    /// Factory method to create new instances of a SnykService.
    pub fn new(token: String) -> Self {
        let client = Client::new(token);
        Self {
            client,
        }
    }

    pub async fn build_registry(&self) -> Result<Registry, Error> {
        let mut registry = Registry::new();

        // Build the set of groups and their organizations.
        let orgs = self.client.orgs().await?;

        let orgs = match orgs {
            None => {
                return Err(Error::Config("no organizations for token".to_string()));
            }
            Some(o) => o,
        };

        // Keep a group to add projects with no group set to, and then add to Registry at the end.
        let mut none_group = Group {
            inner: Default::default(),
            id: "none-group".to_string(),
            name: "Orgs with no group set".to_string(),
            orgs: vec![],
            org_count: 0,
        };

        // WATCH: This logic is indirect and may change. Snyk orgs return references to their
        // containing group. By getting the list of Orgs first rather than the Group we can reduce
        // the number of network calls. However, we want to structure our model with the Groups as
        // the Aggregate Root. Groups map to GitHub organizations which is a more useful perspective
        // of the data for analysis. This requires us to iterate over the organizations, and filter
        // them into Groups along the way. We are also analyzing and filtering Projects within an
        // Org in this process. Project network calls are serial currently, but will likely change
        // to concurrent in future iterations.
        for org in orgs {
            let mut org = Organization::new(org);
            // fill the org projects first.
            match org.inner.id.clone() {
                None => {}
                Some(org_id) => {
                    match self.client.projects(org_id.as_str()).await {
                        Ok(projects) => {
                            match projects {
                                Some(projects) => {
                                    for project in projects {
                                        org.projects(project);
                                    }
                                },
                                _ => {},
                            }
                        },
                        Err(e) => {
                            debug!("error fetching projects: {}", e);
                        }
                    };

                    // Removing for now. Have to have purls for this to work.  It's a way
                    // to get batched issues though so is more efficient for sure.
                    // match self.client.get_org_issues(org_id.as_str()).await {
                    //     Ok(issues) => {
                    //         match issues {
                    //             Some(issues) => {
                    //                 for issue in issues {
                    //                     org.issues(issue);
                    //                 }
                    //             },
                    //             _ => {},
                    //         }
                    //     },
                    //     Err(e) => {
                    //         debug!("error fetching issues: {}", e);
                    //     }
                    // };
                }
            }

            match org.inner.group.clone() {
                None => {
                    none_group.orgs(org);
                },
                Some(snyk_group) => {
                    let group_id = snyk_group.id.clone().unwrap_or_default();

                    let group = registry.groups
                        .iter_mut()
                        .find(|g| g.id == group_id);

                    // If the group hasn't been found yet, add it.
                    // In either case, add the org to the group.
                    match group {
                        None => {
                            let mut new_group = Group::new(snyk_group.clone());
                            new_group.orgs(org);
                            registry.groups(new_group);
                        }
                        Some(mut g) => g.orgs(org),
                    };
                },
            }
        }

        registry.groups(none_group);


        Ok(registry)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_build_registry() -> Result<(), Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let service = SnykService::new(token);
        let registry = service.build_registry().await?;

        assert!(registry.groups.len() > 0);

        let json = serde_json::to_string(&registry)
            .map_err(|e| Error::Runtime(e.to_string()))?;

        let mut file = std::fs::File::create("/Users/derek/code/scratch/debug/snyk-registry.json")
            .map_err(|e| Error::Runtime(e.to_string()))?;

        file.write_all(json.as_ref()).map_err(|e| Error::Runtime(e.to_string()))?;

        Ok(())
    }
}