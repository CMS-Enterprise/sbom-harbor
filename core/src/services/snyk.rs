use std::borrow::BorrowMut;
use std::ops::DerefMut;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::clients::snyk::client::Client;
use crate::clients::snyk::models::{CommonIssueModel, CommonIssueModelAttributes, Coordinate, EffectiveSeverityLevel, ListOrgProjects200ResponseDataInner, OrgV1, ProjectAttributes, ProjectStatus, Severity};
use crate::Error;

pub use crate::clients::snyk::client::SbomFormat;
use crate::models::cyclonedx::Bom;
use crate::models::sboms::CycloneDxFormat;
use crate::services::SbomService;

// TODO: Lazy Static or OnceCell this.
// const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
//     "npm", "nuget", "hex", "pip", "poetry", "rubygems",
//     "maven", "yarn", "yarn-workspace", "composer", "gomodules",
//     "govendor", "golang", "golangdep", "paket",
//     "cocoapods", "cpp", "sbt"];

const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
    "npm", "swift", "maven", "cocoapods", "composer", "gem", "nuget", "pypi", "hex", "cargo", "generic"];

// API Returned this
// Error: "https://docs.snyk.io/more-info/error-catalog#snyk-ossi-2020-ecosystem-is-not-supported
// "status\\\":\\\"400\\\",\\\"code\\\":\\\"SNYK-OSSI-2020\\\",\\\"title\\\":\\\"Ecosystem is not supported\\\",
// \\\"detail\\\":\\\"Unsupported ecosystem gradle.
// Supported Ecosystems: npm, swift, maven, cocoapods, composer, gem, nuget, pypi, hex, cargo, generic

//Error: Enrich("runtime error: Remote(\"{\\\"jsonapi\\\":{\\\"version\\\":\\\"1.0\\\"},\\\"errors\\\":[{\\\"id\\\":\\\"21ada504-2f25-4e7e-9e9f-15df9e484814\\\",\\\"links\\\":{\\\"about\\\":\\\"https://docs.snyk.io/more-info/error-catalog#snyk-ossi-2020-ecosystem-is-not-supported\\\"},\\\"status\\\":\\\"400\\\",\\\"code\\\":\\\"SNYK-OSSI-2020\\\",\\\"title\\\":\\\"Ecosystem is not supported\\\",\\\"detail\\\":\\\"Unsupported ecosystem gradle. Supported Ecosystems: npm, swift, maven, cocoapods, composer, gem, nuget, pypi, hex, cargo, generic\\\",\\\"source\\\":{\\\"pointer\\\":\\\"/orgs/0aacbfcb-6a16-4307-b38f-e1ef0cd017ca/packages/pkg%3Agradle%2Fcom.fasterxml.jackson.core%253Ajackson-annotations%402.13.3/issues\\\"},\\\"meta\\\":{\\\"links\\\":[],\\\"purl\\\":\\\"pkg:gradle/com.fasterxml.jackson.core%3Ajackson-annotations@2.13.3\\\"}}]}\")")

pub fn is_sbom_project_type(project_type: &str) -> bool {
    SUPPORTED_SBOM_PROJECT_TYPES.contains(&project_type)
}

/// Provides Snyk related data retrieval and analytics capabilities.
pub struct SnykService {
    client: Client,
}

/// Provides a
pub struct Purl {
    raw: String,
    package: String,
    version: String,
    projects: Vec<Project>,
}

impl Purl {
    pub fn new(raw: String) -> Self {
        let mut package = "".to_string();
        let mut version = "".to_string();

        let parts = raw.split("@").collect::<Vec<&str>>();
        if parts.len() == 2 {
            package = parts[0].to_string();
            version = parts[1].to_string();
        }
        Self{
            raw,
            package,
            version,
            projects: vec![],
        }
    }
}

/// Provides access to the full set of organization data available for the provided Snyk Token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Registry {
    pub purls: Vec<Purl>,
    pub purl_count: u32,
    pub groups: Vec<Group>,
    pub group_count: u32,
}

impl Registry {
    /// Factory method to create new instance of type.
    pub(crate) fn new() -> Self {
        Self {
            purl_count: 0,
            purls: vec![],
            group_count: 0,
            groups: vec![]
        }
    }

    /// Add a Group to the Registry.
    pub(crate) fn groups(&mut self, group: Group) {
        self.groups.push(group);
    }

    /// Add a Purl to the Registry.
    pub(crate) fn purls(&mut self, purl: Purl) {
        self.purls.push(group);
    }

