use std::future::Future;
use std::process::Output;
use tracing::debug;
use platform::mongodb::{Context, Store};

use crate::clients::snyk::client::Client;
use crate::Error;

pub use crate::clients::snyk::client::SbomFormat;
use crate::clients::snyk::models::{OrgV1, ProjectStatus};
use crate::models::cyclonedx::Bom;
use crate::entities::packages::{CycloneDxComponent, Dependency, Package, Registry};
use crate::models::sboms::CycloneDxFormat;
use crate::services::SbomService;

// TODO: Lazy Static or OnceCell this.
// const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
//     "npm", "nuget", "hex", "pip", "poetry", "rubygems",
//     "maven", "yarn", "yarn-workspace", "composer", "gomodules",
//     "govendor", "golang", "golangdep", "paket",
//     "cocoapods", "cpp", "sbt"];

// TODO: Lazy Static or OnceCell this.
const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
    "npm", "swift", "maven", "cocoapods", "composer", "gem", "nuget", "pypi", "hex", "cargo", "generic"];

pub fn is_sbom_project_type(project_type: &str) -> bool {
    SUPPORTED_SBOM_PROJECT_TYPES.contains(&project_type)
}

/// Provides Snyk related data retrieval and analytics capabilities.
pub struct SnykService {
    client: Client,
    cx: Context,
}

impl SnykService {
    /// Factory method to create new instances of a SnykService.
    pub fn new(token: String, cx: Context) -> Self {
        let client = Client::new(token);
        Self {
            client,
            cx,
        }
    }

    pub async fn scan_and_sync_registry<F, Fut>(&self, sync_fn: F) -> Result<Registry, Error>
    where
        F: Fn(String, String) -> Fut + Copy,
        Fut: Future<Output=Result<(), Error>> {

        let mut registry = Registry::new();
        let store = Store::new(&self.cx).await?;

        let packages = store.list().await?;
        registry.packages = packages;

        let orgs = match self.orgs().await {
            Ok(o) => {
                match o {
                    None => {
                        let msg = "snyk_service::scan_and_sync_registry::orgs: no orgs found";
                        debug!(msg);
                        return Err(Error::Remote(msg.to_string()));
                    },
                    Some(orgs) => orgs,
                }
            },
            Err(e) => {
                debug!("snyk_service::scan_and_sync_registry::orgs::{}", e);
                return Err(Error::Remote(e.to_string()));
            }
        };

        for inner in orgs {
            let mut org = adapters::Organization::new(inner);
            // Get the projects for the org.
            match self.projects(&mut org).await {
                Ok(_) => {
                    debug!("snyk_service::scan_and_sync_registry::projects::org_id::{}::projects_total::{}", org.id, org.projects_total);
                }
                Err(e) => {
                    debug!("snyk_service::scan_and_sync_registry::org_id::{}::{}", org.id, e);
                    continue;
                }
            }

            match self.sync_sboms(&mut org, sync_fn).await {
                Ok(sboms_total) => {
                    debug!("snyk_service::scan_and_sync_registry::sync_sboms::org_id::{}::sboms_total::{}", org.id, sboms_total);
                }
                Err(e) => {
                    debug!("snyk_service::scan_and_sync_registry::sync_sboms::{}", e);
                    continue;
                }
            }

            match self.sync_packages(&mut registry, &mut org).await {
                Ok(_) => {}
                Err(e) => { debug!("snyk_service::scan_and_sync_registry::sync_packages::{}", e); }
            }
        }

        Self::sync_registry(&mut registry, store).await;

        Ok(registry)
    }

