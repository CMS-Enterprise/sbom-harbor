use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use async_trait::async_trait;
use platform::config::{from_env, sdk_config_from_env};
use platform::mongodb::{Context, Service, Store};
use platform::persistence::s3;
use tracing::debug;
use crate::entities::cyclonedx::{Bom, Vulnerability};


use crate::entities::packages::{
    CdxComponent, Dependency, DependencyKind, Package, Purl, Source, Registry, SnykXRef,
    Unsupported, Vulnerability,
};
use crate::entities::packages::Source::Dependency;
use crate::entities::sboms::{CdxFormat, Spec};
use crate::Error;
use crate::services::cyclonedx::models::{Bom, Component};
use crate::services::cyclonedx::{bom_eq, bom_parse, bom_purl, extract_purls};
use crate::services::sbom::{SbomProvider, SbomService};
use crate::services::snyk::adapters::{Issue, Organization};
use crate::services::{SbomProvider, SbomService};

use crate::models::sbom::Spec;
use crate::services::sboms::{SbomProvider, SbomService};
use crate::services::snyk::client::{Client, SbomFormat};
use crate::services::snyk::service::adapters::Organization;

// TODO: Lazy Static or OnceCell this.
const SUPPORTED_SBOM_PROJECT_TYPES: &'static [&'static str] = &[
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
    client: Client,
    cx: Context,
}

// Implement mongo Service with type arg for all the types that this service can persist.
impl Service<Bom> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Dependency> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Vulnerability> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Package> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Purl> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<SnykXRef> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Unsupported> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

#[async_trait]
impl SbomProvider for SnykService {
    /// Synchronizes a Snyk instance with the Harbor [Registry].
    async fn sync(&self) -> Result<(), Error> {
        // This function intentionally inlines some logic that could be moved into specialized
        // functions so that this process can emit metrics at specific control flow points.
        let mut registry = Registry::new();

        // Build set of orgs and projects
        let mut orgs = match self.build_orgs().await {
            Ok(o) => {
                debug!("snyk_service::sync::orgs_total::{}", o.len());
                o
            }
            Err(e) => {
                return Err(Error::Snyk(
                    format!("snyk_service::sync::{}", e).to_string(),
                ));
            }
        };

        for org in orgs.iter_mut() {
            match self.sync_sboms(org).await {
                Ok(sboms_total) => {
                    debug!(
                        "snyk_service::sync::sync_sboms::org_id::{}::sboms_total::{}",
                        org.id, sboms_total
                    );
                }
                Err(e) => {
                    debug!("snyk_service::sync::sync_sboms::{}", e);
                    continue;
                }
            }
        }

        for org in orgs.iter_mut() {
            match self.sync_packages(&mut registry, org).await {
                Ok(_) => {}
                Err(e) => {
                    debug!("snyk_service::sync::sync_packages::{}", e);
                }
            }
        }

        match self.update_registry(&mut registry).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!("snyk_service::sync::update_registry::{}", e);
                debug!(msg);
                Err(Error::Snyk(msg))
            }
        }
    }
}

impl SnykService {
    /// Factory method to create new instance of type.
    pub fn new(token: String, cx: Context) -> Self {
        let client = Client::new(token);
        Self { client, cx }
    }

