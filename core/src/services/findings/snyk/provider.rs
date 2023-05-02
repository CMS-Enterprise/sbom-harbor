use crate::entities::packages::{Finding, Purl};
use crate::entities::scans::Scan;
use crate::entities::xrefs::XrefKind;
use crate::services::findings::{FindingProvider, FindingService};
use crate::services::packages::PackageService;
use crate::services::scans::ScanProvider;
use crate::services::snyk::{SnykService, SNYK_DISCRIMINATOR};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Context, Service};
use tracing::log::debug;

/// Analyzes the full set of [Purl] entities that have a Snyk [Xref] for new [Finding]s.
#[derive(Debug)]
pub struct FindingScanProvider {
    cx: Context,
    snyk: SnykService,
    pub(in crate::services::findings::snyk) packages: PackageService,
    findings: FindingService,
}

impl ScanProvider<'_> for FindingScanProvider {}

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
impl FindingProvider<'_> for FindingScanProvider {
    async fn scan(&self, scan: &mut Scan) -> Result<(), Error> {
        // Populate the ChangeSet
        match self.scan_targets(scan).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
                return Err(Error::Snyk(format!("scan::{}", e)));
            }
        };

        Ok(())
    }
}

impl FindingScanProvider {
    /// Factory method to create new instance of type.
    pub fn new(
        cx: Context,
        snyk: SnykService,
        packages: PackageService,
        findings: FindingService,
    ) -> Self {
        Self {
            cx,
            snyk,
            packages,
            findings,
        }
    }

    /// Builds the Scan and Finding results.
    pub(in crate::services::findings::snyk) async fn scan_targets(
        &self,
        scan: &mut Scan,
    ) -> Result<(), Error> {
        // TODO: This needs to actually constrain on Purls that have a Snyk Ref once other
        // Providers start writing to the data store.
        let mut purls: Vec<Purl> = match self.list().await {
            Ok(purls) => purls,
            Err(e) => {
                return Err(Error::Finding(format!("scan_purls::{}", e)));
            }
        };

        println!("processing findings for {} purls...", purls.len());
        let mut iteration = 1;

        for purl in purls.iter_mut() {
            println!(
                "==> processing iteration {} for purl {}",
                iteration, purl.purl
            );
            iteration += 1;
            match self.scan_target(scan, purl).await {
                Ok(_) => {
                    println!("==> iteration {} succeeded", iteration);
                }
                Err(e) => {
                    // Don't fail on a single error.  Don't add errors to change_set, it should
                    // have been done by process_purl.
                    println!("==> iteration {} failed with error: {}", iteration, e);
                }
            }
        }

        Ok(())
    }

    pub(in crate::services::findings::snyk) async fn scan_target(
        &self,
        scan: &mut Scan,
        purl: &mut Purl,
    ) -> Result<(), Error> {
        let scan_ref = match purl.init_scan(scan) {
            Ok(scan_ref) => scan_ref,
            Err(e) => {
                let msg = format!("purl::scan_refs::{}", e);
                self.error(scan, purl, msg.clone());
                return Err(Error::Finding(msg));
            }
        };

        let findings = match self.findings_by_purl(purl).await {
            Ok(findings) => findings,
            Err(e) => {
                self.error(scan, purl, e.to_string());
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

        // TODO: Store file_path somewhere?
        let _file_path = match self.findings.store_by_purl(purl, &scan_ref).await {
            Ok(file_path) => file_path,
            Err(e) => {
                self.error(scan, purl, e.to_string());
                return Ok(());
            }
        };

        self.commit_target(scan, purl).await
    }

    /// Transaction script for saving scan results to data store.
    pub(in crate::services::findings::snyk) async fn commit_target(
        &self,
        scan: &mut Scan,
        purl: &mut Purl,
    ) -> Result<(), Error> {
        match self.packages.upsert_purl(purl).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("commit_findings::update::purl_id::{}::{}", purl.id, e);
                println!("{}", msg);
                self.error(scan, purl, msg);
            }
        }

        Ok(())
    }

    /// Stores the error detail in the [Scan] with a reference to the package url that was being
    /// processed when the error occurred.
    fn error(&self, scan: &mut Scan, purl: &mut Purl, error: String) {
        println!("==> error processing {}: {}", purl.purl, error);

        scan.ref_errs(purl.purl.clone(), error.clone());

        match purl.scan_err(scan, Some(error)) {
            Ok(_) => {}
            Err(e) => {
                debug!("scan_ref_none::{}::{}", purl.purl, e);
            }
        }
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
