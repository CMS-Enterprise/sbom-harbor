use crate::entities::packages::{Dependency, Package, Purl, Unsupported};
use crate::entities::sboms::{CdxFormat, Sbom, SbomProviderKind, Source};
use crate::services::snyk::adapters::Project;
use crate::services::snyk::{ProjectStatus, SbomFormat, API_VERSION};
use crate::services::snyk::{
    SnykRef, SnykService, SNYK_DISCRIMINATOR, SUPPORTED_SBOM_PROJECT_TYPES,
};
use crate::Error;

use crate::entities::enrichment::Scan;
use crate::entities::xrefs::XrefKind;
use crate::services::enrichment::snyk::changeset::ScanSbomsChangeSet;
use crate::services::enrichment::SbomProvider;
use async_trait::async_trait;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use tracing::debug;

// Implement mongo Service with type arg for all the types that this service can persist.
impl Service<Project> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Sbom> for SnykService {
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
impl SbomProvider<'_> for SnykService {
    /// Synchronizes a Snyk instance with Harbor.
    async fn scan(&self, scan: &mut Scan) -> Result<(), Error> {
        let mut change_set = ScanSbomsChangeSet::new(scan);

        // Populate the ChangeSet
        match self.build_sboms(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
                return Err(Error::Snyk(
                    format!("snyk_service::sync::{}", e).to_string(),
                ));
            }
        };

        match self.commit_sboms(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
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
    pub(crate) async fn build_sboms(
        &self,
        change_set: &mut ScanSbomsChangeSet<'_>,
    ) -> Result<(), Error> {
        let mut projects = match self.projects().await {
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

        // TODO: Emit Metric for changeset totals.

        Ok(())
    }

    pub(crate) async fn process_project(
        &self,
        change_set: &mut ScanSbomsChangeSet<'_>,
        project: &mut Project,
    ) -> Result<(), Error> {
        if project.status == ProjectStatus::Inactive {
            // TODO: Track if a project went from Active to Inactive.
            let msg = "process_project::inactive";
            debug!("{}", msg);
            return Ok(());
        }

        if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
            let msg = format!("process_project::unsupported::{}", &project.package_manager);
            debug!("{}", msg);
            change_set.unsupported(project.to_unsupported())?;
            return Ok(());
        }

        let snyk_ref = project.to_snyk_ref();

        let raw = match self.sbom_raw(&snyk_ref).await {
            Ok(raw) => raw,
            Err(e) => {
                let msg = format!("process_project::sbom_raw::{}", e);
                debug!("{}", msg);
                return Err(Error::Snyk(msg.to_string()));
            }
        };

        let mut sbom = match Sbom::from_raw_cdx(
            raw.as_str(),
            CdxFormat::Json,
            Source::Harbor(SbomProviderKind::Snyk {
                api_version: API_VERSION.to_string(),
            }),
            &change_set.scan,
            Some(HashMap::from([(
                XrefKind::External(SNYK_DISCRIMINATOR.to_string()),
                HashMap::from(snyk_ref.clone()),
            )])),
        ) {
            Ok(sbom) => sbom,
            Err(e) => {
                let msg = format!("process_project::from_raw_cdx::{}", e);
                debug!("{}", msg);
                return Err(Error::Snyk(msg.to_string()));
            }
        };

        match self.sbom_service.store_by_purl(&raw, &mut sbom).await {
            Ok(()) => {
                // TODO: Emit Metric.
                debug!("process_project::save_sbom::success");
            }
            Err(e) => {
                // TODO: Emit Metric.
                let msg = format!("process_project::save_sbom::{}", e);
                debug!("{}", msg);
                return Err(Error::Snyk(msg.to_string()));
            }
        };

        // Don't add the sbom & its components unless it is successfully saved.
        match change_set.track(&mut sbom, project.package_manager.clone(), snyk_ref.clone()) {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("change_set::track::{}", e);
                debug!("{}", msg);
                return Err(Error::Snyk(msg.to_string()));
            }
        }

        Ok(())
    }

    /// Transaction script for saving scan results to data store.
    pub(crate) async fn commit_sboms(
        &self,
        change_set: &mut ScanSbomsChangeSet<'_>,
    ) -> Result<(), Error> {
        // TODO: Handle ScanRefs on Purls.
        let mut processed = HashMap::<String, u32>::new();

        for (key, sbom) in change_set.sboms.iter_mut() {
            if sbom.id.is_empty() {
                match self.insert(sbom).await {
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        debug!("commit::sbom::insert::{}", e);
                        continue;
                    }
                }
            } else {
                match self.update(sbom).await {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("commit::sbom::purl::update::{}", e);
                        continue;
                    }
                }
            }

            if processed.contains_key(key) {
                debug!("change_set contains duplicated sboms");
                match processed.get(key) {
                    None => {}
                    Some(count) => {
                        processed.insert(key.to_string(), *count + 1);
                    }
                }
            } else {
                processed.insert(key.to_string(), 1);
            }
        }

        processed.clear();

        for (key, purl) in change_set.purls.iter_mut() {
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
        }

        processed.clear();

        for (key, package) in change_set.packages.iter_mut() {
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
        }

        processed.clear();

        for (key, dependency) in change_set.dependencies.iter_mut() {
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
        }

        processed.clear();

        for (key, unsupported) in change_set.unsupported.iter_mut() {
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
        }

        Ok(())
    }

    pub async fn sbom_raw(&self, snyk_ref: &SnykRef) -> Result<String, Error> {
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
}
