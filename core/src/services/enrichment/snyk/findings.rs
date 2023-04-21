use crate::entities::enrichment::{Scan, ScanRef};
use crate::entities::packages::{Finding, Purl};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::enrichment::snyk::changeset::ScanFindingsChangeSet;
use crate::services::enrichment::{FindingProvider, ScanProvider};
use crate::services::snyk::{SnykRef, SnykService};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use tracing::log::debug;

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
        match self.build_findings(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
                return Err(Error::Snyk(format!("scan::{}", e).to_string()));
            }
        };

        match self.commit_findings(&mut change_set).await {
            Ok(()) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
                return Err(Error::Snyk(format!("scan::{}", e).to_string()));
            }
        }

        Ok(())
    }
}

impl SnykService {
    /// Builds the Scan and Finding results.
    pub(in crate::services::enrichment::snyk) async fn build_findings(
        &self,
        change_set: &mut ScanFindingsChangeSet<'_>,
    ) -> Result<(), Error> {
        let mut purls: Vec<Purl> = match Service::<Purl>::list(self).await {
            Ok(purls) => purls,
            Err(e) => {
                return Err(Error::Enrichment(format!("build_findings::{}", e)));
            }
        };

        for purl in purls.iter_mut() {
            match self.process_purl(change_set, purl).await {
                Ok(_) => {}
                Err(e) => {
                    // Don't fail on a single error.  Don't add errors to change_set, it should
                    // have been done by process_purl.
                    debug!("{}", e.to_string());
                }
            }
        }

        Ok(())
    }

    pub(in crate::services::enrichment::snyk) async fn process_purl(
        &self,
        change_set: &mut ScanFindingsChangeSet<'_>,
        purl: &mut Purl,
    ) -> Result<(), Error> {
        let findings = match self.scan_purl(purl).await {
            Ok(findings) => {
                purl.findings(findings.clone());
                findings
            }
            Err(e) => {
                change_set.error(purl, e.to_string());
                return Ok(());
            }
        };

        // TODO: Store file_path somewhere?
        let _file_path = match self
            .finding_service
            .store_by_purl(purl.purl.clone(), findings)
            .await
        {
            Ok(file_path) => file_path,
            Err(e) => {
                change_set.error(purl, e.to_string());
                return Ok(());
            }
        };

        match change_set.track(purl) {
            Ok(_) => {}
            Err(e) => {
                change_set.error(purl, e.to_string());
                return Ok(());
            }
        }

        Ok(())
    }

    /// Transaction script for saving scan results to data store.
    pub(in crate::services::enrichment::snyk) async fn commit_findings(
        &self,
        change_set: &mut ScanFindingsChangeSet<'_>,
    ) -> Result<(), Error> {
        for (_, purl) in change_set.purls.iter_mut() {
            // TODO: Log errors in the change_set Scan.
            match self.update(purl).await {
                Ok(_) => {}
                Err(e) => {
                    debug!("{}", e);
                }
            }
        }

        Ok(())
    }

    pub async fn scan_purl(&self, purl: &mut Purl) -> Result<Option<Vec<Finding>>, Error> {
        let org_id = match SnykRef::org_id(&purl.xrefs) {
            None => {
                return Err(Error::Enrichment(
                    "scan_purl::snyk_ref::org_id_none".to_string(),
                ));
            }
            Some(org_id) => org_id.clone(),
        };

        let snyk_refs = match &purl.xrefs {
            None => {
                return Err(Error::Enrichment("scan_purl::snyk_refs_none".to_string()));
            }
            Some(snyk_refs) => snyk_refs,
        };

        self.findings(org_id.as_str(), purl.purl.as_str(), &purl.xrefs)
            .await
    }

    pub(in crate::services::enrichment::snyk) async fn findings(
        &self,
        org_id: &str,
        purl: &str,
        xrefs: &Option<HashMap<XrefKind, Xref>>,
    ) -> Result<Option<Vec<Finding>>, Error> {
        let issues = match self.client.get_issues(org_id, purl).await {
            Ok(issues) => issues,
            Err(e) => {
                return Err(Error::Snyk(format!(
                    "snyk::issues: purl - {} - {}",
                    purl, e
                )));
            }
        };

        let issues = match issues {
            None => {
                return Ok(None);
            }
            Some(issues) => issues,
        };

        if issues.is_empty() {
            return Ok(None);
        }

        let mut results = vec![];

        issues.iter().for_each(|issue| {
            results.push(Finding::from_snyk(
                purl.to_string(),
                issue.clone(),
                xrefs.clone(),
            ));
        });

        Ok(Some(results))
    }
}
