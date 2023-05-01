use crate::entities::sboms::{CdxFormat, Sbom, SbomProviderKind, Source};
use crate::services::snyk::adapters::Project;
use crate::services::snyk::{ProjectStatus, API_VERSION};
use crate::services::snyk::{SnykService, SUPPORTED_SBOM_PROJECT_TYPES};
use crate::Error;

use crate::entities::packages::{ComponentKind, Purl};
use crate::entities::scans::{Scan, ScanKind};
use crate::entities::xrefs::Xref;
use crate::services::packages::PackageService;
use crate::services::sboms::SbomService;
use crate::services::scans::ScanProvider;
use async_std::stream;
use async_trait::async_trait;
use futures::stream::StreamExt;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct SbomScanProvider {
    scan: Arc<Mutex<Scan>>,
    cx: Context,
    pub(in crate::services::sboms::snyk) snyk: SnykService,
    packages: PackageService,
    sboms: SbomService,
    pub(in crate::services::sboms::snyk) targets: Vec<Project>,
}

#[async_trait]
impl ScanProvider for SbomScanProvider {
    fn current(&self) -> Arc<Mutex<Scan>> {
        self.scan.clone()
    }

    /// Builds the Packages Dependencies, Purls, and Unsupported from the Snyk API.
    async fn scan_targets(&self) -> Result<HashMap<String, String>, Error> {
        println!("processing sboms for {} projects...", self.targets.len());
        let scan_id = self.current().lock().unwrap().id.clone();
        let mut project_scanner = Arc::new(Mutex::new(ProjectScanner::new(scan_id)));

        //for project in self.targets.iter_mut() {
        stream::from_iter(&self.targets)
            .for_each_concurrent(8, |project| async move {
                let mut project = project.clone();
                let mut scanner = Arc::clone(&project_scanner);
                let mut iteration = scanner.lock().unwrap().iteration;
                iteration += 1;
                scanner.lock().unwrap().iteration = iteration;

                println!(
                    "==> processing iteration {} for project {}",
                    iteration, project.project_name
                );

                let result = scanner
                    .lock()
                    .unwrap()
                    .scan_target(&mut project, &self.snyk, &self.packages, &self.sboms)
                    .await;

                match result {
                    Ok(()) => {
                        // TODO: Emit Metric
                        println!("==> iteration {} succeeded", iteration);
                    }
                    Err(e) => {
                        // TODO: Emit Metric
                        println!("==> iteration {} failed with error: {}", iteration, e);
                    }
                }
            })
            .await;
        //}

        // TODO: Emit Metric for errors
        let results = Arc::clone(&project_scanner).lock().unwrap().errors.clone();

        Ok(results)
    }

    async fn load_targets(&mut self) -> Result<(), Error> {
        self.targets = match self.snyk.projects().await {
            Ok(p) => p,
            Err(e) => {
                return Err(Error::Snyk(e.to_string()));
            }
        };

        if self.targets.is_empty() {
            return Err(Error::Snyk("no_projects".to_string()));
        }

        Ok(())
    }

    fn target_count(&self) -> u64 {
        self.targets.len() as u64
    }
}

impl Service<Scan> for SbomScanProvider {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl SbomScanProvider {
    /// Factory method to create new instance of type.
    pub fn new(
        cx: Context,
        snyk: SnykService,
        packages: PackageService,
        sboms: SbomService,
    ) -> Result<SbomScanProvider, Error> {
        let scan = Scan::new(ScanKind::Sbom(SbomProviderKind::Snyk {
            api_version: API_VERSION.to_string(),
        }))?;

        Ok(SbomScanProvider {
            cx,
            snyk,
            packages,
            sboms,
            scan: Arc::new(Mutex::new(scan)),
            targets: vec![],
        })
    }
}

// ProjectScanner is a flyweight that processes
struct ProjectScanner {
    iteration: u64,
    scan_id: String,
    errors: HashMap<String, String>,
}

impl ProjectScanner {
    pub fn new(scan_id: String) -> Self {
        let errors: HashMap<String, String> = HashMap::new();

        Self {
            iteration: 0,
            scan_id,
            errors,
        }
    }

