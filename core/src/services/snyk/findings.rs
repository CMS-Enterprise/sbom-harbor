use crate::entities::cyclonedx::Bom;
use crate::entities::packages::Purl;
use crate::entities::sboms::{Finding, FindingProviderKind, Sbom, Scan, ScanStatus};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::findings::FindingProvider;
use crate::services::snyk::adapters::Issue;
use crate::services::snyk::{SnykRef, SnykService};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Context, Service};
use std::collections::HashMap;
use tracing::log::debug;
use crate::services::snyk::changeset::ChangeSet;

#[async_trait]
impl FindingProvider for SnykService {
    async fn sync(&self) -> Result<(), Error> {
        let mut sboms: Vec<Sbom> = match self.list().await {};

        let mut change_set = ChangeSet::new();

        for mut sbom in sboms.iter_mut() {
            match self.scan_sbom(sbom).await {
                Ok(_) => {}
                Err(_) => {}
            }
        }


        Ok(())
    }
}

impl SnykService {
    /// Perform a [Scan] on a single [Sbom] instance.
    pub async fn scan_sbom(&self, sbom: &mut Sbom) -> Result<(),
        Error> {
        let status = ScanStatus::Failed("scan_failure_unknown".to_string());
        let mut scan = match Scan::new(FindingProviderKind::Snyk, status, None ) {
            Ok(scan) => scan,
            Err(e) => {
                return Err(Error::Entity(e.to_string()));
            }
        };

        // TODO: Strongly typed conversion.
        let snyk_ref = match sbom.snyk_ref() {
            Ok(map) => map,
            Err(e) => {
                return Err(Error::Entity(e.to_string()));
            }
        };

        let org_id = match snyk_ref.get("org_id"){
            None => {return Err(Error::Entity("snyk_ref_org_id_none".to_string())); }
            Some(org_id) => org_id.as_str()
        };

        let findings = match self.findings(org_id, sbom.purl()?.as_str(), xrefs).await {
            Ok(findings) => findings,
            Err(e) => {
                return Err(Error::Entity(e.to_string()));
            }
        };

        self.finding_service.s

        sbom.scans(scan)?
    }

    pub async fn findings(
        &self,
        org_id: &str,
        purl: &str,
        xrefs: HashMap<XrefKind, Xref>,
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