    async fn sync_registry(registry: &mut Registry, store: Store) {
        for package in &registry.packages {
            if package.id.is_empty() {
                match store.insert(package).await {
                    Ok(_) => { continue; }
                    Err(e) => {
                        debug!("snyk_service::scan_and_sync_registry::store::package::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match store.update(package).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("snyk_service::scan_and_sync_registry::store::package::update::{}", e);
                        continue;
                    }
                }
            }
        }

        for dependency in &registry.dependencies {
            if dependency.id.is_empty() {
                match store.insert(dependency).await {
                    Ok(_) => { continue; }
                    Err(e) => {
                        debug!("snyk_service::scan_and_sync_registry::store::dependency::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match store.update(dependency).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("snyk_service::scan_and_sync_registry::store::dependency::update::{}", e);
                        continue;
                    }
                }
            }
        }

        for unsupported in &registry.unsupported {
            if unsupported.id.is_empty() {
                match store.insert(unsupported).await {
                    Ok(_) => { continue; }
                    Err(e) => {
                        debug!("snyk_service::scan_and_sync_registry::store::unsupported::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match store.update(unsupported).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("snyk_service::scan_and_sync_registry::store::unsupported::update::{}", e);
                        continue;
                    }
                }
            }
        }
    }

    async fn sync_sboms<F, Fut>(&self, org: &mut adapters::Organization, sync_fn: F) -> Result<u32, Error>
        where
        F: Fn(String, String) -> Fut + Copy,
        Fut: Future<Output=Result<(), Error>> {

        let mut sboms_total = 0;

        for mut project in org.projects.iter_mut() {
            if project.status == ProjectStatus::Inactive {
                // TODO: Emit Metric.
                debug!("sync_sboms::sync_fn::is_sbom_project_type::inactive::project_name::{}::package_manager::{}", project.name, project.package_manager);
                continue;
            }

            if !is_sbom_project_type(&project.package_manager) {
                // TODO: Emit Metric.
                debug!("sync_sboms::sync_fn::is_sbom_project_type::project_name::{}: package_manager::{}", project.name, project.package_manager);
                // TODO: Handle tracking packages that aren't SBOM targets so that we can profile what the Snyk registry looks like.
                continue;
            }

            let bom = self.sbom(
                org.id.as_str(),
                project.id.as_str(),
                SbomFormat::CycloneDxJson)
                .await?;

            let bom = match bom {
                None => {
                    // TODO: Emit Metric.
                    debug!("sync_sboms::sbom::empty::project_name::{}", project.name);
                    continue;
                }
                Some(raw) => {
                    match sync_fn(raw.clone(), project.name.clone()).await {
                        Ok(()) => {
                            // TODO: Emit Metric.
                            debug!("sync_sboms::sync_fn::success::project_name::{}", project.name);
                        },
                        Err(e) => {
                            // TODO: Emit Metric.
                            debug!("sync_sboms::sync_fn::failure::project_name::{}: {}", project.name, e);
                            continue;
                        }
                    };
                    raw
                },
            };

            match SbomService::parse_cyclonedx_bom(bom.as_str(), CycloneDxFormat::Json) {
                Ok(bom) => { project.bom = Some(bom); }
                Err(e) => {
                    // TODO: Emit Metric.
                    debug!("sync_sboms::parse_cyclonedx_bom::project_name::{}: {}", project.name, e);
                }
            }

            sboms_total += 1;
        }

        Ok(sboms_total)
    }

    async fn sync_packages(&self, registry: &mut Registry, org: &mut adapters::Organization) -> Result<(), Error> {
        let shadow_org = org.clone();
        for project in org.projects.iter_mut() {
            if project.status == ProjectStatus::Inactive {
                continue;
            }

            if !is_sbom_project_type(&project.package_manager) {
                registry.unsupported(project.to_unsupported(&shadow_org));
                continue;
            }

            let bom = match &project.bom {
                None => { continue; }
                Some(b) => b,
            };

            let component = match &bom.metadata {
                None => {
                    debug!("sync_packages: {} missing metadata", project.name);
                    continue;
                }
                Some(metadata) => {
                    match &metadata.component {
                        None => {
                            debug!("sync_packages: {} missing component", project.name);
                            continue;
                        }
                        Some(c) => { c }
                    }
                }
            };

            registry.cyclonedx_packages(Package {
                id: "".to_string(),
                manager: Some(project.package_manager.clone()),
                cdx_component: Some(CycloneDxComponent::from(*component.to_owned())),
                xref: project.to_package_xref(&shadow_org),
            });

            match &bom.components {
                None => {
                    debug!("sync_packages: {} no components", project.name);
                    continue;
                }
                Some(components) => {
                    for component in components {
                        registry.cyclonedx_dependencies(Dependency{
                            id: "".to_string(),
                            cdx_component: Some(CycloneDxComponent::from(component.to_owned())),
                            xref: project.to_package_xref(&shadow_org),
                        })
                    }
                }
            }
        }

        Ok(())
    }

    /// Tries to retrieve [Projects] for an [Organization].
    async fn projects(&self, org: &mut adapters::Organization) -> Result<(), Error>{
        let projects = match self.client.projects(org.id.as_str()).await {
            Ok(projects) => projects,
            Err(e) => {
                // TODO: Emit metric
                debug!("error fetching projects for org {} - {}: {}", org.name, org.id, e);
                return Err(Error::Remote(format!("error fetching projects for org {} - {}: {}", org.name, org.id, e)));
            }
        };

        match projects {
            Some(projects) => {
                projects
                    .iter()
                    .for_each(|inner| {
                        org.projects(adapters::Project::new(inner.clone()));
                    });
            },
            _ => {
                // TODO: Emit metric
                debug!("no projects for org {} - {}", org.name, org.id);
            },
        };

        Ok(())
    }

    async fn orgs(&self) -> Result<Option<Vec<OrgV1>>, Error> {
        match self.client.orgs().await? {
            None => {
                return Err(Error::Config("no organizations for token".to_string()));
            }
            Some(o) => Ok(Some(o)),
        }
    }

    pub async fn sbom(&self, org_id: &str, project_id: &str, format: SbomFormat) -> Result<Option<String>, Error> {
        self.client.sbom_raw(org_id, project_id, format).await
    }

    pub async fn sbom_issues(&self, org_id: &str, bom: &Bom) -> Result<Option<Vec<adapters::Issue>>, Error> {
        let mut issues = Vec::<adapters::Issue>::new();
        let mut purls = SbomService::extract_purls(&bom);

        let purls = match purls {
            None => {
                return Ok(None)
            }
            Some(p) => p,
        };

        for purl in purls {
            match self.issues(org_id, purl.as_str()).await? {
                None => {}
                Some(_) => {}
            }
        }

        if issues.is_empty() {
            return Ok(None);
        }

        Ok(Some(issues))
    }

    pub async fn issues(&self, org_id: &str, purl: &str) -> Result<Option<Vec<adapters::Issue>>, Error> {
        let issues = match self.client.get_issues(org_id, purl).await {
            Ok(issues) => issues,
            Err(e) => {
                return Err(Error::Remote(format!("snyk::issues: purl - {} - {}", purl, e)));
            }
        };

        let issues = match issues {
            None => { return Ok(None); },
            Some(issues) => issues,
        };

        if issues.is_empty() { return Ok(None); }

        let mut results = vec![];
        issues
            .iter()
            .for_each(|inner| results.push(adapters::Issue::new(inner.clone())));

        Ok(Some(results))
    }
}

mod adapters {
    use serde::{Deserialize, Serialize};
    use crate::clients::snyk::models::{CommonIssueModel, ListOrgProjects200ResponseDataInner, OrgV1, ProjectStatus, Severity};
    use crate::entities::packages::{PackageXRef, SnykXRef, Unsupported};
    use crate::models::cyclonedx::Bom;

    /// Adapter over a native Snyk Group.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Group {
        pub id: String,
        pub name: String,
        pub orgs_total: u32,
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
                orgs_total: 0,
            }
        }

        pub(crate) fn orgs(&mut self, org: Organization) {
            self.orgs.push(org);
            self.orgs_total = self.orgs.len() as u32;
        }

        pub(crate) fn none() -> Self {
             Self {
                inner: Default::default(),
                id: "none-group".to_string(),
                name: "Orgs with no group set".to_string(),
                orgs: vec![],
                orgs_total: 0,
            }
        }
    }

