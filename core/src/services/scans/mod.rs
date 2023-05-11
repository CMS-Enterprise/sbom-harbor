use crate::entities::scans::{Scan, ScanStatus};
use crate::Error;
use std::collections::HashMap;

use async_trait::async_trait;
use chrono::Utc;
use platform::mongodb::Service;
use tracing::log::debug;

/// Transaction script for a batch scanning operation.
#[async_trait]
pub trait ScanProvider: Service<Scan> {
    /// Implement this to load and process data.
    async fn scan_targets(&self, scan: &mut Scan) -> Result<HashMap<String, String>, Error>;

    /// Run the transaction script and store the results.
    async fn execute(&self, scan: &mut Scan) -> Result<(), Error> {
        match self.init_scan(scan).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("init_scan::failed::{}", e);
                println!("{}", msg);
                return Err(Error::Scan(msg));
            }
        }

        let errors = self.scan_targets(scan).await;

        match errors {
            Ok(errors) => {
                for (key, value) in errors {
                    println!("==> error processing {}: {}", key, value);
                    scan.ref_errs(key, value);
                }
            }
            Err(e) => {
                let msg = format!("scan_failed::{}", e);
                println!("{}", msg);

                scan.err = Some(msg);
            }
        };

        // TODO: Emit Metric for changeset totals.

        let result = self.commit_scan(scan).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => return Err(Error::Scan(e.to_string())),
        }
    }

    /// Inserts the scan record at the start of the transaction script.
    async fn init_scan(&self, scan: &mut Scan) -> Result<(), Error> {
        match self.insert(scan).await {
            Ok(()) => {}
            Err(e) => {
                let msg = format!("init_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Scan(msg));
            }
        };

        // TODO: Write tests.
        if scan.id.is_empty() {
            return Err(Error::Entity("scan_id_empty".to_string()));
        }

        Ok(())
    }

    /// Updates the scan record at the end of the transaction script.
    async fn commit_scan(&self, scan: &mut Scan) -> Result<(), Error> {
        match scan.err {
            None => {
                scan.err_total = scan
                    .ref_errs
                    .iter()
                    .filter(|ref_err| !ref_err.is_empty())
                    .count() as u64;

                match scan.err_total > 0 {
                    true => scan.status = ScanStatus::CompleteWithErrors,
                    false => {
                        scan.status = ScanStatus::Complete;
                    }
                }
            }
            Some(_) => {
                scan.status = ScanStatus::Failed;
            }
        }

        scan.finish = Utc::now();
        scan.duration_seconds = scan.finish.signed_duration_since(scan.start).num_seconds();

        match self.update(scan).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let complete_raw = match serde_json::to_string(&scan) {
                    Ok(raw) => raw,
                    Err(serde_err) => {
                        println!("error serializing scan: {}", serde_err);
                        "{ err: null }".to_string()
                    }
                };

                let msg = format!("commit_scan::store_failed::{} - {}", e, complete_raw);
                debug!("{}", msg);
                return Err(Error::Scan(msg));
            }
        }
    }
}
