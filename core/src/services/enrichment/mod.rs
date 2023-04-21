use crate::entities::enrichment::{Scan, ScanKind, ScanStatus};
use crate::entities::packages::FindingProviderKind;
use crate::entities::sboms::SbomProviderKind;
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::Service;
use std::future::Future;
use tracing::log::debug;

/// Contains all Snyk related enrichment logic.
pub mod snyk;

#[async_trait]
pub trait ScanProvider<'a>: Service<Scan> {
    async fn commit_scan(&self, scan: &mut Scan) -> Result<(), Error> {
        match scan.err {
            None => match scan.ref_errs.iter().any(|scan_ref| !scan_ref.is_empty()) {
                true => scan.status = ScanStatus::CompleteWithErrors,
                false => {
                    scan.status = ScanStatus::Complete;
                }
            },
            Some(_) => {
                scan.status = ScanStatus::Failed;
            }
        }

        match self.update(scan).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!("commit_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        }
    }
}
/// Service that is capable of synchronizing one or more SBOMs from a dynamic source.
#[async_trait]
pub trait SbomProvider<'a>: ScanProvider<'a> {
    /// Sync an external Sbom source with Harbor.
    async fn enrich(&self, provider: SbomProviderKind) -> Result<(), Error> {
        let mut scan = match self.init_scan(provider).await {
            Ok(scan) => scan,
            Err(e) => {
                return Err(Error::Enrichment(format!("finding::init_scan::{}", e)));
            }
        };

        match self.scan(&mut scan).await {
            Ok(_) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
            }
        }

        self.commit_scan(&mut scan).await
    }

    async fn scan(&self, scan: &mut Scan) -> Result<(), Error>;

    async fn init_scan(&self, provider: SbomProviderKind) -> Result<Scan, Error> {
        let mut scan = match Scan::new(ScanKind::Finding, ScanStatus::Started, Some(provider), None)
        {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("init_scan::new_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        };

        match self.insert(&mut scan).await {
            Ok(_) => Ok(scan),
            Err(e) => {
                let msg = format!("init_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        }
    }
}

/// Service that is capable of determining Findings for a Purl.
#[async_trait]
pub trait FindingProvider<'a>: ScanProvider<'a> {
    /// Sync an external Findings source with Harbor.
    async fn enrich(&self, provider: FindingProviderKind) -> Result<(), Error> {
        let mut scan = match self.init_scan(provider).await {
            Ok(scan) => scan,
            Err(e) => {
                return Err(Error::Enrichment(format!("finding::init_scan::{}", e)));
            }
        };

        match self.scan(&mut scan).await {
            Ok(_) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
            }
        }

        self.commit_scan(&mut scan).await
    }

    async fn scan(&self, scan: &mut Scan) -> Result<(), Error>;

    async fn init_scan(&self, provider: FindingProviderKind) -> Result<Scan, Error> {
        let mut scan = match Scan::new(ScanKind::Finding, ScanStatus::Started, None, Some(provider))
        {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("init_scan::new_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        };

        match self.insert(&mut scan).await {
            Ok(_) => Ok(scan),
            Err(e) => {
                let msg = format!("init_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        }
    }
}
