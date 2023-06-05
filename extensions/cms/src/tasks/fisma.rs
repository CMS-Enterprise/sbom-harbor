use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use harbcore::entities::packages::{Package, PackageKind};
use harbcore::entities::tasks::Task;
use harbcore::errors::Error;
use harbcore::services::packages::PackageService;
use harbcore::services::tasks::TaskProvider;
use platform::mongodb::{Context, Service, Store};

use crate::services::snyk::SnykService;

/// Example of how to implement a [TaskProvider] that can interact with the Harbor backend.
#[derive(Debug)]
pub struct FismaTask {
    store: Arc<Store>,
    packages: PackageService,
    snyk: SnykService,
}

impl FismaTask {
    /// Factory method to create new instance of type.
    pub async fn new(cx: Context, token: String) -> Result<FismaTask, Error> {
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Config(e.to_string()))?,
        );

        Ok(FismaTask {
            store: store.clone(),
            packages: PackageService::new(store),
            snyk: SnykService::new(token),
        })
    }
}

/// A `TaskProvider` must implement the `platform::mongodb::Service` trait so that the `Task`
/// entity can be persisted.
impl Service<Task> for FismaTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

#[async_trait]
impl TaskProvider for FismaTask {
    /// Looks for primary packages without a FISMA ID, and synchronizes them with the Snyk
    /// OrgTags API.
    async fn run(
        &self,
        task: &mut Task,
    ) -> Result<HashMap<String, String>, harbcore::errors::Error> {
        println!("==> fetching packages");

        // Retrieve the list of packages.
        let packages: Vec<Package> = match self
            .packages
            .query(HashMap::from([(
                "kind",
                format!("{}", PackageKind::Primary).as_str(),
            )]))
            .await
        {
            Ok(packages) => packages,
            Err(e) => {
                return Err(Error::Fisma(format!("run::{}", e)));
            }
        };

        let tags = self.snyk.org_tags().await {
            Ok(tags) => tags,
            Err(e) => {
                return Err(Error::Snyk(e.to_string()))
            }
        };

        // Example of reporting telemetry to standard out.
        let total = packages.len();
        println!("==> processing {} packages...", total);

        // Optionally, record the batch size using the Task entity.
        task.count = packages.len() as u64;

        // Optionally track and report task progress via stdout.
        let mut iteration = 0;
        let mut errors = HashMap::new();

        // Perform the task.
        for package in packages.iter() {
            iteration += 1;

            println!(
                "==> processing iteration {} of {} for purl {}",
                iteration, total, purl
            );
        }

        // Return error summary.
        Ok(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use harbcore::config::dev_context;
    use harbcore::entities::tasks::TaskKind;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_run() -> Result<(), Error> {
        let cx = harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?;

        let mut task: Task =
            Task::new(TaskKind::Extension("fisma")).map_err(|e| Error::Task(e.to_string()))?;

        let provider = FismaTask::new(cx)
            .await
            .map_err(|e| Error::Task(e.to_string()))?;

        provider
            .execute(&mut task)
            .await
            .map_err(|e| Error::Task(e.to_string()))
    }
}
