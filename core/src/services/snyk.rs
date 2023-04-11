use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::future::Future;
use std::path::Path;
use std::process::Output;
use std::sync::Arc;

use async_trait::async_trait;
use platform::config::from_env;
use platform::mongodb::{Context, Service, Store};
use tracing::debug;

use crate::clients::snyk::client::Client;
use crate::Error;

pub use crate::clients::snyk::client::SbomFormat;
use crate::clients::snyk::models::{Coordinate, OrgV1, ProjectStatus};
use crate::models::cyclonedx::Bom;
use crate::entities::packages::{CycloneDxComponent, Dependency, Package, Purl, PurlSource, Registry, SnykXRef, Unsupported};
use crate::models::sboms::CycloneDxFormat;
use crate::models::teams::Project;
use crate::services::{SbomProvider, SbomService};
use crate::services::snyk::adapters::Issue;

// TODO: Lazy Static or OnceCell this.
const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
    "npm", "swift", "maven", "cocoapods", "composer", "gem", "nuget", "pypi", "hex", "cargo", "generic"];

pub fn is_sbom_project_type(project_type: &str) -> bool {
    SUPPORTED_SBOM_PROJECT_TYPES.contains(&project_type)
}

/// Provides Snyk related data retrieval and analytics capabilities.
#[derive(Debug)]
pub struct SnykService {
    client: Client,
    cx: Context,
}

// Implement mongo Service with type arg for all the types that this service can persist.
impl Service<Bom> for SnykService {
    fn cx(&self) -> &Context{ &self.cx }
}

impl Service<Dependency> for SnykService {
    fn cx(&self) -> &Context{ &self.cx }
}

impl Service<Issue> for SnykService {
    fn cx(&self) -> &Context{ &self.cx }
}

impl Service<Package> for SnykService {
    fn cx(&self) -> &Context{ &self.cx }
}

impl Service<Purl> for SnykService {
    fn cx(&self) -> &Context{ &self.cx }
}

impl Service<SnykXRef> for SnykService {
    fn cx(&self) -> &Context{ &self.cx }
}

impl Service<Unsupported> for SnykService {
    fn cx(&self) -> &Context{ &self.cx }
}

