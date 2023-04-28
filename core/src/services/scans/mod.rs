use crate::entities::scans::{Scan, ScanRef, ScanStatus};
use crate::Error;

use async_trait::async_trait;
use chrono::Utc;
use platform::mongodb::Service;
use tracing::log::debug;

// TODO: Review with the team if Batch is a better model concept than Scan.

#[async_trait]
pub trait ScanProvider<'a>: Service<Scan> {
    // TODO: Abstract this further and inject scan specific behaviors.
    // async fn scan(&self, scan: &mut Scan) -> Result<(), Error>;
    //
    // fn scan_kind(&self) -> ScanKind;
    //
    // fn targets(&self) -> HashMap<String, T>;
    //
    // async fn load_targets(&self) -> Result<(), Error>;
    //
    // fn target_count(&self) -> u32;
    //
    // fn process_target(&self, scan_target: T) -> Result<(), Error>;

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
                let msg = format!("commit_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Scan(msg));
            }
        }
    }
}
