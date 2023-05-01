mod service;
pub mod snyk;

pub use service::*;

use async_trait::async_trait;
use std::fmt::Debug;
use tracing::log::debug;

use crate::entities::packages::{Finding, FindingProviderKind, Purl};
use crate::entities::scans::{Scan, ScanKind, ScanRef, ScanStatus};
use crate::services::scans::ScanProvider;
use crate::Error;

// TODO: This could be generalized and combined with Sbom version.
/// Abstract storage provider for findings.
#[async_trait]
pub trait StorageProvider: Debug + Send + Sync {
    /// Write findings to storage provider and return output path.
    async fn write(
        &self,
        purl: &str,
        findings: &Vec<Finding>,
        scan_ref: &ScanRef,
    ) -> Result<String, Error>;
}

/// Saves findings results to the local filesystem.
#[derive(Clone, Debug)]
pub struct FileSystemStorageProvider {
    out_dir: String,
}

impl FileSystemStorageProvider {
    pub fn new(out_dir: String) -> FileSystemStorageProvider {
        let out_dir = match out_dir.is_empty() {
            true => "/tmp/harbor/findings".to_string(),
            false => out_dir,
        };

        FileSystemStorageProvider { out_dir }
    }
}

#[async_trait]
impl StorageProvider for FileSystemStorageProvider {
    async fn write(
        &self,
        purl: &str,
        findings: &Vec<Finding>,
        scan_ref: &ScanRef,
    ) -> Result<String, Error> {
        match std::fs::create_dir_all(&self.out_dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "write::create_dir_all::{}",
                    e.to_string()
                )));
            }
        }

        let file_name = format!(
            "findings-{}-{}",
            Purl::format_file_name(purl),
            scan_ref.iteration
        );
        let file_path = format!("{}/{}", self.out_dir, file_name);

        let json_raw = serde_json::to_string(findings)
            .map_err(|e| Error::Serde(format!("write::to_string::{}", e)))?;

        match std::fs::write(file_path.as_str(), json_raw) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e.to_string())));
            }
        }

        // TODO: Add checksum to findings files and relate Sboms to Findings.
        // let checksum = platform::cryptography::sha256::file_checksum(file_path)?;
        // let checksum = platform::encoding::base64::standard_encode(checksum.as_str());
        // sbom.checksum_sha256 = checksum;

        Ok(file_name)
    }
}
