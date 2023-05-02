mod service;

/// Supports generating [Finding] instances from the Snyk API.
pub mod snyk;

pub use service::*;
use std::collections::HashMap;

use async_trait::async_trait;
use platform::persistence::s3;
use std::fmt::Debug;
use std::io::BufReader;
use tracing::log::debug;

use crate::entities::packages::{Finding, FindingProviderKind, Purl};
use crate::entities::scans::{Scan, ScanKind, ScanRef, ScanStatus};
use crate::entities::xrefs;
use crate::entities::xrefs::Xref;
use crate::services::scans::ScanProvider;
use crate::{config, Error};

/// Service that is capable of synchronizing findings from a dynamic source.
#[async_trait]
pub trait FindingProvider<'a>: ScanProvider<'a> {
    /// Perform the [Scan] operation.
    async fn execute(&self, provider: FindingProviderKind) -> Result<(), Error> {
        let mut scan = match self.init_scan(provider, None).await {
            Ok(scan) => scan,
            Err(e) => {
                return Err(Error::Finding(format!("finding::init_scan::{}", e)));
            }
        };

        match self.scan(&mut scan).await {
            Ok(_) => {}
            Err(e) => {
                scan.err = Some(e.to_string());
            }
        }

        self.commit_scan(&mut scan).await
    }

    /// The dynamic implementation unique to each type that implements this trait.
    async fn scan(&self, scan: &mut Scan) -> Result<(), Error>;

    /// Initializes a new [Scan] entity and writes it to the data store.
    async fn init_scan(
        &self,
        provider: FindingProviderKind,
        count: Option<u64>,
    ) -> Result<Scan, Error> {
        let mut scan = match Scan::new(ScanKind::Finding(provider), ScanStatus::Started, count) {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("init_scan::new_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Finding(msg));
            }
        };

        match self.insert(&mut scan).await {
            Ok(_) => Ok(scan),
            Err(e) => {
                let msg = format!("init_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Finding(msg));
            }
        }
    }
}
// TODO: This could maybe be generalized and combined with Sbom version.
/// Abstract storage provider for findings.
#[async_trait]
pub trait StorageProvider: Debug + Send + Sync {
    /// Write findings to storage provider and return output path.
    async fn write(
        &self,
        purl: &str,
        findings: &[Finding],
        scan_ref: &ScanRef,
        xrefs: &[Xref],
    ) -> Result<String, Error>;
}

/// Saves findings results to the local filesystem.
#[derive(Clone, Debug)]
pub struct FileSystemStorageProvider {
    out_dir: String,
}

impl FileSystemStorageProvider {
    /// Factory method for creating new instances of type.
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
        findings: &[Finding],
        scan_ref: &ScanRef,
        _xrefs: &[Xref],
    ) -> Result<String, Error> {
        match std::fs::create_dir_all(&self.out_dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::create_dir_all::{}", e)));
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
                return Err(Error::Runtime(format!("write::{}", e)));
            }
        }

        // TODO: Add checksum to findings files and relate Sboms to Findings.
        // let checksum = platform::cryptography::sha256::file_checksum(file_path)?;
        // let checksum = platform::encoding::base64::standard_encode(checksum.as_str());
        // sbom.checksum_sha256 = checksum;

        Ok(file_name)
    }
}

/// Save SBOMs to an S3 bucket.
#[derive(Debug)]
pub struct S3StorageProvider {}

#[async_trait]
impl StorageProvider for S3StorageProvider {
    async fn write(
        &self,
        purl: &str,
        findings: &[Finding],
        scan_ref: &ScanRef,
        xrefs: &[Xref],
    ) -> Result<String, Error> {
        let mut metadata = HashMap::<String, String>::new();

        for xref in xrefs {
            metadata.extend(xrefs::flatten(xref));
        }

        // TODO: Probably want to inject these values.
        let s3_store = s3::Store::new_from_env().await?;
        let bucket_name = config::harbor_bucket()?;
        let object_key = format!(
            "findings-{}-{}",
            Purl::format_file_name(purl),
            scan_ref.iteration
        );

        let json_raw = serde_json::to_vec(findings)
            .map_err(|e| Error::Serde(format!("write::to_string::{}", e)))?;

        // TODO: Add checksum to findings files and relate Sboms to Findings.
        let reader = BufReader::new(json_raw.as_slice());

        let checksum = platform::cryptography::sha256::reader_checksum_sha256(reader)?;
        let checksum = platform::encoding::base64::standard_encode(checksum.as_str());

        s3_store
            .put(
                bucket_name,
                object_key.clone(),
                Some(checksum),
                json_raw,
                Some(metadata),
            )
            .await?;

        Ok(object_key)
    }
}
