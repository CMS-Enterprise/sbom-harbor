use crate::entities::packages::{Finding, Purl};
use crate::entities::scans::Scan;
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::findings::snyk::changeset::ChangeSet;
use crate::services::findings::{FindingProvider, FindingService};
use crate::services::packages::PackageService;
use crate::services::scans::ScanProvider;
use crate::services::snyk::{SnykRef, SnykService, SNYK_DISCRIMINATOR};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use tracing::log::debug;

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
        let mut change_set = ChangeSet::new(scan);

        // Populate the ChangeSet
        match self.scan_targets(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
                return Err(Error::Snyk(format!("scan::{}", e).to_string()));
            }
        };

        Ok(())
    }
}

impl FindingScanProvider {
    /// Factory method to create new instance of type.
    pub(crate) fn new(
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
        change_set: &mut ChangeSet<'_>,
    ) -> Result<(), Error> {
        let mut purls: Vec<Purl> = match self.list().await {
            Ok(purls) => purls,
            Err(e) => {
                return Err(Error::Enrichment(format!("scan_purls::{}", e)));
            }
        };

        println!("processing findings for {} purls...", purls.len());
        let mut iteration = 1;

        for purl in purls.iter_mut() {
            println!(
                "==> attempting to process iteration {} for purl {}",
                iteration, purl.purl
            );
            iteration = iteration + 1;
            match self.scan_purl(change_set, purl).await {
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

    pub(in crate::services::findings::snyk) async fn scan_purl(
        &self,
        change_set: &mut ChangeSet<'_>,
        purl: &mut Purl,
    ) -> Result<(), Error> {
        let scan_ref = match change_set.track(purl) {
            Ok(scan_ref) => scan_ref,
            Err(e) => {
                change_set.error(purl, e.to_string());
                return Err(Error::Enrichment(e.to_string()));
            }
        };

        let findings = match self.findings_by_purl(purl).await {
            Ok(findings) => {
                purl.findings(findings.clone());
                findings
            }
            Err(e) => {
                change_set.error(purl, e.to_string());
                return Err(Error::Enrichment(e.to_string()));
            }
        };

        // TODO: Store file_path somewhere?
        let _file_path = match self
            .findings
            .store_by_purl(purl.purl.clone(), findings, &scan_ref)
            .await
        {
            Ok(file_path) => file_path,
            Err(e) => {
                change_set.error(purl, e.to_string());
                return Ok(());
            }
        };

        self.commit_findings(change_set, purl).await
    }

    /// Transaction script for saving scan results to data store.
    pub(in crate::services::findings::snyk) async fn commit_findings(
        &self,
        change_set: &mut ChangeSet<'_>,
        purl: &mut Purl,
    ) -> Result<(), Error> {
        match self.packages.upsert_purl(purl).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("commit_findings::update::purl_id::{}::{}", purl.id, e);
                println!("{}", msg);
                change_set.error(purl, msg);
            }
        }

        Ok(())
    }

    pub async fn findings_by_purl(&self, purl: &mut Purl) -> Result<Option<Vec<Finding>>, Error> {
        // Make this a HashMap so we don't process the same org multiple times.
        let mut org_ids = HashMap::<String, bool>::new();
        purl.xrefs.iter().for_each(|x| {
            if x.kind == XrefKind::External(SNYK_DISCRIMINATOR.to_string()) {
                match x.map.get("orgId") {
                    None => {}
                    Some(org_id) => {
                        org_ids.insert(org_id.to_owned(), true);
                    }
                }
            }
        });

        let mut findings = vec![];
        for org_id in org_ids {
            match self
                .snyk
                .findings(org_id.0.as_str(), purl.purl.as_str(), purl.xrefs.clone())
                .await
            {
                Ok(f) => match f {
                    Some(f) => {
                        let mut f = f;
                        findings.append(&mut f);
                    }
                    _ => {}
                },
                Err(e) => {
                    // TODO: This error gets lost.
                    debug!("findings_by_purl::{}", e);
                }
            };
        }

        if findings.is_empty() {
            return Ok(None);
        }

        Ok(Some(findings))
    }
}
