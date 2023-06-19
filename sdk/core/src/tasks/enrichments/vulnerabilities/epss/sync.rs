use super::client::Client;
use crate::entities::enrichments::Vulnerability;
use crate::entities::tasks::Task;
use crate::tasks::TaskProvider;
use crate::Error;
use async_trait::async_trait;
use platform::persistence::mongodb::{Service, Store};
use std::collections::HashMap;
use std::sync::Arc;

/// Synchronizes the full set of [Vulnerability] entries with their current EPSS Score.
#[derive(Debug)]
pub struct SyncTask {
    store: Arc<Store>,
    client: Client,
}

#[async_trait]
impl TaskProvider for SyncTask {
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        println!("==> fetching vulnerabilities");

        let mut targets: Vec<Vulnerability> = match self.list().await {
            Ok(vulnerabilities) => vulnerabilities,
            Err(e) => {
                return Err(Error::Vulnerability(format!("run::{}", e)));
            }
        };

        if targets.is_empty() {
            return Err(Error::Snyk("run::no_vulnerabilities".to_string()));
        }

        let total = targets.len();
        println!("==> processing {} vulnerabilities...", total);
        task.count = targets.len() as u64;

        let mut iteration = 0;
        let mut errors = HashMap::new();

        for vulnerability in targets.iter_mut() {
            iteration += 1;
            println!("==> processing iteration {} of {}", iteration, total);

            match self.process_target(vulnerability).await {
                Ok(_) => {
                    println!("==> iteration {} succeeded", iteration);
                }
                Err(e) => {
                    // Don't fail on a single error.
                    println!("==> iteration {} failed with error: {}", iteration, e);
                    errors.insert(vulnerability.purl.clone(), e.to_string());
                }
            }
        }

        Ok(errors)
    }
}

impl Service<Vulnerability> for SyncTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<Task> for SyncTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl SyncTask {
    /// Factory method to create new instance of type.
    pub fn new(store: Arc<Store>) -> SyncTask {
        let client = Client::new();

        SyncTask { store, client }
    }

    pub(in crate::tasks::enrichments::vulnerabilities::epss) async fn process_target(
        &self,
        vulnerability: &mut Vulnerability,
    ) -> Result<(), Error> {
        let cve = match &vulnerability.cve {
            None => {
                return Err(Error::Vulnerability("cve_none".to_string()));
            }
            Some(cve) => cve,
        };

        vulnerability.epss_score = match self.client.score(cve.clone()).await {
            Ok(score) => Some(score),
            Err(e) => {
                return Err(Error::Vulnerability(e.to_string()));
            }
        };

        self.update(vulnerability)
            .await
            .map_err(|e| Error::Vulnerability(e.to_string()))
    }
}
