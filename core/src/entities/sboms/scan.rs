use crate::entities::sboms::{Finding, FindingProviderKind, Sbom};
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A [Scan] is a value type that contains the results of an enrichment cycle where the SBOM is
/// assessed for vulnerability [Findings].
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scan {
    /// Indicates which enrichment provider performed the Scan.
    pub provider: FindingProviderKind,

    /// The Scan iteration for the SBOM instance. Forward-only incrementing counter.
    pub iteration: u32,

    /// The unix timestamp for when the Scan was performed.
    pub timestamp: u64,

    /// Result of the [Scan].
    pub status: ScanStatus,

    /// Findings that were a result of the [Scan].
    pub findings: Option<Vec<Finding>>,
}

impl Scan {
    pub fn new(
        provider: FindingProviderKind,
        status: ScanStatus,
        findings: Option<Vec<Finding>>,
    ) -> Result<Scan, Error> {
        let timestamp = platform::time::timestamp().map_err(|e| Error::Entity(e.to_string()))?;

        Ok(Scan {
            provider,
            iteration: 1,
            timestamp,
            status,
            findings,
        })
    }
}

/// Used to track [Scan] results and errors.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ScanStatus {
    /// Scan completed successfully.
    Complete,
    /// Scan completed with recoverable errors.
    CompleteWithErrors(String),
    /// Scan completed with unrecoverable errors.
    Failed(String),
}