    /// Builds the Org and Project adapters from native Snyk API types.
    async fn build_orgs(&self) -> Result<Vec<Organization>, Error> {
        let mut orgs = match self.orgs().await {
            Ok(o) => o,
            Err(e) => {
                let msg = format!("build_orgs::orgs::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg));
            }
        };

        // Get projects for each org.
        for org in orgs.iter_mut() {
            // Get the projects for the org.
            match self.projects(org).await {
                Ok(_) => {}
                Err(e) => {
                    debug!("build_orgs::projects::{}", e);
                    continue;
                }
            }
        }

        if orgs.is_empty() {
            return Err(Error::Snyk("build_orgs::orgs_empty".to_string()));
        }

        Ok(orgs)
    }

    /// Retrieves orgs from the Snyk API.
    async fn orgs(&self) -> Result<Vec<Organization>, Error> {
        match self.client.orgs().await {
            Ok(orgs) => match orgs {
                None => {
                    return Err(Error::Snyk("orgs_not_found".to_string()));
                }
                Some(orgs) => {
                    if orgs.is_empty() {
                        return Err(Error::Snyk("orgs_empty".to_string()));
                    }

                    let mut result = vec![];

                    orgs.into_iter().for_each(|inner| {
                        result.push(Organization::new(inner));
                    });

                    Ok(result)
                }
            },
            Err(e) => {
                return Err(Error::Snyk(e.to_string()));
            }
        }
    }

    /// Retrieves [Projects] for an [Organization] from the Snyk API.
    async fn projects(&self, org: &mut Organization) -> Result<(), Error> {
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

                projects.iter().for_each(|inner| {
                    org.projects(adapters::Project::new(inner.clone()));
                });
            }
        }

