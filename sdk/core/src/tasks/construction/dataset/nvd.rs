use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;

use crate::entities::datasets::{CpesContainer, Purl2Cpes, PurlsContainer};
use platform::persistence::mongodb::{Service, Store};

use crate::entities::tasks::Task;
use crate::services::nvd::Service as NvdService;
use crate::tasks::TaskProvider;
use crate::Error;

use serde_yaml::from_reader;

/// Builds data set for NVD.
#[derive(Debug)]
pub struct ConstructionTask {
    pub(in crate::tasks::construction::dataset) service: NvdService,
}

impl ConstructionTask {
    /// Creates a new NVD ConstructionTask
    pub fn new(service: NvdService) -> Self {
        Self { service }
    }
}

impl Service<Task> for ConstructionTask {
    fn store(&self) -> Arc<Store> {
        self.service.store()
    }
}

#[async_trait]
impl TaskProvider for ConstructionTask {
    async fn run(&self, _task: &mut Task) -> Result<HashMap<String, String>, Error> {
        println!("==> Running ConstructionTask Provider to build nvd dataset");

        // Create an Errors map to return at the end of the task
        let mut errors = HashMap::<String, String>::new();

        // self.service.get_vulnerabilities()

        Ok(errors)
    }
}
