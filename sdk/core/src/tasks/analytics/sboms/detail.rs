use crate::entities::tasks::{Task, TaskStatus};
use crate::services::analytics::sboms::service::AnalyticService;
use crate::tasks::TaskProvider;
use crate::Error;
use async_trait::async_trait;
use platform::persistence::mongodb::{Service, Store};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// AnalyticExecutionTask to add errors to the data store
#[derive(Debug)]
pub struct DetailTask {
    service: AnalyticService,
}

impl DetailTask {
    /// Creates a new AnalyticExecutionTask
    pub fn new(service: AnalyticService) -> Self {
        Self { service }
    }
}

impl Service<Task> for DetailTask {
    fn store(&self) -> Arc<Store> {
        self.service.store.clone()
    }
}

#[async_trait]
impl TaskProvider for DetailTask {
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        let mut errors = HashMap::<String, String>::new();
        let mut report_paths = Vec::<String>::new();

        let primary_purls = match self.service.get_primary_purls().await {
            Ok(opt) => match opt {
                Some(purls) => purls,
                None => {
                    return Err(Error::Analytic(
                        "Error attempting to get primary purls, none found!".to_string(),
                    ))
                }
            },
            Err(err) => {
                return Err(Error::Analytic(format!(
                    "Error attempting to get primary purls: {err}"
                )))
            }
        };

        task.count = primary_purls.len() as u64;

        for purl in primary_purls {
            match self.service.generate_detail(purl.clone()).await {
                Ok(file_path_option) => {
                    if let Some(file_path) = file_path_option {
                        report_paths.push(file_path)
                    }
                }
                Err(err) => {
                    errors.insert(purl, format!("{}", err));
                }
            }
        }

        task.status = TaskStatus::Complete;

        Ok(errors)
    }
}