    /// Generates an [Sbom] and associated types from a Snyk [Project].
    pub(crate) async fn scan_target(
        &mut self,
        project: &mut Project,
        snyk: &SnykService,
        packages: &PackageService,
        sboms: &SbomService,
    ) -> Result<(), Error> {
        if project.status == ProjectStatus::Inactive {
            self.handle_inactive(project)?;
            return Ok(());
        }

        if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
            let mut unsupported = project.to_unsupported();
            match packages
                .upsert_unsupported_by_external_id(&mut unsupported)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    self.errors.insert(unsupported.external_id, e.to_string());
                }
            }
            return Ok(());
        }

        let package_manager = Some(project.package_manager.clone());
        let snyk_ref = project.to_snyk_ref();

        // Get the raw Sbom result from the API.
        let raw = match snyk.sbom_raw(&snyk_ref).await {
            Ok(raw) => raw,
            Err(e) => {
                let msg = format!("scan_target::sbom_raw::{}", e);
                println!("{}", msg);
                self.errors.insert(project.id.clone(), msg.clone());
                return Ok(());
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
                println!("{}", msg);
                self.errors.insert(project.id.clone(), msg.clone());
                return Ok(());
            }
        };

        let raw_purl = match sbom.purl() {
            Ok(raw_purl) => raw_purl,
            Err(e) => {
                let msg = format!("scan_target::sbom_purl_none::{}", e);
                println!("{}", msg);
                self.errors.insert(project.id.clone(), msg.clone());
                return Ok(());
            }
        };

        // Determine how many times this Sbom has been scanned/synced.
        match sboms.set_instance_by_purl(&mut sbom).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("scan_target::from_raw_cdx::{}", e);
                println!("{}", msg);
                self.errors.insert(raw_purl, msg.clone());
                return Ok(());
            }
        }

        // Write the sbom to the storage provider.
        match sboms
            .write_to_storage(
                raw.as_bytes().to_vec(),
                &mut sbom,
                Some(Xref::from(snyk_ref.clone())),
            )
            .await
        {
            Ok(()) => {
                // TODO: Emit Metric.
                println!("scan_target::write_to_storage::success");
            }
            Err(e) => {
                // TODO: Emit Metric.
                let msg = format!("scan_target::write_to_storage::{}", e);
                println!("{}", msg);
                self.errors.insert(raw_purl, msg.clone());
                return Ok(());
            }
        };

        // Ensure the timestamp is set if successful.
        sbom.timestamp = platform::time::timestamp()?;

        // Commit the Sbom.
        match self
            .commit_target(&mut sbom, &packages, &sboms, Xref::from(snyk_ref))
            .await
        {
            Ok(errs) => {}
            Err(e) => {
                return Err(Error::Scan(e.to_string()));
            }
        }

        Ok(())
    }

    fn handle_inactive(&self, project: &mut Project) -> Result<(), Error> {
        // TODO: Track if a project went from Active to Inactive.
        let msg = "handle_inactive::inactive";
        println!("{}::{}", msg, project.project_name);
        // TODO: Track inactive?
        Ok(())
    }

    /// Transaction script for saving Sbom results to data store. If change_set None, indicates Sbom
    /// has errors and dependent entities should not be committed.
    pub(crate) async fn commit_target(
        &mut self,
        sbom: &mut Sbom,
        packages: &PackageService,
        sboms: &SbomService,
        xref: Xref,
    ) -> Result<(), Error> {
        // Should always insert Sbom. It should never be a duplicate, but a new instance from scan.
        match sboms.insert(sbom).await {
            Ok(_) => {}
            Err(e) => {
                self.errors.insert("sbom".to_string(), e.to_string());
            }
        }

        match sbom.package.clone() {
            None => {
                self.errors
                    .insert("package".to_string(), "sbom_package_none".to_string());
            }
            Some(mut package) => match packages.upsert_package_by_purl(&mut package).await {
                Ok(_) => {}
                Err(e) => {
                    self.errors.insert("package".to_string(), e.to_string());
                }
            },
        }

        match sbom.component() {
            Some(component) => {
                match Purl::from_component(
                    &component,
                    ComponentKind::Package,
                    self.scan_id.as_str(),
                    sbom.iteration(),
                    xref.clone(),
                ) {
                    Ok(mut purl) => match packages.upsert_purl(&mut purl).await {
                        Ok(_) => {}
                        Err(e) => {
                            self.errors.insert("purl".to_string(), e.to_string());
                        }
                    },
                    Err(e) => {
                        self.errors.insert("purl".to_string(), e.to_string());
                    }
                }
            }
            None => {}
        };

        for dependency in sbom.dependencies.iter_mut() {
            let key = match dependency.purl() {
                None => "unset".to_string(),
                Some(purl) => format!("dependency::{}", purl),
            };

            match packages.upsert_dependency_by_purl(dependency).await {
                Ok(_) => {}
                Err(e) => {
                    self.errors.insert(key.clone(), e.to_string());
                }
            }

            match &dependency.component {
                None => {
                    self.errors
                        .insert(key, "dependency_component_none".to_string());
                }
                Some(component) => {
                    let mut purl = match Purl::from_component(
                        component,
                        ComponentKind::Dependency,
                        self.scan_id.as_str(),
                        0,
                        xref.clone(),
                    ) {
                        Ok(purl) => purl,
                        Err(e) => {
                            self.errors.insert(key.clone(), e.to_string());
                            continue;
                        }
                    };

                    match packages.upsert_purl(&mut purl).await {
                        Ok(_) => {}
                        Err(e) => {
                            self.errors.insert(
                                key.clone(),
                                format!("upsert_dependency_purl::{}", e.to_string()),
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