    pub(crate) fn summarize(&mut self) {
        self.group_count = self.groups.len() as u32;
        self.purl_count = self.purls.len() as u32;

        for group in self.groups.iter_mut() {
            for org in group.orgs.iter_mut() {
                org.summarize();
            }
        }
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

    pub(crate) fn none() -> Self {
         Self {
            inner: Default::default(),
            id: "none-group".to_string(),
            name: "Orgs with no group set".to_string(),
            orgs: vec![],
            org_count: 0,
        };
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

    pub(crate) fn projects(&mut self, project: Project) {
        self.projects.push(project);
        self.project_count = self.projects.len() as u32;
    }

    pub(crate) fn summarize(&mut self) {
        for project in &self.projects {
            self.issue_count = self.issue_count + project.issues_count;
        }
    }
}

/// Adapter over a native Snyk Project.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,

    pub(crate) package_manager: String,
    pub bom: Option<Bom>,
    pub issues: Vec<Issue>,
    pub issues_count: u32,
    /// Path within the target to identify a specific file/directory/image etc. when scanning just part of the target, and not the entity.
    pub(crate) target_file: String,
    /// The additional information required to resolve which revision of the resource should be scanned.
    pub(crate) target_reference: String,
    /// Describes if a project is currently monitored or it is de-activated.
    pub(crate) status: ProjectStatus, // Useful for skipping inactive projects.
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
            bom: None,
            issues: vec![],
            issues_count: 0,
            target_file,
            target_reference,
            status,
            package_manager,
            inner,
        }
    }

    fn issues(&mut self, issue: Issue) {
        self.issues.push(issue);
        self.issues_count = self.issues.len() as u32;
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
        let mut none_group = Group::none();

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
                    self.build_organization(&mut org).await
                }
            }

            // Add the org to it's group.
            match org.inner.group.clone() {
                None => {
                    none_group.orgs(org);
                },
                Some(snyk_group) => {
                    Self::add_or_append_to_group(&mut registry, org, snyk_group);
                },
            }
        }

        registry.groups(none_group);
        registry.summarize();

        Ok(registry)
    }

    fn add_or_append_to_group(registry: &mut Registry, mut org: Organization, snyk_group: crate::clients::snyk::models::Group) {
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
    }

    async fn build_organization(&self, org: &mut Organization) {
        match self.client.projects(org_id.as_str()).await {
            Ok(projects) => {
                match projects {
                    Some(projects) => {
                        for inner in projects {
                            org.projects(Project::new(inner));
                        }
                    },
                    _ => {},
                }
            },
            Err(e) => {
                debug!("error fetching projects: {}", e);
            }
        };

        for mut project in org.projects.iter_mut() {
            // TODO: Emit metric that we are skipping an supported project type.
            // TODO: Should guard sbom methods, return not supported and trap specific error.
            if !is_sbom_project_type(&project.package_manager) {
                continue;
            }
            self.try_build_sbom_and_issues(org.id.as_str(), &mut project).await?
        }
    }

    async fn try_build_sbom_and_issues(&self, org_id: &str, mut project: &mut &mut Project) {
        match self.sbom_and_issues_by_project(
            org_id,
            project.id.as_str(),
            SbomFormat::CycloneDxJson)
            .await {
            Ok((sbom, issues)) => {
                project.bom = bom;
                match issues {
                    None => {}
                    Some(issues) => {
                        for issue in issues {
                            project.issues(issue);
                        }
                    },
                }
            }
            Err(e) => {debug!("error building sbom and issues - {}", e)}
        }
    }

    pub async fn sbom_and_issues_by_project(&self, org_id: &str, project_id: &str, format: SbomFormat) -> Result<(Option<Bom>, Option<Vec<Issue>>), Error> {
        let sbom = self.sbom(org_id, project_id, format).await?;
        match sbom {
            None => {return Ok((None, None));}
            _ => {}
        }

        let bom = SbomService::parse_cyclonedx_bom(sbom.clone().unwrap(), CycloneDxFormat::Json)?;

        let mut issues = Vec::<Issue>::new();
        let mut purls = SbomService::extract_purls(&bom);

        let purls = match purls {
            None => {
                return Ok((None, None))
            }
            Some(p) => p,
        };

        for purl in purls {
            let snyk_issues = self.client.get_issues(org_id, purl.as_str()).await?;

            match snyk_issues {
                None => {

                }
                Some(snyk_issues) => {
                    for inner in snyk_issues {
                        issues.push(Issue::new(inner));
                    }
                }
            }
        }

        if issues.len() > 0 {
            println!("found {} issues", issues.len());
        }
        Ok((Some(bom), Some(issues)))
    }

    pub async fn sbom(&self, org_id: &str, project_id: &str, format: SbomFormat) -> Result<Option<String>, Error> {
        self.client.sbom(org_id, project_id, format).await
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