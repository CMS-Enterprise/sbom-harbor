use crate::entities::packages::Purl;
use crate::entities::scans::{Scan, ScanRef};
use crate::Error;
use platform::mongodb::{Context, MongoDocument};
use tracing::log::debug;

pub(crate) struct ChangeSet<'a> {
    /// The [Scan] instance used to relate all changes in the changeset.
    pub scan: &'a mut Scan,
}

impl ChangeSet<'_> {
    pub(in crate::services::findings::snyk) fn new(scan: &mut Scan) -> ChangeSet {
        ChangeSet { scan }
    }

    /// Track a Findings Scan.
    pub(in crate::services::findings::snyk) fn track(
        &mut self,
        purl: &mut Purl,
    ) -> Result<ScanRef, Error> {
        match purl.scan_refs(self.scan) {
            Ok(scan_ref) => Ok(scan_ref),
            Err(e) => {
                let msg = format!("changeset::error::purl::scan_refs::{}", e);
                return Err(Error::Enrichment(msg));
            }
        }
    }

    pub(in crate::services::findings::snyk) fn error(&mut self, purl: &mut Purl, error: String) {
        self.scan.ref_errs(purl.purl.clone(), error.clone());

        match purl.scan_ref_err(self.scan, Some(error)) {
            Ok(_) => {}
            Err(e) => {
                debug!("changeset::error::purl::scan_refs::{}", e);
            }
        }
    }
}