    /// Adapter over a native Snyk Org.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Organization {
        pub id: String,
        pub name: String,
        pub projects_total: u32,
        pub issues_total: u32,
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
                issues_total: 0,
                issues: vec![],
                projects: vec![],
                projects_total: 0,
            }
        }

        pub(crate) fn projects(&mut self, project: Project) {
            self.projects.push(project);
            self.projects_total = self.projects.len() as u32;
        }

        pub(crate) fn group(&self) -> Group {
            match self.inner.group.clone() {
                None => {
                    Group {
                        id: "not set".to_string(),
                        name: "not set".to_string(),
                        orgs_total: 0,
                        inner: Default::default(),
                        orgs: vec![]
                    }
                },
                Some(inner) => {Group::new(inner)}
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
        pub issues_total: u32,
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
                issues_total: 0,
                target_file,
                target_reference,
                status,
                package_manager,
                inner,
            }
        }

        fn issues(&mut self, issue: Issue) {
            self.issues.push(issue);
            self.issues_total = self.issues.len() as u32;
        }

        pub(crate) fn to_unsupported(&self, org: &Organization) -> Unsupported {
            Unsupported {
                    id: "".to_string(),
                    name: self.name.clone(),
                    manager: Some(self.package_manager.clone()),
                    xref: PackageXRef {
                        fisma: vec![],
                        codebase: vec![],
                        product: vec![],
                        snyk: vec![self.to_snyk_xref(org)],
                    },
                }
        }

        pub(crate) fn to_package_xref(&self, org: &Organization) -> PackageXRef {
            PackageXRef {
                fisma: vec![],
                codebase: vec![],
                product: vec![],
                snyk: vec![self.to_snyk_xref(org)],
            }
        }

        pub(crate) fn to_snyk_xref(&self, org: &Organization) -> SnykXRef {
            SnykXRef{
                org_id: org.id.clone(),
                org_name: org.name.clone(),
                group_id: org.group().id,
                group_name: org.group().name,
                project_id: self.id.clone(),
                project_name: self.name.clone(),
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


}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use platform::mongodb::Context;
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_build_registry() -> Result<(), Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let cx = Context{
            host: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            port: 0,
            db_name: "".to_string(),
            key_name: "".to_string(),
        };

        let service = SnykService::new(token, cx);
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