use crate::entities::packages::{Finding, FindingProviderKind};
use crate::entities::sboms::{Sbom, SbomProviderKind};
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// A [Scan] is a value type that contains the results of an enrichment cycle where the SBOM is
/// assessed for vulnerability [Findings].
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scan {
    /// The unique identifier for the [Scan] batch.
    pub id: String,

    /// Discriminator indicating the type of scan operation performed.
    pub kind: ScanKind,

    // TODO: This is a code smell. Feels like we need separate scan types.
    /// Provider that performed the scan for Findings.
    pub finding_provider: Option<FindingProviderKind>,

    /// Provider the performed the scan for Sboms.
    pub sbom_provider: Option<SbomProviderKind>,

    /// The unix timestamp for when the Scan was performed.
    pub timestamp: u64,

    /// Result of the [Scan].
    pub status: ScanStatus,

    /// Optional error message if the [Scan] failed.
    pub err: Option<String>,

    /// Map of recoverable errors that occurred during the [Scan]. Used to track recoverable
    /// errors and the scan target that produced the error.
    pub ref_errs: Option<HashMap<String, String>>,
}

impl Scan {
    /// Factory method to create new instance of type.
    pub fn new(
        kind: ScanKind,
        status: ScanStatus,
        sbom_provider: Option<SbomProviderKind>,
        finding_provider: Option<FindingProviderKind>,
    ) -> Result<Scan, Error> {
        let timestamp = platform::time::timestamp().map_err(|e| Error::Runtime(e.to_string()))?;

        Ok(Scan {
            id: "".to_string(),
            kind,
            finding_provider,
            sbom_provider,
            timestamp,
            status,
            err: None,
            ref_errs: None,
        })
    }

    /// Add an error string for a specific target.
    pub fn ref_errs(&mut self, target_id: String, err: String) {
        match self.ref_errs.clone() {
            None => {
                self.ref_errs = Some(HashMap::from([(target_id, err)]));
            }
            Some(mut ref_errs) => {
                ref_errs.insert(target_id, err);
                self.ref_errs = Some(ref_errs);
            }
        }
    }
}

/// Discriminator indicating the type of scan operation performed and which provider performed
/// the [Scan].
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ScanKind {
    /// Scan was performed to assess Findings.
    Finding,
    /// Scan was performed to assess Sboms.
    Sbom,
}

/// Reference to an instance of a [Scan]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScanRef {
    /// The unique identifier for the [Scan] batch.
    pub id: String,

    /// The unique identifier for the [Scan] batch.
    pub scan_id: String,

    /// The Purl for the Scan target. Optional because Spdx won't have a Purl,
    pub purl: Option<String>,

    /// The Scan iteration for the target being scanned instance. Forward-only incrementing counter.
    pub iteration: u32,

    /// Optional error message if the [Scan] failed for this target.
    pub err: Option<String>,
}

impl ScanRef {
    pub fn new(scan: &Scan, purl: Option<String>) -> Self {
        Self {
            id: "".to_string(),
            scan_id: scan.id.clone(),
            purl,
            iteration: 0,
            err: None,
        }
    }
}

/// Used to track [Scan] results and errors.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ScanStatus {
    /// Scan started.
    Started,
    /// Scan completed successfully.
    Complete,
    /// Scan completed with recoverable errors.
    CompleteWithErrors,
    /// Scan completed with unrecoverable errors.
    Failed,
}
