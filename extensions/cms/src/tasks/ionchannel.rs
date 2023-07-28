use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use harbcore::entities::packages::{Package, PackageKind};
use harbcore::entities::tasks::Task;
use harbcore::errors::Error;
use harbcore::services::packages::PackageService;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::{Context, Service, Store};

use crate::services::ionchannel::IonChannelService;

/// [TaskProvider] that can store Ion Channel metrics enrichment data in The Harbor backend.
#[derive(Debug)]
pub struct IonChannelTask {
    store: Arc<Store>,
    packages: PackageService,
    ionchannel: IonChannelService,
}

impl IonChannelTask {
    /// Factory method to create new instance of type.
    pub async fn new(cx: Context, token: String, org_id: String) -> Result<IonChannelTask, Error> {
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Config(e.to_string()))?,
        );

        Ok(IonChannelTask {
            store: store.clone(),
            packages: PackageService::new(store.clone()),
            ionchannel: IonChannelService::new(store, token, org_id),
        })
    }
}

impl Service<Task> for IonChannelTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

#[async_trait]
impl TaskProvider for IonChannelTask {
    /// Iterates over all dependency purls in the Package collection and attempts to retrieve
    /// metrics data for each.
    async fn run(
        &self,
        task: &mut Task,
    ) -> Result<HashMap<String, String>, harbcore::errors::Error> {
        println!("==> fetching packages");

        // Retrieve the list of Packages.
        let packages: Vec<Package> = match self
            .packages
            .query(HashMap::from([(
                "kind",
                format!("{}", PackageKind::Dependency).as_str(),
            )]))
            .await
        {
            Ok(packages) => packages,
            Err(e) => {
                return Err(Error::Snyk(format!("run::{}", e)));
            }
        };

        let total = packages.len();
        println!("==> processing metrics for {} packages...", total);

        task.count = packages.len() as u64;

        let mut iteration = 0;
        let mut errors = HashMap::new();

        for package in packages.iter() {
            iteration += 1;

            let purl = match &package.purl {
                None => {
                    errors.insert(package.id.clone(), "purl_none".to_string());
                    continue;
                }
                Some(purl) => purl.as_str(),
            };

            println!(
                "==> processing metrics iteration {} of {} for package {}",
                iteration, total, purl
            );

            let result = match self
                .ionchannel
                .metrics(None, Some(purl.clone()), None)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    println!(
                        "==> error processing metrics iteration {}: {}",
                        iteration, e
                    );
                    errors.insert(purl.to_string(), e.to_string());
                    continue;
                }
            };

            match self
                .ionchannel
                .save_metrics(purl.clone(), result.clone())
                .await
            {
                Ok(errs) => {
                    if errs.len() > 0 {
                        for (k, v) in errs.iter() {
                            println!(
                                "==> error processing metrics iteration {}: {}",
                                iteration, v
                            );
                            errors.insert(k.to_string(), v.to_string());
                        }
                        continue;
                    } else {
                        println!("==> success processing metrics iteration {}", iteration);
                    }
                }
                Err(e) => {
                    println!(
                        "==> error processing metrics iteration {}: {}",
                        iteration, e
                    );
                    errors.insert(purl.to_string(), e.to_string());
                    continue;
                }
            }

            println!("==> process metrics result {:#?}", result);
        }

        // Return error summary.
        Ok(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use harbcore::config::*;
    use harbcore::entities::tasks::TaskKind;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_run_metrics() -> Result<(), Error> {
        let cx = dev_context(None).map_err(|e| Error::Config(e.to_string()))?;
        let token = ion_channel_token().map_err(|e| Error::Config(e.to_string()))?;
        let org_id = ion_channel_org_id().map_err(|e| Error::Config(e.to_string()))?;

        let mut task: Task = Task::new(TaskKind::Extension("ionchannel::metrics".to_string()))
            .map_err(|e| Error::Fisma(e.to_string()))?;

        let provider = IonChannelTask::new(cx, token, org_id)
            .await
            .map_err(|e| Error::Fisma(e.to_string()))?;

        provider
            .execute(&mut task)
            .await
            .map_err(|e| Error::Fisma(e.to_string()))
    }
}
