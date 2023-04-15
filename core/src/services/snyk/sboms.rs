use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::{Dependency, Package, Purl, SourceKind};
use crate::entities::sboms::{CdxFormat, Sbom, Spec};
use crate::services::packages::service::PackageService;
use crate::services::sboms::{SbomProvider, SbomService};
use crate::services::snyk::adapters::{Organization, Project};
use crate::services::snyk::client::SbomFormat;
use crate::services::snyk::service::SUPPORTED_SBOM_PROJECT_TYPES;
use crate::services::snyk::SnykService;
use crate::Error;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use std::future::Future;
use tracing::debug;

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

impl Service<Unsupported> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

#[async_trait]
impl SbomProvider for SnykService {
    /// Synchronizes a Snyk instance with the Harbor [Registry].
    async fn sync(&self) -> Result<(), Error> {
        let mut sync_set = ChangeSet::new();

        // Populate the SyncSet
        match self.build_sync_set(sync_set).await {
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

        match self.commit(sync_set) {}

        Ok(())
    }
}

impl SnykService {
    /// Builds the Packages and Dependencies from adapters for the native Snyk API types.
    async fn build_change_set(&self, change_set: &mut ChangeSet) -> Result<(), Error> {
        let mut projects = match self.gather_projects().await {
            Ok(p) => p,
            Err(e) => {
                let msg = format!("build_sync_set::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg));
            }
        };

        if projects.is_empty() {
            return Err(Error::Snyk("no_projects".to_string()));
        }

        for project in projects.iter() {
            match self.process_project(change_set, project) {
                Ok(()) => {
                    // TODO: Emit Metric
                    debug!("process_project::success");
                }
                Err(e) => {
                    // TODO: Emit Metric
                    debug!("build_change_set::{}", e);
                }
            }
        }

        Ok(())
    }

    /// Gathers all projects across all orgs so that index can be analyzed linearly.
    async fn gather_projects(&self) -> Result<Vec<Project>, Error> {
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
            match self.projects(org).await {
                Ok(mut p) => {
                    projects.append(p.borrow_mut());
                }
                Err(e) => {
                    debug!("gather_projects::projects::{}", e);
                    continue;
                }
            }
        }

        Ok(projects)
    }

    async fn process_project(
        &self,
        &mut change_set: ChangeSet,
        project: &Project,
    ) -> Result<(), Error> {
        if project.status == ProjectStatus::Inactive {
            let msg = "process_project::inactive";
            debug!(msg);
            return Err(Error::Snyk(msg.to_string()));
        }

        if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
            let msg = format!("process_project::unsupported::{}", &project.package_manager);
            debug!(msg);
            return Err(Error::Snyk(msg.to_string()));
        }

        let snyk_ref = project.to_snyk_xref();

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
                let msg = format!("process_project::sbom_raw::package_manager::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg));
            }
        };

        let raw = match raw {
            None => {
                // TODO: Emit Metric.
                let msg = "process_project::sbom::none";
                debug!(msg);
                return Err(Error::Snyk(msg.to_string()));
            }
            Some(raw) => {
                if raw.is_empty() {
                    let msg = "process_project::sbom::empty";
                    debug!(msg);
                    return Err(Error::Snyk(msg.to_string()));
                }
                raw
            }
        };

        match self.save_sbom(&raw, &mut bom).await {
            Ok(()) => {
                // TODO: Emit Metric.
                debug!("process_project::save_sbom::success");
            }
            Err(e) => {
                // TODO: Emit Metric.
                let msg = format!("process_project::save_sbom::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg.to_string()));
            }
        };

        let mut bom: Bom = match Bom::parse(raw.as_str(), CdxFormat::Json) {
            Ok(bom) => bom,
            Err(e) => {
                // TODO: Emit Metric.
                let msg = format!("process_project::bom::parse::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg.to_string()));
            }
        };

        let package = Package::from_bom(
            &bom,
            Some(Spec::Cdx(CdxFormat::Json)),
            Some(project.package_manager.clone()),
            Some(snyk_ref.clone()),
        )?;

        change_set.package(package);

        let package_ref = match bom.component() {
            None => {
                let msg = "process_project::bom_component_none";
                debug!(msg);
                return Err(Error::Snyk(msg.to_string()));
            }
            Some(component) => {
                change_set.purl(Purl::from_snyk(
                    &component,
                    SourceKind::Package,
                    snyk_ref.clone(),
                ));

                component.purl
            }
        };

        let package_ref = match package_ref {
            None => {
                let msg = "process_project::package_ref_none";
                debug!(msg);
                return Err(Error::Snyk(msg.to_string()));
            }
            Some(p) => {
                if p.is_empty() {
                    let msg = "process_project::package_ref_empty";
                    debug!(msg);
                    return Err(Error::Snyk(msg.to_string()));
                }
                p
            }
        };

        let components = match bom.components {
            None => {
                return Ok(());
            }
            Some(components) => components,
        };

        for component in components {
            change_set.purl(Purl::from_snyk(
                &component,
                SourceKind::Dependency,
                snyk_ref.clone(),
            ));

            change_set.dependency(Dependency::from_snyk(
                &component,
                package_ref.clone(),
                Some(project.package_manager.clone()),
                snyk_ref.clone(),
            ));
        }

        Ok(())
    }

    async fn register_purls(&self) -> Result<(), Error> {
        let mut purls = HashMap::new();

        let packages: Vec<Package> = self.list().await?;
        for package in packages {
            let component = package.cdx.unwrap();
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
                        purl: "".to_string(),
                        name,
                        version,
                        source: SourceKind::Package,
                        findings: None,
                        snyk_refs: package.xref.snyk.clone(),
                    },
                );
            }
        }

        let dependencies: Vec<Dependency> = self.list().await?;
        for dependency in dependencies {
            let component = dependency.cdx.unwrap();
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
                        purl: "".to_string(),
                        name,
                        version,
                        source: SourceKind::Dependency,
                        findings: None,
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

    /// Stores a raw SBOM. Uses parsed instance to check if identical copy is already stored.
    pub async fn save_sbom(&self, raw: &String, candidate: &mut Bom) -> Result<(), Error> {
        // Validate that sbom has or has not changed.
        let purl = match candidate.purl() {
            Some(p) => p,
            None => {
                return Err(Error::Snyk("sync_sbom::purl_not_set".to_string()));
            }
        };

        let mut boms: Vec<Bom> = self
            .query(HashMap::from([("metadata.component.purl", purl.as_str())]))
            .await?;

        let exists = boms.iter().any(|bom| {
            bom.eq(candidate)
                .expect("sync_bom::eq_cyclonedx_bom::error")
        });

        if !exists {
            self.insert(candidate).await?;
        }

        let sbom_service = SbomService::new(self.cx.clone(), ());

        match sbom_service.upload(purl, raw).await {}

        Ok(())
    }

    /// Transaction script for saving sync results to data store.
    async fn update_registry(&self, change_set: &mut ChangeSet) -> Result<(), Error> {
        // TODO: fix these metrics
        for package in change_set.packages.iter_mut() {
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

        for dependency in change_set.dependencies.iter_mut() {
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

        for unsupported in change_set.unsupported.iter_mut() {
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
}
