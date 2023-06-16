use crate::entities::tasks::{Task, TaskStatus};
use crate::Error;
use std::collections::HashMap;

use async_trait::async_trait;
use chrono::Utc;
use platform::persistence::mongodb::Service;
use tracing::log::debug;

/// Provides a [Template Method](https://en.wikipedia.org/wiki/Template_method_pattern) for running
/// and logging Task operations.
#[async_trait]
pub trait TaskProvider: Service<Task> {
    /// Implement this to load and process data.
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error>;

    /// Run the task and store the results.
    async fn execute(&self, task: &mut Task) -> Result<(), Error> {
        match self.init(task).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("init::failed::{}", e);
                println!("{}", msg);
                return Err(Error::Task(msg));
            }
        }

        let errors = self.run(task).await;

        match errors {
            Ok(errors) => {
                for (key, value) in errors {
                    println!("==> error processing {}: {}", key, value);
                    task.ref_errs(key, value);
                }
            }
            Err(e) => {
                let msg = format!("run::failed::{}", e);
                println!("{}", msg);

                task.err = Some(msg);
            }
        };

        // TODO: Emit Metric for changeset totals.

        let result = self.complete(task).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => return Err(Error::Task(e.to_string())),
        }
    }

    /// Inserts the [Task] record at the start of the task run.
    async fn init(&self, task: &mut Task) -> Result<(), Error> {
        match self.insert(task).await {
            Ok(()) => {}
            Err(e) => {
                let msg = format!("init::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Task(msg));
            }
        };

        // TODO: Write tests.
        if task.id.is_empty() {
            return Err(Error::Entity("task::id_empty".to_string()));
        }

        Ok(())
    }

    /// Updates the [Task] record at the end of the task run.
    async fn complete(&self, task: &mut Task) -> Result<(), Error> {
        match task.err {
            None => {
                task.err_total = match &task.ref_errs {
                    None => 0,
                    Some(ref_errs) => ref_errs.len() as u64,
                };

                match task.err_total > 0 {
                    true => task.status = TaskStatus::CompleteWithErrors,
                    false => {
                        task.status = TaskStatus::Complete;
                    }
                }
            }
            Some(_) => {
                task.status = TaskStatus::Failed;
            }
        }

        task.finish = Utc::now();
        task.duration_seconds = task.finish.signed_duration_since(task.start).num_seconds();

        match self.update(task).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let complete_raw = match serde_json::to_string(&task) {
                    Ok(raw) => raw,
                    Err(serde_err) => {
                        println!("error serializing task: {}", serde_err);
                        "{ err: null }".to_string()
                    }
                };

                let msg = format!("update::failed::{} - {}", e, complete_raw);
                debug!("{}", msg);
                return Err(Error::Task(msg));
            }
        }
    }
}
