use crate::entities::enrichment::{Scan, ScanRef};
use crate::entities::packages::{Finding, Purl};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::enrichment::snyk::changeset::ScanFindingsChangeSet;
use crate::services::enrichment::{FindingProvider, ScanProvider};
use crate::services::snyk::{IssueSnyk, SnykRef, SnykService};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;

impl ScanProvider<'_> for SnykService {}

impl Service<Scan> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<ScanRef> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

#[async_trait]
impl FindingProvider<'_> for SnykService {
    async fn scan(&self, scan: &mut Scan) -> Result<(), Error> {
        let mut change_set = ScanFindingsChangeSet::new(scan);

        // Populate the ChangeSet
        match self.scan_purls(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
                return Err(Error::Snyk(format!("scan::{}", e).to_string()));
            }
        };

        Ok(())
    }
}

impl SnykService {
    /// Builds the Scan and Finding results.
    pub(in crate::services::enrichment::snyk) async fn scan_purls(
        &self,
        change_set: &mut ScanFindingsChangeSet<'_>,
    ) -> Result<(), Error> {
        let mut purls: Vec<Purl> = match Service::<Purl>::list(self).await {
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

    pub(in crate::services::enrichment::snyk) async fn scan_purl(
        &self,
        change_set: &mut ScanFindingsChangeSet<'_>,
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
            .finding_service
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
    pub(in crate::services::enrichment::snyk) async fn commit_findings(
        &self,
        change_set: &mut ScanFindingsChangeSet<'_>,
        purl: &mut Purl,
    ) -> Result<(), Error> {
        match self.update(purl).await {
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
        let org_id = match SnykRef::org_id(&purl.xrefs) {
            None => {
                return Err(Error::Enrichment(
                    "scan_purl::snyk_ref::org_id_none".to_string(),
                ));
            }
            Some(org_id) => org_id.clone(),
        };

        self.findings(org_id.as_str(), purl.purl.as_str(), &purl.xrefs)
            .await
    }
}
