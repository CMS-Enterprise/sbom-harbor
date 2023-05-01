use crate::entities::packages::{Finding, FindingProviderKind, Purl};
use crate::entities::scans::{Scan, ScanKind};
use crate::entities::xrefs::XrefKind;
use crate::services::findings::FindingService;
use crate::services::packages::PackageService;
use crate::services::scans::ScanProvider;
use crate::services::snyk::{SnykService, SNYK_DISCRIMINATOR};
use crate::Error;
use async_std::stream;
use async_trait::async_trait;
use futures::stream::StreamExt;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct FindingScanProvider {
    scan: Arc<Mutex<Scan>>,
    cx: Context,
    snyk: SnykService,
    pub(in crate::services::findings::snyk) packages: PackageService,
    findings: FindingService,
    targets: Vec<Purl>,
}

impl Service<Purl> for FindingScanProvider {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Scan> for FindingScanProvider {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

#[async_trait]
impl ScanProvider for FindingScanProvider {
    fn current(&self) -> Arc<Mutex<Scan>> {
        self.scan.clone()
    }

    /// Builds the Scan and Finding results.
    async fn scan_targets(&self) -> Result<HashMap<String, String>, Error> {
        println!("processing findings for {} purls...", self.targets.len());
        let scan_id = self.current().lock().unwrap().id.clone();
        let mut purl_scanner = Arc::new(Mutex::new(PurlScanner::new(scan_id)));

        stream::from_iter(&self.targets)
            .for_each_concurrent(8, |purl| async move {
                //for purl in purls.iter_mut() {
                let mut purl = purl;
                let mut scanner = Arc::clone(&purl_scanner);
                *scanner.lock().unwrap().iteration += 1;
                let mut iteration = scanner.lock().unwrap().iteration;
                iteration += 1;
                scanner.lock().unwrap().iteration = iteration;

                println!(
                    "==> processing iteration {} for purl {}",
                    iteration, purl.purl
                );

                let result = scanner
                    .lock()
                    .unwrap()
                    .scan_target(&mut purl, &self.snyk, &self.packages, &self.findings)
                    .await;

                match result {
                    Ok(_) => {
                        println!("==> iteration {} succeeded", iteration);
                    }
                    Err(e) => {
                        // Don't fail on a single error.  Don't add errors to change_set, it should
                        // have been done by process_purl.
                        println!("==> iteration {} failed with error: {}", iteration, e);
                    }
                }
            })
            .await;
        //}

        // TODO: Emit Metric for errors
        let results = Arc::clone(&purl_scanner).lock().unwrap().errors.clone();

        Ok(results)
    }

    async fn load_targets(&mut self) -> Result<(), Error> {
        self.targets = self
            .list()
            .await
            .map_err(|e| Error::Scan(format!("load_targets::{}", e)))?;

        if self.targets.is_empty() {
            return Err(Error::Snyk("scan_targets::no_purls".to_string()));
        }

        Ok(())
    }

    fn target_count(&self) -> u64 {
        self.targets.len() as u64
    }
}

impl FindingScanProvider {
    /// Factory method to create new instance of type.
    pub fn new(
        cx: Context,
        snyk: SnykService,
        packages: PackageService,
        findings: FindingService,
    ) -> Result<FindingScanProvider, Error> {
        let scan = Scan::new(ScanKind::Finding(FindingProviderKind::Snyk))?;

        Ok(FindingScanProvider {
            cx,
            snyk,
            packages,
            findings,
            scan: Arc::new(Mutex::new(scan)),
            targets: vec![],
        })
    }
}

struct PurlScanner {
    iteration: u64,
    scan_id: String,
    errors: HashMap<String, String>,
}

impl PurlScanner {
    fn new(scan_id: String) -> Self {
        let errors: HashMap<String, String> = HashMap::new();

        Self {
            iteration: 0,
            scan_id,
            errors,
        }
    }

    pub(in crate::services::findings::snyk) async fn scan_target(
        &mut self,
        purl: &mut Purl,
        snyk: &SnykService,
        packages: &PackageService,
        findings: &FindingService,
    ) -> Result<(), Error> {
        let scan_ref = match purl.join_scan(self.scan_id.as_str()) {
            Ok(scan_ref) => scan_ref,
            Err(e) => {
                let msg = format!("join_scan::{}", e);
                self.errors.insert(purl.purl.clone(), msg.clone());
                return Ok(());
            }
        };

        let results = match self.findings_by_purl(purl, snyk).await {
            Ok(findings) => findings,
            Err(e) => {
                let msg = format!("findings_by_purl::{}", e);
                self.errors.insert(purl.purl.clone(), msg.clone());
                return Ok(());
            }
        };

        match results {
            None => {
                println!("==> no findings for {}", purl.purl);
                return Ok(());
            }
            Some(results) => {
                println!("==> found {} findings", results.len());
                purl.findings(&results);
            }
        }

        // TODO: Store file_path somewhere?
        let _file_path = match findings.store_by_purl(purl, &scan_ref).await {
            Ok(file_path) => file_path,
            Err(e) => {
                let msg = format!("store_by_purl::{}", e);
                self.errors.insert(purl.purl.clone(), msg.clone());
                return Ok(());
            }
        };

        self.commit_target(purl, packages).await
    }

    /// Transaction script for saving scan results to data store.
    pub(in crate::services::findings::snyk) async fn commit_target(
        &mut self,
        purl: &mut Purl,
        packages: &PackageService,
    ) -> Result<(), Error> {
        match packages.upsert_purl(purl).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("commit_findings::update::purl_id::{}::{}", purl.id, e);
                println!("{}", msg);
                self.errors.insert(purl.purl.clone(), msg);
            }
        }

        Ok(())
    }

    pub async fn findings_by_purl(
        &self,
        purl: &mut Purl,
        snyk: &SnykService,
    ) -> Result<Option<Vec<Finding>>, Error> {
        // TODO: Validate getting this for a single org is good enough. We seem to get dupes.
        let xref = purl.xrefs.iter().find(|x| {
            x.kind == XrefKind::External(SNYK_DISCRIMINATOR.to_string())
                && x.map.get("orgId").is_some()
        });

        let org_id = xref.unwrap().map.get("orgId").unwrap();

        let findings = match snyk
            .findings(org_id.as_str(), purl.purl.as_str(), purl.xrefs.clone())
            .await
        {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::Snyk(format!("findings_by_purl::{}", e)));
            }
        };

        let findings = match findings {
            None => {
                return Ok(None);
            }
            Some(findings) => findings,
        };

        if findings.is_empty() {
            return Ok(None);
        }

        Ok(Some(findings))
    }
}
