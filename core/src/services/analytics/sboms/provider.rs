use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use platform::mongodb::{Service, Store};
use crate::entities::tasks::{Task, TaskStatus};
use crate::Error;
use crate::services::analytics::sboms::service::AnalyticService;
use crate::services::tasks::TaskProvider;
use async_trait::async_trait;

/// AnalyticExecutionTask to add errors to the data store
#[derive(Debug)]
pub struct SbomDetailTask {
    service: AnalyticService
}

impl SbomDetailTask {

    /// Creates a new AnalyticExecutionTask
    pub fn new(service: AnalyticService) -> Self {
        Self {
            service
        }
    }
}

impl Service<Task> for SbomDetailTask {
    fn store(&self) -> Arc<Store> {
        self.service.store.clone()
    }
}

#[async_trait]
impl TaskProvider for SbomDetailTask {

    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {

        let mut errors = HashMap::<String, String>::new();
        let mut report_paths = Vec::<String>::new();

        let primary_purls = match self.service.get_primary_purls().await {
            Ok(opt) => match opt {
                Some(purls) => purls,
                None => panic!("Error attempting to get primary purls, none found!"),
            },
            Err(err) => panic!("Error attempting to get primary purls: {}", err)
        };

        task.count = primary_purls.len() as u64;

        for purl in primary_purls {
            match self.service.generate_detail(purl.clone()).await {
                Ok(file_path_option) =>
                    if let Some(file_path) = file_path_option {
                        report_paths.push(file_path)
                    },
                Err(err) => {
                    errors.insert(purl, format!("{}", err));
                }
            }
        }

        task.status = TaskStatus::Complete;

        Ok(errors)
    }
}

