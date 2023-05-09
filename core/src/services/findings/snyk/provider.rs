use crate::entities::packages::{Finding, Purl};
use crate::entities::scans::Scan;
use crate::entities::xrefs::XrefKind;
use crate::services::findings::FindingService;
use crate::services::packages::PackageService;
use crate::services::scans::ScanProvider;
use crate::services::snyk::{SnykService, SNYK_DISCRIMINATOR};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Service, Store};
use std::collections::HashMap;
use std::sync::Arc;

/// Analyzes the full set of [Purl] entities that have a Snyk [Xref] for new [Finding]s.
#[derive(Debug)]
pub struct FindingScanProvider {
    store: Arc<Store>,
    snyk: SnykService,
    pub(in crate::services::findings::snyk) packages: PackageService,
    findings: FindingService,
}

#[async_trait]
impl ScanProvider for FindingScanProvider {
    /// Builds the Scan and Finding results.
    async fn scan_targets(&self, scan: &mut Scan) -> Result<HashMap<String, String>, Error> {
        println!("==> fetching purls");

        // TODO: This needs to actually constrain on Purls that have a Snyk Ref once other
        // Providers start writing to the data store.
        let mut targets: Vec<Purl> = match self.list().await {
            Ok(purls) => purls,
            Err(e) => {
                return Err(Error::Finding(format!("scan_purls::{}", e)));
            }
        };

        if targets.is_empty() {
            return Err(Error::Snyk("scan_targets::no_purls".to_string()));
        }

        println!("==> processing findings for {} purls...", targets.len());
        scan.count = targets.len() as u64;

        let mut iteration = 0;
        let mut errors = HashMap::new();

        for purl in targets.iter_mut() {
            iteration += 1;
            println!(
                "==> processing iteration {} for purl {}",
                iteration, purl.purl
            );

            match self.scan_target(purl, scan).await {
                Ok(_) => {
                    println!("==> iteration {} succeeded", iteration);
                }
                Err(e) => {
                    // Don't fail on a single error.
                    println!("==> iteration {} failed with error: {}", iteration, e);
                    errors.insert(purl.purl.clone(), e.to_string());
                }
            }
        }

        Ok(errors)
    }
}

impl Service<Purl> for FindingScanProvider {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<Scan> for FindingScanProvider {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl FindingScanProvider {
    /// Factory method to create new instance of type.
    pub fn new(
        store: Arc<Store>,
        snyk: SnykService,
        packages: PackageService,
        findings: FindingService,
    ) -> Result<FindingScanProvider, Error> {
        Ok(FindingScanProvider {
            store,
            snyk,
            packages,
            findings,
        })
    }

    pub(in crate::services::findings::snyk) async fn scan_target(
        &self,
        purl: &mut Purl,
        scan: &Scan,
    ) -> Result<(), Error> {
        let findings = match self.findings_by_purl(purl).await {
            Ok(findings) => findings,
            Err(e) => {
                return Err(Error::Finding(e.to_string()));
            }
        };

        match findings {
            None => {
                println!("no findings for {}", purl.purl);
                return Ok(());
            }
            Some(findings) => {
                println!("==> found {} findings", findings.len());
                purl.findings(&findings);
            }
        }

        let scan_ref = match purl.init_scan(scan) {
            Ok(scan_ref) => scan_ref,
            Err(e) => {
                let msg = format!("purl::scan_refs::{}", e);
                return Err(Error::Finding(msg));
            }
        };

        // TODO: Store file_path somewhere?
        let _file_path = match self.findings.store_by_purl(purl, &scan_ref).await {
            Ok(file_path) => file_path,
            Err(e) => {
                return Err(Error::Finding(e.to_string()));
            }
        };

        self.packages.upsert_purl(purl).await
    }

    /// Retrieves a set of native Snyk Issues from the API and converts them to native Harbor
    /// [Finding]s.
    pub async fn findings_by_purl(&self, purl: &mut Purl) -> Result<Option<Vec<Finding>>, Error> {
        // TODO: Validate getting this for a single org is good enough. We seem to get dupes.
        let xref = purl.xrefs.iter().find(|x| {
            x.kind == XrefKind::External(SNYK_DISCRIMINATOR.to_string())
                && x.map.get("orgId").is_some()
        });

        let org_id = xref.unwrap().map.get("orgId").unwrap();

        let findings = match self
            .snyk
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
