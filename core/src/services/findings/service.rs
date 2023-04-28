use crate::entities::packages::Finding;
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
    pub fn new(cx: Context, storage: Box<dyn StorageProvider>) -> Self {
        Self { cx, storage }
    }

    pub async fn store_by_purl(
        &self,
        purl: String,
        findings: Option<Vec<Finding>>,
        scan_ref: &ScanRef,
    ) -> Result<Option<String>, Error> {
        let findings = match findings {
            None => {
                return Ok(None);
            }
            Some(findings) => findings,
        };

        match findings.is_empty() {
            true => {
                return Ok(None);
            }
            false => {}
        }

        match self.storage.write(purl.as_str(), &findings, scan_ref).await {
            Ok(file_path) => Ok(Some(file_path)),
            Err(e) => {
                return Err(Error::Enrichment(format!(
                    "finding::store_by_purl::write::{}",
                    e.to_string()
                )));
            }
        }
    }
}