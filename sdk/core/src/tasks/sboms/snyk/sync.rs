use crate::entities::sboms::SbomProviderKind;
use crate::services::snyk::adapters::Project;
use crate::services::snyk::ProjectStatus;
use crate::services::snyk::{SnykService, SUPPORTED_SBOM_PROJECT_TYPES};
use crate::Error;

use crate::entities::tasks::Task;
use crate::entities::xrefs::Xref;
use crate::services::packages::PackageService;
use crate::services::sboms::SbomService;
use crate::tasks::TaskProvider;
use async_trait::async_trait;
use platform::persistence::mongodb::{Service, Store};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

/// Synchronizes SBOMS for a Snyk Group with Harbor.
#[derive(Debug)]
pub struct SyncTask {
    store: Arc<Store>,
    pub(in crate::tasks::sboms::snyk) snyk: SnykService,
    packages: PackageService,
    sboms: SbomService,
}

#[async_trait]
impl TaskProvider for SyncTask {
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

impl Service<Task> for SyncTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl SyncTask {
    /// Factory method to create new instance of type.
    pub fn new(
        store: Arc<Store>,
        snyk: SnykService,
        packages: PackageService,
        sboms: SbomService,
    ) -> Result<SyncTask, Error> {
        Ok(SyncTask {
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
        project: &Project,
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

        // Load the raw Snyk result into the Harbor model and save to the data store.
        self.sboms
            .ingest(
                raw.as_str(),
                package_manager,
                SbomProviderKind::Snyk,
                Xref::from(snyk_ref),
                Some(task),
            )
            .await?;

        Ok(())
    }

    fn handle_inactive(&self, project: &Project) -> Result<(), Error> {
        // TODO: Track if a project went from Active to Inactive.
        let msg = "handle_inactive::inactive";
        debug!("{}::{}", msg, project.project_name);
        // TODO: Track inactive?
        Ok(())
    }
}