        Ok(())
    }

    /// Transaction script for saving sync results to data store.
    async fn update_registry(&self, registry: &mut Registry) -> Result<(), Error> {
        // TODO: fix these metrics
        for package in registry.packages.iter_mut() {
            if package.id.is_empty() {
                match self.insert(package).await {
                    Ok(_) => {
                        continue;
                    }
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
                    Ok(_) => {
                        continue;
                    }
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
                    Ok(_) => {
                        continue;
                    }
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

    async fn sync_sboms(&self, org: &mut Organization) -> Result<u32, Error> {
        let mut sboms_total = 0;

        for mut project in org.projects.iter_mut() {
            if project.status == ProjectStatus::Inactive {
                // TODO: Emit Metric.
                debug!("sync_sboms::sync_fn::is_sbom_project_type::inactive::project_name::{}::package_manager::{}", project.name, project.package_manager);
                continue;
            }

            if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
                // TODO: Emit Metric.
                debug!("sync_sboms::sync_fn::is_sbom_project_type::project_name::{}::package_manager::{}", project.name, project.package_manager);
                // TODO: Handle tracking packages that aren't SBOM targets so that we can profile what the Snyk registry looks like.
                continue;
            }

            let raw = match self
                .client
                .sbom_raw(
                    org.id.as_str(),
                    project.id.as_str(),
                    SbomFormat::CycloneDxJson,
                )
                .await
            {
                Ok(bom) => bom,
                Err(e) => {
                    debug!(
                        "sync_sboms::sbom_raw::project_name::{}::package_manager::{}::{}",
                        project.name, project.package_manager, e
                    );
                    continue;
                }
            };

            let raw = match raw {
                None => {
                    // TODO: Emit Metric.
                    debug!("sync_sboms::sbom::empty::project_name::{}", project.name);
                    continue;
                }
                Some(raw) => raw,
            };

            let mut bom: Bom = match bom_parse(raw.as_str(), CdxFormat::Json) {
                Ok(bom) => bom,
                Err(e) => {
                    // TODO: Emit Metric.
                    debug!(
                        "sync_sboms::parse_cyclonedx_bom::project_name::{}: {}",
                        project.name, e
                    );
                    continue;
                }
            };

            match self.sync_sbom(&raw, &mut bom).await {
                Ok(()) => {
                    // TODO: Emit Metric.
                    debug!(
                        "sync_sboms::sync_sbom::success::project_name::{}",
                        project.name
                    );
                }
                Err(e) => {
                    // TODO: Emit Metric.
                    debug!(
                        "sync_sboms::sync_sbom::failure::project_name::{}: {}",
                        project.name, e
                    );
                    continue;
                }
            };

            project.bom = Some(bom);
            sboms_total += 1;
        }

        Ok(sboms_total)
    }

    /// Stores a raw SBOM. Uses parsed instance to check if identical copy is already stored.
    pub async fn sync_sbom(&self, raw: &String, candidate: &mut Bom) -> Result<(), Error> {
        // Validate that sbom has or has not changed.
        let purl = match bom_purl(candidate) {
            Some(p) => p,
            None => {
                return Err(Error::Snyk("sync_sbom::purl_not_set".to_string()));
            }
        };

        let mut boms: Vec<Bom> = self
            .query(HashMap::from([("metadata.component.purl", purl.as_str())]))
            .await?;

        let exists = boms
            .iter_mut()
            .any(|b| bom_eq(b, candidate).expect("sync_bom::eq_cyclonedx_bom::error"));

        if !exists {
            self.insert(candidate).await?;
        }

        let sbom_service = SbomService::new(self.cx.clone(), ());

        match sbom_service.upload(purl, raw).await {}

        Ok(())
    }

    async fn sync_packages(
        &self,
        registry: &mut Registry,
        org: &mut Organization,
    ) -> Result<(), Error> {
        let projects = &org.projects;

        for project in projects.iter() {
            if project.status == ProjectStatus::Inactive {
                continue;
            }

            if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
                registry.unsupported(project.to_unsupported(&org));
                continue;
            }

            let bom = match &project.bom {
                None => {
                    continue;
                }
                Some(b) => b,
            };

            let component = match &bom.metadata {
                None => {
                    debug!("sync_packages: {} missing metadata", project.name);
                    continue;
                }
                Some(metadata) => match &metadata.component {
                    None => {
                        debug!("sync_packages: {} missing component", project.name);
                        continue;
                    }
                    Some(c) => c,
                },
            };

            let snyk_ref = project.to_package_xref(&org);

            let package = Package::from_bom(
                bom,
                Some(Spec::Cdx(CdxFormat::Json)),
                Some(project.package_manager.clone()),
                snyk_ref: snyk_ref.clone());

            registry.cyclonedx_packages(package);

            match &bom.components {
                None => {
                    debug!("sync_packages: {} no components", project.name);
                    continue;
                }
                Some(components) => {
                    for component in components {
                        let dependency = Dependency::from_direct(component);
                        match registry.cyclonedx_dependencies(dependency) {
                            Ok(_) => {}
                            Err(e) => {
                                debug!("sync_packages: add direct dependency - {}", e);
                                continue;
                            }
                        }
                    }
                }
            }

            match &bom.dependencies {
                None => {

                }
                Some(dependencies) => {
                    for dep in dependencies {
                        let dependency = Dependency::from_transitive(dep);
                        match registry.cyclonedx_dependencies(dependency) {
                            Ok(_) => {}
                            Err(e) => {
                                debug!("sync_packages: add transitive dependency - {}", e);
                                continue;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn sbom_issues(
        &self,
        org_id: &str,
        bom: &Bom,
    ) -> Result<Option<Vec<adapters::Issue>>, Error> {
        let issues = Vec::<adapters::Issue>::new();
        let purls = extract_purls(&bom);

        let purls = match purls {
            None => return Ok(None),
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

    pub async fn issues(
        &self,
        org_id: &str,
        purl: &str,
    ) -> Result<Option<Vec<adapters::Issue>>, Error> {
        let issues = match self.client.get_issues(org_id, purl).await {
            Ok(issues) => issues,
            Err(e) => {
                return Err(Error::Snyk(format!(
                    "snyk::issues: purl - {} - {}",
                    purl, e
                )));
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
        issues
            .iter()
            .for_each(|inner| results.push(adapters::Issue::new(purl.to_string(), inner.clone())));

        Ok(Some(results))
    }

    // TODO: This is primarily for data exploration at this point. Needs more thought to be operationalized.
    pub async fn register_purls(&self) -> Result<(), Error> {
        let mut purls = HashMap::new();

        let packages: Vec<Package> = self.list().await?;
        for package in packages {
            let component = package.cdx_component.unwrap();
            let package_url = component.purl.clone().unwrap();

            if purls.contains_key(package_url.as_str()) {
                let existing: &mut Purl = purls.get_mut(package_url.as_str()).unwrap();
                existing.merge_snyk_refs(package.xref.snyk);
            } else {
                let (name, version) = Purl::parse(package_url.clone());
                purls.insert(
                    package_url.clone(),
                    Purl {
                        id: package_url,
                        name,
                        version,
                        source: Source::Package,
                        snyk_refs: package.xref.snyk.clone(),
                    },
                );
            }
        }

        let dependencies: Vec<Dependency> = self.list().await?;
        for dependency in dependencies {
            let component = dependency.cdx_component.unwrap();
            let package_url = component.purl.clone().unwrap();

            if purls.contains_key(package_url.as_str()) {
                let existing: &mut Purl = purls.get_mut(package_url.as_str()).unwrap();
                existing.merge_snyk_refs(dependency.xref.snyk);
            } else {
                let (name, version) = Purl::parse(package_url.clone());
                purls.insert(
                    package_url.clone(),
                    Purl {
                        id: package_url,
                        name,
                        version,
                        source: Source::Dependency,
                        snyk_refs: dependency.xref.snyk,
                    },
                );
            }
        }

        for (package_url, mut purl) in purls {
            match self.insert(&mut purl).await {
                Ok(_) => {}
                Err(e) => {
                    debug!("failed to insert purl for {} - {}", package_url, e);
                }
            }
        }

        Ok(())
    }

    pub async fn register_issues(&self) -> Result<(), Error> {
        // let mut distinct = HashMap::<&str, Vec<SnykXRef>>::new();
        let purls: Vec<Purl> = self.list().await?;

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

                let mut issues = match issues {
                    None => {
                        continue;
                    }
                    Some(issues) => issues,
                };

                for issue in issues.iter_mut() {
                    match self.insert(issue).await {
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
    use crate::clients::snyk::models::{
        CommonIssueModel, ListOrgProjects200ResponseDataInner, OrgV1, ProjectStatus, Severity,
    };
    use crate::entities::packages::{PackageXRef, SnykXRef, Unsupported};
    use crate::models::cyclonedx::Bom;
    use crate::services::cyclonedx::models::Bom;
    use platform::mongodb::{mongo_doc, MongoDocument};
    use serde::{Deserialize, Serialize};
    use crate::entities::cyclonedx::{Bom, Severity};

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
            let name = inner
                .name
                .clone()
                .unwrap_or("group name not set".to_string());

            Self { id, name }
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
                None => Group {
                    id: "not set".to_string(),
                    name: "not set".to_string(),
                },
                Some(inner) => Group::new(inner),
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
            SnykXRef {
                id: "".to_string(),
                active: self.status == ProjectStatus::Active,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::mongo_context;
    use crate::Error;

    fn test_service() -> Result<SnykService, Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let cx = mongo_context(Some("core-test"))?;

        let service = SnykService::new(token, cx);
        Ok(service)
    }

    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_sync() -> Result<(), Error> {
        let service = test_service()?;
        service.sync().await?;

        Ok(())
    }

    #[async_std::test]
    async fn can_sync_purls() -> Result<(), Error> {
        let service = test_service()?;

        service
            .register_purls()
            .await
            .map_err(|e| Error::Snyk(e.to_string()))?;

        //service.registry_issues(purls).await?;

        Ok(())
    }

    #[async_std::test]
    async fn can_sync_issues() -> Result<(), Error> {
        let service = test_service()?;

        service
            .register_issues()
            .await
            .map_err(|e| Error::Snyk(e.to_string()))?;

        Ok(())
    }

    #[async_std::test]
    async fn can_register_sboms() -> Result<(), Error> {
        let service = test_service()?;

        service
            .register_sboms()
            .await
            .map_err(|e| Error::Snyk(e.to_string()))?;

        Ok(())
    }
}
