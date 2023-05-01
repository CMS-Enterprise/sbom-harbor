use crate::entities::scans::{Scan, ScanKind, ScanStatus};
use crate::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use platform::mongodb::Service;
use tracing::log::debug;

// TODO: Review with the team if Batch is a better model concept than Scan.

#[async_trait]
pub trait ScanProvider: Service<Scan> {
    fn current(&self) -> Arc<Mutex<Scan>>;

    // TODO: Abstract this further and inject scan specific behaviors.
    async fn scan(&mut self) -> Result<(), Error> {
        match self.load_targets().await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("scan_load_targets_failed::{}", e);
                println!("{}", msg);
                return Err(Error::Scan(msg));
            }
        }

        self.set_count();

        let result = self.scan_targets().await;

        match result {
            Ok(results) => {
                for (key, value) in results {
                    self.error(key.as_str(), value.as_str());
                }
            }
            Err(e) => {
                let msg = format!("scan_failed::{}", e);
                println!("{}", msg);

                let mut scan = self.current().lock().unwrap();
                scan.err = Some(msg.clone());
                drop(scan);
            }
        };

        let result = self.commit_scan().await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => return Err(Error::Scan(e.to_string())),
        }
    }

    async fn scan_targets(&self) -> Result<HashMap<String, String>, Error>;

    async fn load_targets(&mut self) -> Result<(), Error>;

    fn target_count(&self) -> u64;

    fn set_count(&mut self) {
        let mut scan = self.current().lock().unwrap();
        scan.count = self.target_count();
    }

    async fn init_scan(&mut self, kind: ScanKind, count: Option<u64>) -> Result<(), Error> {
        let mut scan = self.current().lock().unwrap().clone();

        match self.insert(&mut scan).await {
            Ok(scan) => {}
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

        *self.current().lock().unwrap() = scan;

        Ok(())
    }

    async fn commit_scan(&mut self) -> Result<(), Error> {
        let mut complete = self.current().lock().unwrap().clone();

        match complete.err {
            None => {
                complete.err_total = complete
                    .ref_errs
                    .iter()
                    .filter(|ref_err| !ref_err.is_empty())
                    .count() as u64;

                match complete.err_total > 0 {
                    true => complete.status = ScanStatus::CompleteWithErrors,
                    false => {
                        complete.status = ScanStatus::Complete;
                    }
                }
            }
            Some(_) => {
                complete.status = ScanStatus::Failed;
            }
        }

        complete.finish = Utc::now();
        complete.duration_seconds = complete
            .finish
            .signed_duration_since(complete.start)
            .num_seconds();

        match self.update(&mut complete).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!("commit_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Scan(msg));
            }
        }
    }

    fn error(&mut self, identifier: &str, error: &str) {
        println!("==> error processing {}: {}", identifier, error);
        let mut scan = self.current().lock().unwrap();

        scan.ref_errs(identifier.to_string(), error.to_string());
    }
}