#[async_trait]
impl SbomProvider for SnykService {
    /// Synchronizes a Snyk instance with the Harbor [Registry].
    async fn sync(&self) -> Result<(), Error> {
        // This function intentionally inlines some logic that could be moved into specialized
        // functions so that this process can emit metrics at specific control flow points.
        let mut registry = Registry::new();
        let store = Store::new(&self.cx).await?;

        let packages = store.list().await?;
        registry.packages = packages;

        let orgs = match self.client.orgs().await {
            Ok(o) => {
                match o {
                    None => {
                        let msg = "snyk_service::sync::orgs:orgs_not_found";
                        debug!(msg);
                        return Err(Error::Remote(msg.to_string()));
                    },
                    Some(orgs) => orgs,
                }
            },
            Err(e) => {
                debug!("snyk_service::sync::orgs::{}", e);
                return Err(Error::Remote(e.to_string()));
            }
        };

        for inner in orgs {
            let mut org = adapters::Organization::new(inner);
            // Get the projects for the org.
            match self.projects(&mut org).await {
                Ok(_) => {
                    debug!("snyk_service::sync::projects::org_id::{}::projects_total::{}", org.id, org.projects_total);
                }
                Err(e) => {
                    debug!("snyk_service::sync::org_id::{}::{}", org.id, e);
                    continue;
                }
            }

            match self.sync_sboms(&mut org).await {
                Ok(sboms_total) => {
                    debug!("snyk_service::sync::sync_sboms::org_id::{}::sboms_total::{}", org.id, sboms_total);
                }
                Err(e) => {
                    debug!("snyk_service::sync::sync_sboms::{}", e);
                    continue;
                }
            }

            match self.sync_packages(&mut registry, &mut org).await {
                Ok(_) => {}
                Err(e) => { debug!("snyk_service::sync::sync_packages::{}", e); }
            }
        }

        match self.update_registry(&mut registry).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!("snyk_service::sync::update_registry::{}", e);
                debug!(msg);
                Err(Error::Runtime(msg))
            }
        }

    }
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

    async fn update_registry(&self, registry: &mut Registry) -> Result<(), Error>{
        // TODO: fix these metrics
        for mut package in registry.packages.iter_mut() {
            if package.id.is_empty() {
                match self.insert(package).await {
                    Ok(_) => { continue; }
                    Err(e) => {
                        debug!("snyk_service::sync::store::package::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(package).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("snyk_service::sync::store::package::update::{}", e);
                        continue;
                    }
                }
            }
        }

        for dependency in registry.dependencies.iter_mut() {
            if dependency.id.is_empty() {
                match self.insert(dependency).await {
                    Ok(_) => { continue; }
                    Err(e) => {
                        debug!("snyk_service::sync::store::dependency::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(dependency).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("snyk_service::sync::store::dependency::update::{}", e);
                        continue;
                    }
                }
            }
        }

        for unsupported in registry.unsupported.iter_mut() {
            if unsupported.id.is_empty() {
                match self.insert(unsupported).await {
                    Ok(_) => { continue; }
                    Err(e) => {
                        debug!("snyk_service::sync::store::unsupported::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(unsupported).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("snyk_service::sync::store::unsupported::update::{}", e);
                        continue;
                    }
                }
            }
        }

        Ok(())
    }

    async fn sync_sboms(&self, org: &mut adapters::Organization) -> Result<u32, Error> {
        let mut sboms_total = 0;

        for mut project in org.projects.iter_mut() {
            if project.status == ProjectStatus::Inactive {
                // TODO: Emit Metric.
                debug!("snyk_service::sync_sboms::sync_fn::is_sbom_project_type::inactive::project_name::{}::package_manager::{}", project.name, project.package_manager);
                continue;
            }

            if !is_sbom_project_type(&project.package_manager) {
                // TODO: Emit Metric.
                debug!("snyk_service::sync_sboms::sync_fn::is_sbom_project_type::project_name::{}: package_manager::{}", project.name, project.package_manager);
                // TODO: Handle tracking packages that aren't SBOM targets so that we can profile what the Snyk registry looks like.
                continue;
            }

            let bom = self.client.sbom_raw(
                org.id.as_str(),
                project.id.as_str(),
                SbomFormat::CycloneDxJson)
                .await?;

            let bom = match bom {
                None => {
                    // TODO: Emit Metric.
                    debug!("snyk_service::sync_sboms::sbom::empty::project_name::{}", project.name);
                    continue;
                }
                Some(raw) => {
                    match self.sync_sbom(&raw, project).await {
                        Ok(()) => {
                            // TODO: Emit Metric.
                            debug!("snyk_service::sync_sboms::sync_sbom::success::project_name::{}", project.name);
                        },
                        Err(e) => {
                            // TODO: Emit Metric.
                            debug!("snyk_service::sync_sboms::sync_sbom::failure::project_name::{}: {}", project.name, e);
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
                    debug!("snyk_service::sync_sboms::parse_cyclonedx_bom::project_name::{}: {}", project.name, e);
                }
            }

            sboms_total += 1;
        }

        Ok(sboms_total)
    }

    pub async fn sync_sbom(&self, _raw: &String, project: &mut adapters::Project) -> Result<(), Error> {
        // TODO: Post to v1 API.

        let bom = match project.bom.borrow_mut() {
            None => { return Ok(()); }
            Some(b) => b
        };

        self.insert(bom).await?;

        Ok(())
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
            .for_each(|inner| results.push(adapters::Issue::new(purl.to_string(), inner.clone())));

        Ok(Some(results))
    }

    // TODO: This is primarily for data exploration at this point. Needs more thought to be operationalized.
    pub async fn register_purls(&self) -> Result<(), Error> {
        let mut purls = HashMap::new();
        let store = Store::new(&self.cx).await?;

        let packages = store.list::<Package>().await?;
        for package in packages {
            let component = package.cdx_component.unwrap();
            let package_url = component.purl.clone().unwrap();

            if purls.contains_key(package_url.as_str()) {
                let existing: &mut Purl = purls.get_mut(package_url.as_str()).unwrap();
                existing.merge_snyk_refs(package.xref.snyk);
            } else {
                let (name, version) = Purl::parse(package_url.clone());
                purls.insert(package_url.clone(), Purl {
                    id: package_url,
                    name,
                    version,
                    source: PurlSource::Package,
                    snyk_refs: package.xref.snyk.clone(),
                });
            }
        }

        let dependencies = store.list::<Dependency>().await?;
        for dependency in dependencies {
            let component = dependency.cdx_component.unwrap();
            let package_url = component.purl.clone().unwrap();

            if purls.contains_key(package_url.as_str()) {
                let existing: &mut Purl = purls.get_mut(package_url.as_str()).unwrap();
                existing.merge_snyk_refs(dependency.xref.snyk);
            } else {
                let (name, version) = Purl::parse(package_url.clone());
                purls.insert(package_url.clone(), Purl {
                    id: package_url,
                    name,
                    version,
                    source: PurlSource::Dependency,
                    snyk_refs: dependency.xref.snyk,
                });
            }
        }

        for (package_url, purl) in purls {
            match store.insert(&purl).await {
                Ok(_) => {}
                Err(e) => {
                    debug!("failed to insert purl for {} - {}", package_url, e);
                }
            }
        }

        Ok(())
    }

    pub async fn register_sboms(&self) -> Result<(), Error> {
        let store = Store::new(&self.cx).await?;

        let debug_dir = from_env("DEBUG_DIR").unwrap();
        let sbom_dir = Path::new(&debug_dir).join("sboms");

        for entry in std::fs::read_dir(sbom_dir)
            .map_err(|e| Error::Runtime(format!("snyk_service::register_sboms::read_dir::{}", e)))? {

            let entry = entry
                .map_err(|e| Error::Runtime(format!("snyk_service::register_sboms::entry::{}", e)))?;
            let path = entry.path();

            let metadata = fs::metadata(path.clone())
                .map_err(|e| Error::Runtime(format!("snyk_service::register_sboms::metadata::{}", e)))?;

            if !metadata.is_file() {
                continue;
            }

            let file = fs::read_to_string(path).unwrap();
            let bom = SbomService::parse_cyclonedx_bom(file.as_str(), CycloneDxFormat::Json)?;

            store.insert(&bom).await?;
        }

        Ok(())
    }

    pub async fn register_issues(&self) -> Result<(), Error> {
        //let mut distinct = HashMap::<&str, Vec<SnykXRef>>::new();
        let store = Store::new(&self.cx).await?;
        let purls = store.list::<Purl>().await?;

        for purl in purls.clone() {
            let _raw_purl = purl.id.clone();
            // let raw_purl = raw_purl.as_str();
            // match distinct.get::<&Vec<SnykXRef>>(raw_purl.as_ref()) {
            //     None => { distinct.insert(raw_purl, vec![]);}
            //     Some(refs) => {
            //
            //     }
            // }
        }

        for purl in purls {

            // This is a BUG!!!  Could and probably DO call same purl numerous times.
            //     I need to do some work to make sure I'm calling a distinct set of Purl/Org Id
            //     combos and can map it back to the right project.
            for r in purl.snyk_refs {
                let org_id = r.org_id.clone();

                let issues = match self.issues(org_id.as_str(), purl.id.as_str()).await {
                    Ok(i) => i,
                    Err(e) => {
                        debug!("failed to get issues for purl: {}", purl.id);
                        continue;
                    }
                };

                let issues = match issues {
                    None => { continue; }
                    Some(issues) => issues,
                };

                for issue in issues {
                    match store.insert(&issue).await {
                        Ok(_) => {}
                        Err(e) => {
                            debug!("failed to insert issue for purl: {}", purl.id);
                        }
                    }
                }

            }
        }

        Ok(())
    }
}

mod adapters {
    use platform::mongodb::{MongoDocument, mongo_doc};
    use serde::{Deserialize, Serialize};
    use crate::clients::snyk::models::{CommonIssueModel, ListOrgProjects200ResponseDataInner, OrgV1, ProjectStatus, Severity};
    use crate::entities::packages::{PackageXRef, SnykXRef, Unsupported};
    use crate::models::cyclonedx::Bom;

    mongo_doc!(Issue);

    /// Adapter over a native Snyk Group.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Group {
        pub id: String,
        pub name: String,
    }

    impl Group {
        pub(crate) fn new(inner: crate::clients::snyk::models::Group) -> Self {
            let id = inner.id.clone().unwrap_or("group id not set".to_string());
            let name = inner.name.clone().unwrap_or("group name not set".to_string());

            Self {
                id,
                name,
            }
        }
    }

    /// Adapter over a native Snyk Org.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Organization {
        pub id: String,
        pub name: String,
        pub projects_total: u32,
        pub projects: Vec<Project>,
        pub(crate) inner: OrgV1,
    }

    impl Organization {
        pub(crate) fn new(inner: OrgV1) -> Self {
            let id = inner.id.clone().unwrap_or("org id not set".to_string()).clone();
            let name = inner.name.clone().unwrap_or("org name not set".to_string()).clone();
            Self {
                id,
                name,
                inner,
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
                id: "".to_string(),
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
                purl,
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
    use crate::config::{Environ, environment, mongo_context};
    use super::*;
    use crate::Error;

    fn test_service() -> SnykService {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let cx = mongo_context(Some("core-test"))?;

        let service = SnykService::new(token, cx);
        service
    }

    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_sync() -> Result<(), Error> {
        let service = test_service();
        service.sync().await?;

        Ok(())
    }

    #[async_std::test]
    async fn can_sync_purls() -> Result<(), Error> {
        let service = test_service();

        service.register_purls().await
            .map_err(|e| Error::Enrich(e.to_string()))?;;


        //service.registry_issues(purls).await?;

        Ok(())
    }

    #[async_std::test]
    async fn can_sync_issues() -> Result<(), Error> {
        let service = test_service();

        service.register_issues().await
            .map_err(|e| Error::Enrich(e.to_string()))?;

        Ok(())
    }

    #[async_std::test]
    async fn can_register_sboms() -> Result<(), Error> {
        let service = test_service();

        service.register_sboms().await
            .map_err(|e| Error::Enrich(e.to_string()))?;

        Ok(())
    }

}