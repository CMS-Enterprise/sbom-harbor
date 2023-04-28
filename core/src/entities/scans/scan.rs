use crate::entities::packages::{Finding, FindingProviderKind};
use crate::entities::sboms::{Sbom, SbomProviderKind};
use crate::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::time::Duration;

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

    /// The total number of items to be processed by the [Scan].
    pub count: u64,

    /// The unix timestamp for when the Scan was created.
    pub timestamp: u64,

    /// Human readable start time.
    pub start: DateTime<Utc>,

    /// Human readable end time.
    pub finish: DateTime<Utc>,

    /// The duration of the completed [Scan] in seconds.
    pub duration_seconds: i64,

    /// Result of the [Scan].
    pub status: ScanStatus,

    /// Optional error message if the [Scan] failed.
    pub err: Option<String>,

    /// Map of recoverable errors that occurred during the [Scan]. Used to track recoverable
    /// errors and the scan target that produced the error.
    pub ref_errs: Option<HashMap<String, String>>,

    /// The total count of errors encountered during the scan.
    pub err_total: u64,
}

impl Scan {
    /// Factory method to create new instance of type.
    pub fn new(kind: ScanKind, status: ScanStatus, count: Option<u64>) -> Result<Scan, Error> {
        let timestamp = platform::time::timestamp().map_err(|e| Error::Runtime(e.to_string()))?;
        let now = Utc::now();

        let count = match count {
            None => 0,
            Some(count) => count,
        };

        Ok(Scan {
            id: "".to_string(),
            kind,
            count,
            timestamp,
            start: now.clone(),
            finish: now,
            duration_seconds: 0,
            status,
            err: None,
            ref_errs: None,
            err_total: 0,
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
    Finding(FindingProviderKind),
    /// Scan was performed to assess Sboms.
    Sbom(SbomProviderKind),
}

/// Reference to an instance of a [Scan]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScanRef {
    /// The unique identifier for the [Scan] batch.
    pub scan_id: String,

    /// The unique identifier for the [Scan] target.
    pub target_id: String,

    /// The Scan iteration for the target being scanned instance. Forward-only incrementing counter.
    pub iteration: u32,

    /// Optional error message if the [Scan] failed for this target.
    pub err: Option<String>,
}

impl ScanRef {
    pub fn new(scan: &Scan, target_id: String, iteration: u32) -> Self {
        Self {
            scan_id: scan.id.clone(),
            target_id,
            iteration,
            err: None,
        }
    }

    pub fn eq(&self, other: &ScanRef) -> bool {
        self.scan_id.eq(&other.scan_id)
            && self.target_id == other.target_id
            && self.iteration == other.iteration
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
