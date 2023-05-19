use crate::entities::enrichments::Vulnerability;
use crate::entities::packages::Package;
use crate::entities::tasks::Task;
use crate::entities::xrefs::XrefKind;
use crate::services::enrichments::vulnerabilities::VulnerabilityService;
use crate::services::packages::PackageService;
use crate::services::snyk::{SnykService, SNYK_DISCRIMINATOR};
use crate::services::tasks::TaskProvider;
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Service, Store};
use std::collections::HashMap;
use std::sync::Arc;

/// Analyzes the full set of [Purl] entities that have a Snyk [Xref] for new [Vulnerability]s.
#[derive(Debug)]
pub struct VulnerabilityScanTask {
    store: Arc<Store>,
    snyk: SnykService,
    pub(in crate::services::enrichments::vulnerabilities::snyk) packages: PackageService,
    vulnerabilities: VulnerabilityService,
}

#[async_trait]
impl TaskProvider for VulnerabilityScanTask {
    /// Builds the [Task] and [Vulnerability] results.
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        println!("==> fetching purls");

        // TODO: This needs to actually constrain on Packages that have a Snyk Ref once other
        // Providers start writing to the data store.
        let mut targets: Vec<Package> = match self.packages.list().await {
            Ok(packages) => packages,
            Err(e) => {
                return Err(Error::Vulnerability(format!("run::{}", e)));
            }
        };

        if targets.is_empty() {
            return Err(Error::Snyk("run::no_purls".to_string()));
        }

        let total = targets.len();
        println!("==> processing vulnerabilities for {} purls...", total);
        task.count = targets.len() as u64;

        let mut iteration = 0;
        let mut errors = HashMap::new();

        for package in targets.iter_mut() {
            iteration += 1;
            let purl = match &package.purl {
                None => {
                    errors.insert(package.id.clone(), "package_purl_none".to_string());
                    continue;
                }
                Some(purl) => purl.clone(),
            };

            println!(
                "==> processing iteration {} of {} for purl {}",
                iteration, total, purl
            );

            match self.process_target(package, task).await {
                Ok(_) => {
                    println!("==> iteration {} succeeded", iteration);
                }
                Err(e) => {
                    // Don't fail on a single error.
                    println!("==> iteration {} failed with error: {}", iteration, e);
                    errors.insert(purl, e.to_string());
                }
            }
        }

        Ok(errors)
    }
}

impl Service<Vulnerability> for VulnerabilityScanTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<Task> for VulnerabilityScanTask {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl VulnerabilityScanTask {
    /// Factory method to create new instance of type.
    pub fn new(
        store: Arc<Store>,
        snyk: SnykService,
        packages: PackageService,
        vulnerabilities: VulnerabilityService,
    ) -> Result<VulnerabilityScanTask, Error> {
        Ok(VulnerabilityScanTask {
            store,
            snyk,
            packages,
            vulnerabilities,
        })
    }

    pub(in crate::services::enrichments::vulnerabilities::snyk) async fn process_target(
        &self,
        package: &mut Package,
        task: &Task,
    ) -> Result<(), Error> {
        let purl = match &package.purl {
            None => return Err(Error::Vulnerability("package_purl_none".to_string())),
            Some(purl) => purl.clone(),
        };

        // Load existing vulnerabilities
        package.vulnerabilities = self.query(HashMap::from([("purl", purl.as_str())])).await?;
        let task_ref = package.join_task(purl.clone(), task)?;

        let new_vulnerabilities = match self.vulnerabilities_by_purl(package).await {
            Ok(vulnerabilities) => vulnerabilities,
            Err(e) => {
                return Err(Error::Vulnerability(e.to_string()));
            }
        };

        let mut new_vulnerabilities = match new_vulnerabilities {
            None => {
                println!("==> no vulnerabilities for {}", purl);
                return Ok(());
            }
            Some(vulnerabilities) => {
                println!(
                    "==> found {} vulnerabilities for {}",
                    vulnerabilities.len(),
                    purl
                );
                vulnerabilities
            }
        };

        // TODO: Store file_path somewhere?
        let _file_path = match self.vulnerabilities.store_by_purl(package, &task_ref).await {
            Ok(file_path) => file_path,
            Err(e) => {
                return Err(Error::Vulnerability(e.to_string()));
            }
        };

        for vulnerability in new_vulnerabilities.iter_mut() {
            let task_ref = match vulnerability.join_task(task) {
                Ok(task_ref) => task_ref,
                Err(e) => {
                    let msg = format!("purl::task_refs::{}", e);
                    return Err(Error::Vulnerability(msg));
                }
            };

            self.vulnerabilities.upsert_by_purl(vulnerability).await?;

            package.task_refs(&task_ref);
            package.vulnerabilities(vulnerability)
        }

        self.packages.upsert_package_by_purl(package, None).await
    }

    /// Retrieves a set of native Snyk Issues from the API and converts them to a native Harbor
    /// [Vulnerability].
    pub async fn vulnerabilities_by_purl(
        &self,
        package: &Package,
    ) -> Result<Option<Vec<Vulnerability>>, Error> {
        let purl = match &package.purl {
            None => {
                return Err(Error::Snyk("package_purl_none".to_string()));
            }
            Some(purl) => purl.clone(),
        };

        // TODO: Validate getting this for a single org is good enough. We seem to get dupes.
        let xref = package.xrefs.iter().find(|x| {
            x.kind == XrefKind::External(SNYK_DISCRIMINATOR.to_string())
                && x.map.get("orgId").is_some()
        });

        let xref = match xref {
            None => {
                return Err(Error::Snyk("snyk_ref_none".to_string()));
            }
            Some(xref) => xref,
        };

        let org_id = xref.map.get("orgId").unwrap();

        let vulnerabilities = match self
            .snyk
            .vulnerabilities(org_id.as_str(), purl.as_str())
            .await
        {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::Snyk(format!("vulnerabilities_by_purl::{}", e)));
            }
        };

        let vulnerabilities = match vulnerabilities {
            None => {
                return Ok(None);
            }
            Some(vulnerabilities) => vulnerabilities,
        };

        if vulnerabilities.is_empty() {
            return Ok(None);
        }

        Ok(Some(vulnerabilities))
    }
}
