use crate::entities::packages::Purl;
use crate::entities::sboms::Sbom;
use crate::entities::scans::ScanRef;
use crate::services::findings::StorageProvider;
use crate::Error;
use platform::mongodb::{Context, Service};
use std::fmt::Debug;

/// Provides Finding related data management capabilities.
#[derive(Debug)]
pub struct FindingService {
    cx: Context,
    pub(crate) storage: Box<dyn StorageProvider>,
}

impl Service<Sbom> for FindingService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl FindingService {
    /// Factory method to create new instances of type.
    pub fn new(cx: Context, storage: Box<dyn StorageProvider>) -> Self {
        Self { cx, storage }
    }

    /// Stores the set of [Finding] instances for a [Purl] using the configured [StorageProvider].
    pub async fn store_by_purl(
        &self,
        purl: &Purl,
        scan_ref: &ScanRef,
    ) -> Result<Option<String>, Error> {
        let findings = match &purl.findings {
            None => {
                return Ok(None);
            }
            Some(findings) => findings,
        };

        if findings.is_empty() {
            return Ok(None);
        }

        match self
            .storage
            .write(purl.purl.as_str(), findings, scan_ref, &purl.xrefs)
            .await
        {
            Ok(file_path) => Ok(Some(file_path)),
            Err(e) => Err(Error::Enrichment(format!(
                "finding::store_by_purl::write::{}",
                e
            ))),
        }
    }
}
