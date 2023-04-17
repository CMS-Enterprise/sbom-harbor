use crate::entities::cyclonedx::{Bom, Component, Hash};
use crate::entities::packages::{ComponentKind, Dependency, Package, Purl, Unsupported};
use crate::entities::sboms::{CdxFormat, Sbom, SbomSource, Source, Spec};
use crate::services::packages::service::PackageService;
use crate::services::sboms::{SbomProvider, SbomService};
use crate::services::snyk::adapters::{Organization, Project};
use crate::services::snyk::changeset::ChangeSet;
use crate::services::snyk::client::models::ProjectStatus;
use crate::services::snyk::service::SUPPORTED_SBOM_PROJECT_TYPES;
use crate::services::snyk::{SnykRef, SnykService, SNYK_DISCRIMINATOR};
use crate::Error;

use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::snyk::client::client::SbomFormat;
use async_trait::async_trait;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use std::future::Future;
use std::net::ToSocketAddrs;
use tracing::debug;

// Implement mongo Service with type arg for all the types that this service can persist.

impl Service<Project> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

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
        let mut change_set = ChangeSet::new();

        // Populate the ChangeSet
        match self.build_change_set(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(
                    format!("snyk_service::sync::{}", e).to_string(),
                ));
            }
        };

        match self.commit(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                return Err(Error::Snyk(
                    format!("snyk_service::sync::{}", e).to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl SnykService {
    /// Builds the Packages and Dependencies from adapters for the native Snyk API types.
    pub(crate) async fn build_change_set(&self, change_set: &mut ChangeSet) -> Result<(), Error> {
        let mut projects = match self.gather_projects().await {
            Ok(p) => p,
            Err(e) => {
                let msg = format!("build_change_set::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg));
            }
        };

        if projects.is_empty() {
            return Err(Error::Snyk("build_change_set::no_projects".to_string()));
        }

        for project in projects.iter_mut() {
            match self.process_project(change_set, project).await {
                Ok(()) => {
                    // TODO: Emit Metric
                    debug!("build_change_set::success");
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

    async fn process_project(
        &self,
        change_set: &mut ChangeSet,
        project: &mut Project,
    ) -> Result<(), Error> {
        if project.status == ProjectStatus::Inactive {
            let msg = "process_project::inactive";
            debug!(msg);
            return Err(Error::Snyk(msg.to_string()));
        }

        if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
            let msg = format!("process_project::unsupported::{}", &project.package_manager);
            debug!(msg);
            // TODO: Fix ME
            //change_set.unsupported(project.to_unsupported())?;
            return Ok(());
        }

        match self.insert(project).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("project::insert::{}", e);
                debug!(msg);
                return Err(Error::Entity(e.to_string()));
            }
        }

        Ok(())

        // let snyk_ref = project.to_snyk_ref();
        //
        // let raw = self.sbom_raw(&snyk_ref).await?;
        // let mut sbom = Sbom::from_raw_cdx(
        //     raw.as_str(),
        //     CdxFormat::Json,
        //     Source::Harbor(SNYK_DISCRIMINATOR.to_string()),
        //     Some(HashMap::from([(
        //         XrefKind::External(SNYK_DISCRIMINATOR.to_string()),
        //         HashMap::from(snyk_ref.clone()),
        //     )])),
        // )?;
        //
        // // TODO: Reuse this
        // let sbom_service = SbomService::new(self.cx.clone());
        //
        // match sbom_service.insert_by_purl(&raw, &mut sbom).await {
        //     Ok(()) => {
        //         // TODO: Emit Metric.
        //         debug!("process_project::save_sbom::success");
        //     }
        //     Err(e) => {
        //         // TODO: Emit Metric.
        //         let msg = format!("process_project::save_sbom::{}", e);
        //         debug!(msg);
        //         return Err(Error::Snyk(msg.to_string()));
        //     }
        // };
        //
        // // Don't add the sbom & its components unless it is successfully saved.
        // change_set.track(sbom, project.package_manager.clone(), snyk_ref.clone())?;
        //
        // Ok(())
    }

    async fn sbom_raw(&self, snyk_ref: &SnykRef) -> Result<String, Error> {
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

    // pub(crate) async fn register_purls(&self) -> Result<(), Error> {
    //     let mut purls = HashMap::new();
    //
    //     let packages: Vec<Package> = self.list().await?;
    //     for package in packages {
    //         let component = package.cdx.unwrap();
    //         let package_url = component.purl.clone().unwrap();
    //
    //         if purls.contains_key(package_url.as_str()) {
    //             let existing: &mut Purl = purls.get_mut(package_url.as_str()).unwrap();
    //             existing.snyk_refs(package.snyk_refs);
    //         } else {
    //             let (name, version) = Purl::parse(package_url.clone());
    //             purls.insert(
    //                 package_url.clone(),
    //                 Purl {
    //                     id: package_url,
    //                     purl: "".to_string(),
    //                     name,
    //                     version,
    //                     component_kind: ComponentKind::Package,
    //                     findings: None,
    //                     snyk_refs: package.xref.snyk.clone(),
    //                 },
    //             );
    //         }
    //     }
    //
    //     let dependencies: Vec<Dependency> = self.list().await?;
    //     for dependency in dependencies {
    //         let component = dependency.cdx.unwrap();
    //         let package_url = component.purl.clone().unwrap();
    //
    //         if purls.contains_key(package_url.as_str()) {
    //             let existing: &mut Purl = purls.get_mut(package_url.as_str()).unwrap();
    //             existing.snyk_refs(dependency.snyk_refs);
    //         } else {
    //             let (name, version) = Purl::parse(package_url.clone());
    //             purls.insert(
    //                 package_url.clone(),
    //                 Purl {
    //                     id: package_url,
    //                     purl: "".to_string(),
    //                     name,
    //                     version,
    //                     component_kind: ComponentKind::Dependency,
    //                     findings: None,
    //                     xrefs: dependency.xref.snyk,
    //                 },
    //             );
    //         }
    //     }
    //
    //     for (package_url, mut purl) in purls {
    //         match self.insert(&mut purl).await {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 debug!("failed to insert purl for {} - {}", package_url, e);
    //             }
    //         }
    //     }
    //
    //     Ok(())
    // }

    /// Transaction script for saving sync results to data store.
    async fn commit(&self, change_set: &mut ChangeSet) -> Result<(), Error> {
        let mut processed = HashMap::<String, u16>::new();
        for (key, purl) in change_set.purls.iter_mut() {
            if processed.contains_key(key) {
                debug!("change_set contains duplicated purls");
                match processed.get(key) {
                    None => {}
                    Some(count) => {
                        processed.insert(key.to_string(), *count + 1);
                    }
                }
            } else {
                processed.insert(key.to_string(), 1);
            }

            if purl.id.is_empty() {
                match self.insert(purl).await {
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        debug!("commit::purl::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(purl).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("commit::purl::purl::update::{}", e);
                        continue;
                    }
                }
            }
        }

        processed.clear();

        // TODO: fix these metrics
        for (key, package) in change_set.packages.iter_mut() {
            if processed.contains_key(key) {
                debug!("change_set contains duplicated packages");
                match processed.get(key) {
                    None => {}
                    Some(count) => {
                        processed.insert(key.to_string(), *count + 1);
                    }
                }
            } else {
                processed.insert(key.to_string(), 1);
            }

            if package.id.is_empty() {
                match self.insert(package).await {
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        debug!("commit::package::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(package).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("commit::package::update::{}", e);
                        continue;
                    }
                }
            }
        }

        processed.clear();

        for (key, dependency) in change_set.dependencies.iter_mut() {
            if processed.contains_key(key) {
                debug!("change_set contains duplicated dependencies");
                match processed.get(key) {
                    None => {}
                    Some(count) => {
                        processed.insert(key.to_string(), *count + 1);
                    }
                }
            } else {
                processed.insert(key.to_string(), 1);
            }

            if dependency.id.is_empty() {
                match self.insert(dependency).await {
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        debug!("commit::dependency::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(dependency).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("commit::dependency::update::{}", e);
                        continue;
                    }
                }
            }
        }

        processed.clear();

        for (key, unsupported) in change_set.unsupported.iter_mut() {
            if processed.contains_key(key) {
                debug!("change_set contains duplicated unsupported");
                match processed.get(key) {
                    None => {}
                    Some(count) => {
                        processed.insert(key.to_string(), *count + 1);
                    }
                }
            } else {
                processed.insert(key.to_string(), 1);
            }

            if unsupported.id.is_empty() {
                match self.insert(unsupported).await {
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        debug!("commit::unsupported::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(unsupported).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("commit::unsupported::update::{}", e);
                        continue;
                    }
                }
            }
        }

        Ok(())
    }
}
