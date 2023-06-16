use crate::entities::sboms::{Author, CdxFormat, Sbom, SbomProviderKind};
use crate::services::snyk::adapters::Project;
use crate::services::snyk::ProjectStatus;
use crate::services::snyk::{SnykService, SUPPORTED_SBOM_PROJECT_TYPES};
use crate::Error;

use crate::entities::tasks::Task;
use crate::entities::xrefs::Xref;
use crate::services::packages::PackageService;
use crate::services::sboms::SbomService;
use crate::services::tasks::TaskProvider;
use async_trait::async_trait;
use platform::persistence::mongodb::{Service, Store};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

/// Synchronizes SBOMS for a Snyk Group with Harbor.
#[derive(Debug)]
pub struct SbomSyncTask {
    store: Arc<Store>,
    pub(in crate::services::sboms::snyk) snyk: SnykService,
    packages: PackageService,
    sboms: SbomService,
}

#[async_trait]
impl TaskProvider for SbomSyncTask {
    /// Builds the Packages Dependencies, Purls, and Unsupported from the Snyk API.
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        println!("==> fetching projects");

        let mut targets = match self.snyk.projects().await {
            Ok(p) => p,
            Err(e) => {
                return Err(Error::Snyk(e.to_string()));
            }
        };

        if targets.is_empty() {
            return Err(Error::Snyk("no_projects".to_string()));
        }

        let total = targets.len();
        println!("==> found {} projects", total);
        task.count = targets.len() as u64;

        let mut iteration = 0;
        let mut errors = HashMap::new();

        for project in targets.iter_mut() {
            iteration += 1;
            println!(
                "==> processing iteration {} of {} for project {}",
                iteration, total, project.project_name
            );

            match self.process_target(task, project).await {
                Ok(()) => {
                    // TODO: Emit Metric
                    println!("==> iteration {} succeeded", iteration);
                }
                Err(e) => {
                    // TODO: Emit Metric
                    println!("==> iteration {} failed with error: {}", iteration, e);
                    errors.insert(project.project_id.clone(), e.to_string());
                }
            }
        }

        Ok(errors)
    }
}

impl Service<Task> for SbomSyncTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl SbomSyncTask {
    /// Factory method to create new instance of type.
    pub fn new(
        store: Arc<Store>,
        snyk: SnykService,
        packages: PackageService,
        sboms: SbomService,
    ) -> Result<SbomSyncTask, Error> {
        Ok(SbomSyncTask {
            store,
            snyk,
            packages,
            sboms,
        })
    }

    /// Generates an [Sbom] and associated types from a Snyk [Project].
    pub(crate) async fn process_target(
        &self,
        task: &Task,
        project: &mut Project,
    ) -> Result<(), Error> {
        if project.status == ProjectStatus::Inactive {
            self.handle_inactive(project)?;
            return Ok(());
        }

        if !SUPPORTED_SBOM_PROJECT_TYPES.contains(&project.package_manager.as_str()) {
            println!(
                "skipping unsupported project {} for manager {}",
                project.project_name, project.package_manager
            );
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
                let msg = format!("process_target::sbom_raw::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        };

        // Load the raw Snyk result into the Harbor model.
        let mut sbom = match Sbom::from_raw_cdx(
            raw.as_str(),
            CdxFormat::Json,
            Author::Harbor(SbomProviderKind::Snyk),
            &package_manager,
            Xref::from(snyk_ref.clone()),
            task,
        ) {
            Ok(sbom) => sbom,
            Err(e) => {
                let msg = format!("process_target::from_raw_cdx::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        };

        // Determine how many times this Sbom has been synced.
        match self.sboms.set_instance_by_purl(&mut sbom).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("process_target::from_raw_cdx::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg));
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
                debug!("process_target::write_to_storage::success");
            }
            Err(e) => {
                // TODO: Emit Metric.
                let msg = format!("process_target::write_to_storage::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        };

        // Ensure the timestamp is set if successful.
        sbom.timestamp = platform::time::timestamp()?;

        // Commit the Sbom.
        self.commit_target(&mut sbom, Xref::from(snyk_ref)).await?;

        Ok(())
    }

    fn handle_inactive(&self, project: &mut Project) -> Result<(), Error> {
        // TODO: Track if a project went from Active to Inactive.
        let msg = "handle_inactive::inactive";
        debug!("{}::{}", msg, project.project_name);
        // TODO: Track inactive?
        Ok(())
    }

    /// [Transaction script](https://martinfowler.com/eaaCatalog/transactionScript.html) for saving
    /// Sbom results to data store. If change_set None, indicates Sbom has errors and dependent
    /// entities should not be committed.
    pub(crate) async fn commit_target(&self, sbom: &mut Sbom, xref: Xref) -> Result<(), Error> {
        let mut errs = HashMap::<String, String>::new();

        // Should always insert Sbom. It should never be a duplicate, but a new instance from task.
        match self.sboms.insert(sbom).await {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Sbom(e.to_string()));
            }
        }

        let mut package = match sbom.package.clone() {
            Some(package) => package,
            None => {
                return Err(Error::Sbom("sbom_package_none".to_string()));
            }
        };

        match self
            .packages
            .upsert_package_by_purl(&mut package, Some(&xref))
            .await
        {
            Ok(_) => {}
            Err(e) => {
                errs.insert("package".to_string(), e.to_string());
            }
        }

        if !errs.is_empty() {
            let errs = match serde_json::to_string(&errs) {
                Ok(errs) => errs,
                Err(e) => format!("error serializing errs {}", e),
            };
            return Err(Error::Sbom(errs));
        }

        for dependency in package.dependencies.iter_mut() {
            match self
                .packages
                .upsert_package_by_purl(dependency, Some(&xref))
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    errs.insert("package".to_string(), e.to_string());
                }
            }
        }

        if !errs.is_empty() {
            let errs = match serde_json::to_string(&errs) {
                Ok(errs) => errs,
                Err(e) => format!("error serializing errs {}", e),
            };
            return Err(Error::Sbom(errs));
        }

        Ok(())
    }
}
