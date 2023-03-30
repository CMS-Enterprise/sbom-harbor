use std::borrow::BorrowMut;
use std::ops::DerefMut;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::clients::snyk::client::Client;
use crate::clients::snyk::models::{ListOrgProjects200ResponseDataInner, OrgV1};
use crate::Error;

/// Provides Snyk related data retrieval and analytics capabilities.
pub struct SnykService {
    client: Client,
}

/// Provides access to the full set of organization data available for the provided Snyk Token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Registry {
    pub groups: Vec<Group>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self { groups: vec![] }
    }

    /// Add a Group to the Vector. Because Groups are embedded as references, this particular
    /// vector add function uses the local group model rather than the Snyk API model since that
    /// has already been adapted by the controller function.
    pub(crate) fn groups(&mut self, group: Group) {
        self.groups.push(group);
    }
}

/// Adapter over a native Snyk Group.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    inner: crate::clients::snyk::models::Group,
    pub id: String,
    pub name: String,
    pub orgs: Vec<Organization>,
    pub org_count: u32,
    pub empty_org_count: u32,
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
            empty_org_count: 0,
        }
    }

    pub(crate) fn orgs(&mut self, org: Organization) {
        self.orgs.push(org)
    }
}

/// Adapter over a native Snyk Org.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Organization {
    inner: OrgV1,
    pub projects: Vec<Project>,
}

impl Organization {
    pub(crate) fn new(inner: OrgV1) -> Self {
        Self {
            inner,
            projects: vec![],
        }
    }

    pub(crate) fn projects(&mut self, inner: ListOrgProjects200ResponseDataInner) {
        let project = Project::new(inner);
        self.projects.push(project)
    }
}

/// Adapter over a native Snyk Project.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    inner: ListOrgProjects200ResponseDataInner,
}

impl Project {
    pub fn new(inner: ListOrgProjects200ResponseDataInner) -> Self {
        Self {
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
            empty_org_count: 0,
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