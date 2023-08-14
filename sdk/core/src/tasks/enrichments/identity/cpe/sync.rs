use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use platform::persistence::mongodb::{Service, Store};

use crate::entities::datasets::{Purl2Cpes, PurlPlusId};
use crate::entities::packages::Package;
use crate::entities::tasks::Task;
use crate::services::analytics::sboms::service::AnalyticService;
use crate::services::purl2cpe::service::Purl2CpeService;
use crate::tasks::TaskProvider;
use crate::Error;

/// Synchronizes the subset of [Package] entries with the kind of
/// `dependency` and no `cpe` with a cpe derived from their purl(s).
#[derive(Debug)]
pub struct SyncTask {
    store: Arc<Store>,
    analytic_service: AnalyticService,
    purl_2_cpe_service: Purl2CpeService,
}

#[async_trait]
impl TaskProvider for SyncTask {
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        println!("==> fetching dependency Packages with no cpe");
        let mut errors = HashMap::new();

        let ids_and_purls_with_null_cpe: Vec<PurlPlusId> = self
            .analytic_service
            .get_dependant_package_purls_with_null_cpe()
            .await?;

        for id_and_purl in ids_and_purls_with_null_cpe.clone() {
            let id = id_and_purl.id.clone();
            let purl = id_and_purl.purl.clone();

            println!(
                "==> Attempting to add cpe to Package with purl: {}, id({})",
                purl.clone(),
                id.clone()
            );

            let cpe_opt = match self.purl_2_cpe_service.get_cpe(purl.to_string()).await {
                Err(err) => {
                    println!(
                        "==> Error getting cpe for: {}, id({}): {}",
                        purl.clone(),
                        id.clone(),
                        err
                    );

                    task.err_total += 1;
                    task.ref_errs(id.clone(), format!("no_cpe_for_{}", err));
                    errors.insert(id.clone(), format!("no_cpe_for_{}", err));
                    continue;
                }
                Ok(cpe_opt) => cpe_opt,
            };

            let cpe = match cpe_opt {
                None => {
                    println!("==> CPE for {}, id({}) is None", purl.clone(), id.clone());

                    task.err_total += 1;
                    let value = format!("no_cpe_for_{}", purl.clone());
                    task.ref_errs(id.clone(), format!("no_cpe_for_{}", value.clone()));
                    errors.insert(id.clone(), format!("no_cpe_for_{}", value.clone()));

                    String::from("unknown")
                }
                Some(cpe) => {
                    println!(
                        "==> Found CPE({}) for({}), updating Package...",
                        cpe.clone(),
                        purl.clone()
                    );
                    cpe
                }
            };

            match self
                .purl_2_cpe_service
                .update_package_with_cpe(id.clone(), cpe.clone())
                .await
            {
                Err(err) => {
                    println!(
                        "==> Error attempting to update Package: {}, id({}), with cpe({}): {}",
                        purl.clone(),
                        id.clone(),
                        cpe,
                        err
                    );

                    task.err_total += 1;
                    let value = format!("unable_to_update_document_{}", err);
                    task.ref_errs(id.clone(), format!("no_cpe_for_{}", value.clone()));
                    errors.insert(id.clone(), format!("no_cpe_for_{}", value));
                }
                Ok(()) => {
                    println!(
                        "==> Success! Package({}) with purl({}) updated cpe({})",
                        id, purl, cpe
                    );
                }
            }
        }

        Ok(errors)
    }
}

impl Service<Package> for SyncTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<Purl2Cpes> for SyncTask {
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
    /// Conventional Constructor.
    pub fn new(
        store: Arc<Store>,
        analytic_service: AnalyticService,
        purl_2_cpe_service: Purl2CpeService,
    ) -> SyncTask {
        SyncTask {
            store,
            analytic_service,
            purl_2_cpe_service,
        }
    }
}
