use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use harbcore::entities::packages::{Package, PackageKind};
use harbcore::entities::tasks::Task;
use harbcore::entities::xrefs::XrefKind;
use harbcore::errors::Error;
use harbcore::services::packages::PackageService;
use harbcore::services::snyk::SNYK_DISCRIMINATOR;
use harbcore::services::xrefs::XrefService;
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::{Context, Service, Store};

use crate::services::snyk::{extract_xref, SnykService};

/// Adds FISMA ID as an [Xref] to primary packages.
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

        // Retrieve the list of Packages.
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
                return Err(Error::Snyk(format!("run::{}", e)));
            }
        };

        // Retrieve the list of OrgTags.
        let tags = match self.snyk.org_tags().await {
            Ok(tags) => tags,
            Err(e) => return Err(Error::Snyk(e.to_string())),
        };

        let total = packages.len();
        println!("==> processing fisma xref for {} packages...", total);

        task.count = packages.len() as u64;

        let mut iteration = 0;
        let mut errors = HashMap::new();

        for package in packages.iter() {
            iteration += 1;

            let purl = match &package.purl {
                None => package.id.clone(),
                Some(purl) => purl.clone(),
            };

            println!(
                "==> processing fisma xref iteration {} of {} for package {}",
                iteration, total, purl
            );

            let snyk_ref = package
                .xrefs
                .iter()
                .find(|xref| xref.kind == XrefKind::External(SNYK_DISCRIMINATOR.to_string()));

            let org_id = match snyk_ref {
                Some(xref) => match xref.map.get("orgId") {
                    None => {
                        println!(
                            "==> skipping fisma xref for package {} with no organization_id",
                            purl
                        );
                        errors.insert(purl, "org_id_none".to_string());
                        continue;
                    }
                    Some(org_id) => org_id.clone(),
                },
                None => {
                    println!(
                        "==> skipping fisma xref for package {} without snyk_ref",
                        purl
                    );
                    continue;
                }
            };

            let tag = match tags
                .iter()
                .find(|tag| tag.id.is_some() && tag.id.unwrap().to_string() == org_id)
            {
                None => {
                    println!(
                        "==> skipping fisma xref for package {} with no corresponding org_tag",
                        purl
                    );
                    errors.insert(purl, "org_tag_none".to_string());
                    continue;
                }
                Some(tag) => tag,
            };

            let xref = match extract_xref(tag) {
                Ok(xref) => xref,
                Err(e) => {
                    println!("==> fisma extract_xref failed for {} with: {}", purl, e);
                    errors.insert(purl.clone(), e.to_string());
                    continue;
                }
            };

            match self
                .packages
                .save_xref(HashMap::from([("id", package.id.as_str())]), &xref)
                .await
            {
                Ok(_) => {
                    println!("==> fisma xref save succeeded for {}", purl);
                }
                Err(e) => {
                    println!("==> fisma xref save failed for {} with: {}", purl, e);
                    errors.insert(purl.clone(), e.to_string());
                    continue;
                }
            }
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
    async fn can_run() -> Result<(), Error> {
        let cx = dev_context(None).map_err(|e| Error::Config(e.to_string()))?;
        let token = snyk_token().map_err(|e| Error::Config(e.to_string()))?;

        let mut task: Task = Task::new(TaskKind::Extension("fisma".to_string()))
            .map_err(|e| Error::Fisma(e.to_string()))?;

        let provider = FismaTask::new(cx, token)
            .await
            .map_err(|e| Error::Fisma(e.to_string()))?;

        provider
            .execute(&mut task)
            .await
            .map_err(|e| Error::Fisma(e.to_string()))
    }
}
