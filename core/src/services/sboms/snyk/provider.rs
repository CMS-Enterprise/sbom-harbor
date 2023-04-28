use crate::entities::sboms::{CdxFormat, Sbom, SbomProviderKind, Source};
use crate::services::snyk::adapters::Project;
use crate::services::snyk::{ProjectStatus, API_VERSION};
use crate::services::snyk::{SnykService, SUPPORTED_SBOM_PROJECT_TYPES};
use crate::Error;

use crate::entities::cyclonedx::Component;
use crate::entities::packages::{ComponentKind, Package, Purl};
use crate::entities::scans::Scan;
use crate::entities::xrefs::Xref;
use crate::services::packages::PackageService;
use crate::services::sboms::{SbomProvider, SbomService};
use crate::services::scans::ScanProvider;
use async_trait::async_trait;
use platform::mongodb::{Context, MongoDocument, Service};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug)]
pub struct SbomScanProvider {
    cx: Context,
    pub(in crate::services::sboms::snyk) snyk: SnykService,
    packages: PackageService,
    sboms: SbomService,
}

impl ScanProvider<'_> for SbomScanProvider {}

impl Service<Scan> for SbomScanProvider {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

#[async_trait]
impl SbomProvider<'_> for SbomScanProvider {
    /// Synchronizes a Snyk instance with Harbor.
    async fn scan(&self, scan: &mut Scan) -> Result<(), Error> {
        // Scan the targets and capture any unrecoverable error.
        match self.scan_targets(scan).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
                return Err(Error::Snyk(
                    format!("snyk_service::sync::{}", e).to_string(),
                ));
            }
        };

        Ok(())
    }
}

impl SbomScanProvider {
    /// Factory method to create new instance of type.
    pub(crate) fn new(
        cx: Context,
        snyk: SnykService,
        packages: PackageService,
        sboms: SbomService,
    ) -> Self {
        Self {
            cx,
            snyk,
            packages,
            sboms,
        }
    }

    /// Builds the Packages Dependencies, Purls, and Unsupported from the Snyk API.
    pub(crate) async fn scan_targets(&self, scan: &mut Scan) -> Result<(), Error> {
        let mut projects = match self.snyk.projects().await {
            Ok(p) => p,
            Err(e) => {
                let msg = format!("scan_targets::projects::{}", e);
                debug!(msg);
                return Err(Error::Snyk(msg));
            }
        };

        if projects.is_empty() {
            return Err(Error::Snyk("scan_targets::no_projects".to_string()));
        }

        for project in projects.iter_mut() {
            match self.scan_target(scan, project).await {
                Ok(()) => {
                    // TODO: Emit Metric
                    debug!("scan_targets::success");
                }
                Err(e) => {
                    // TODO: Emit Metric
                    debug!("scan_targets::{}", e);
                    scan.ref_errs(project.id.clone(), e.to_string());
                }
            }
        }

        // TODO: Emit Metric for changeset totals.

        Ok(())
    }

    /// Generates an [Sbom] and associated types from a Snyk [Project].
    pub(crate) async fn scan_target(
        &self,
        scan: &mut Scan,
        project: &mut Project,
    ) -> Result<(), Error> {
        if project.status == ProjectStatus::Inactive {
            self.handle_inactive(project)?;
            return Ok(());
        }

        if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
            let mut unsupported = project.to_unsupported();
            self.packages
                .upsert_unsupported_by_external_id(&mut unsupported)
                .await?;
            return Ok(());
        }

        let package_manager = Some(project.package_manager.clone());
        let snyk_ref = project.to_snyk_ref();

