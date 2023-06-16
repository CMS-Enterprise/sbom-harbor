use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use harbcore::entities::packages::Package;
use harbcore::entities::tasks::Task;
use harbcore::errors::Error;
use harbcore::services::packages::PackageService;
use harbcore::services::tasks::TaskProvider;
use platform::persistence::mongodb::{Context, Service, Store};

/// Example of how to implement a [TaskProvider] that can interact with the Harbor backend.
#[derive(Debug)]
pub struct ExampleTask {
    store: Arc<Store>,
    packages: PackageService,
}

impl ExampleTask {
    /// Factory method to create new instance of type.
    pub async fn new(cx: Context) -> Result<ExampleTask, Error> {
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Config(e.to_string()))?,
        );

        Ok(ExampleTask {
            store: store.clone(),
            packages: PackageService::new(store),
        })
    }
}

/// A `TaskProvider` must implement the `platform::mongodb::Service` trait so that the `Task`
/// entity can be persisted.
impl Service<Task> for ExampleTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

#[async_trait]
impl TaskProvider for ExampleTask {
    /// Add the custom logic for the task here. This simple example just fetches and prints
    /// the package purl field to stdout.
    async fn run(
        &self,
        task: &mut Task,
    ) -> Result<HashMap<String, String>, harbcore::errors::Error> {
        println!("==> fetching packages");

        // Retrieve the list of packages.
        let packages: Vec<Package> = match self.packages.list().await {
            Ok(packages) => packages,
            Err(e) => {
                return Err(Error::Entity(format!("run::{}", e)));
            }
        };

        // Example of reporting telemetry to standard out.
        let total = packages.len();
        println!("==> processing {} packages...", total);

        // Optionally, record the batch size using the Task entity.
        task.count = packages.len() as u64;

        // Optionally track and report task progress via stdout.
        let mut iteration = 0;

        // Optionally track and return recoverable errors.
        // - If your task does not support recoverable errors, or some scenarios should be
        //   considered a complete failure, your error handling logic should return `Err` as soon
        //   as an unrecoverable error is detected.
        // - Harbor does not currently support batch updates or transactions.
        //   If an all or nothing update is a requirement for your task, you must perform
        //   validation for unrecoverable errors scenarios _prior_ to performing any updates to
        //   the data store.
        let mut errors = HashMap::new();

        // Perform the task.
        for package in packages.iter() {
            iteration += 1;
            let purl = match &package.purl {
                None => {
                    errors.insert(package.id.clone(), "package_purl_none".to_string());
                    println!(
                        "==> error: iteration {} of {} missing purl",
                        iteration, total
                    );
                    continue;
                }
                Some(purl) => purl.clone(),
            };

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
    use harbcore::entities::tasks::TaskKind;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_run() -> Result<(), Error> {
        let cx = harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?;

        let mut task: Task = Task::new(TaskKind::Extension("example".to_string()))
            .map_err(|e| Error::Task(e.to_string()))?;

        let provider = ExampleTask::new(cx)
            .await
            .map_err(|e| Error::Task(e.to_string()))?;

        provider
            .execute(&mut task)
            .await
            .map_err(|e| Error::Task(e.to_string()))
    }
}