        // Get the raw Sbom result from the API.
        let raw = match self.snyk.sbom_raw(&snyk_ref).await {
            Ok(raw) => raw,
            Err(e) => {
                let msg = format!("scan_target::sbom_raw::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg.to_string()));
            }
        };

        // Load the raw Snyk result into the Harbor model.
        let mut sbom = match Sbom::from_raw_cdx(
            raw.as_str(),
            CdxFormat::Json,
            Source::Harbor(SbomProviderKind::Snyk {
                api_version: API_VERSION.to_string(),
            }),
            &package_manager,
            Xref::from(snyk_ref.clone()),
        ) {
            Ok(sbom) => sbom,
            Err(e) => {
                let msg = format!("scan_target::from_raw_cdx::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg.to_string()));
            }
        };

        // Determine how many times this Sbom has been scanned/synced.
        match self.sboms.set_instance_by_purl(&mut sbom).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("scan_target::from_raw_cdx::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg.to_string()));
            }
        }

        // Write the sbom to the storage provider.
        match self
            .sboms
            .write_to_storage(
                raw.as_bytes().to_vec(),
                &mut sbom,
                Some(Xref::from(snyk_ref.clone())),
            )
            .await
        {
            Ok(()) => {
                // TODO: Emit Metric.
                debug!("scan_target::write_to_storage::success");
            }
            Err(e) => {
                // TODO: Emit Metric.
                let msg = format!("scan_target::write_to_storage::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg.to_string()));
            }
        };

        // Ensure the timestamp is set if successful.
        sbom.timestamp = platform::time::timestamp()?;

        // Commit the Sbom.
        self.commit_target(scan, &mut sbom, Xref::from(snyk_ref))
            .await?;

        Ok(())
    }

    fn handle_inactive(&self, project: &mut Project) -> Result<(), Error> {
        // TODO: Track if a project went from Active to Inactive.
        let msg = "handle_inactive::inactive";
        debug!("{}::{}", msg, project.project_name);
        // TODO: Track inactive?
        Ok(())
    }

    /// Transaction script for saving Sbom results to data store. If change_set None, indicates Sbom
    /// has errors and dependent entities should not be committed.
    pub(crate) async fn commit_target(
        &self,
        scan: &Scan,
        sbom: &mut Sbom,
        xref: Xref,
    ) -> Result<(), Error> {
        let mut errs = HashMap::<String, String>::new();

        // Should always insert Sbom. It should never be a duplicate, but a new instance from scan.
        match self.sboms.insert(sbom).await {
            Ok(_) => {}
            Err(e) => {
                errs.insert("sbom".to_string(), e.to_string());
            }
        }

        match sbom.package.clone() {
            None => {
                errs.insert("package".to_string(), "sbom_package_none".to_string());
            }
            Some(mut package) => match self.packages.upsert_package_by_purl(&mut package).await {
                Ok(_) => {}
                Err(e) => {
                    errs.insert("package".to_string(), e.to_string());
                }
            },
        }

        match sbom.component() {
            Some(component) => {
                match Purl::from_component(
                    &component,
                    ComponentKind::Package,
                    scan,
                    sbom.iteration(),
                    xref,
                ) {
                    Ok(mut purl) => match self.packages.upsert_purl(&mut purl).await {
                        Ok(_) => {}
                        Err(e) => {
                            errs.insert("purl".to_string(), e.to_string());
                        }
                    },
                    Err(e) => {
                        errs.insert("purl".to_string(), e.to_string());
                    }
                }
            }
            None => {}
        };

        for dependency in sbom.dependencies.iter_mut() {
            match self.packages.upsert_dependency_by_purl(dependency).await {
                Ok(_) => {}
                Err(e) => {
                    let key = match dependency.purl() {
                        None => "unset".to_string(),
                        Some(purl) => format!("dependency::{}", purl),
                    };
                    let err = e.to_string();
                    errs.insert(key, err);
                }
            }
        }

        if !errs.is_empty() {
            let errs = match serde_json::to_string(&errs) {
                Ok(errs) => errs,
                Err(e) => format!("error serializing errs {}", e),
            };
            return Err(Error::Sbom(errs.to_string()));
        }

        Ok(())
    }
}
