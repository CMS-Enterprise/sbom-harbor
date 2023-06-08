mod service;

/// Supports generating [Vulnerability] instances from the Snyk API.
pub mod snyk;

/// Supports adding an EPSS Score to a [Vulnerability].
pub mod epss;

pub use service::*;
use std::collections::HashMap;

use async_trait::async_trait;
use platform::persistence::s3;
use std::fmt::Debug;
use std::io::BufReader;

use crate::entities::enrichments::Vulnerability;
use crate::entities::packages::Package;
use crate::entities::xrefs;
use crate::entities::xrefs::Xref;
use crate::{config, Error};

// TODO: This could maybe be generalized and combined with Sbom version.
/// Abstract storage provider for vulnerabilities.
#[async_trait]
pub trait StorageProvider: Debug + Send + Sync {
    /// Write vulnerabilities to storage provider and return output path.
    async fn write(
        &self,
        purl: &str,
        vulnerabilities: &[Vulnerability],
        xrefs: &[Xref],
    ) -> Result<String, Error>;
}

/// Saves vulnerability results to the local filesystem.
#[derive(Clone, Debug)]
pub struct FileSystemStorageProvider {
    out_dir: String,
}

impl FileSystemStorageProvider {
    /// Factory method for creating new instances of type.
    pub fn new(out_dir: String) -> FileSystemStorageProvider {
        let out_dir = match out_dir.is_empty() {
            true => "/tmp/harbor/vulnerabilities".to_string(),
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
        vulnerabilities: &[Vulnerability],
        _xrefs: &[Xref],
    ) -> Result<String, Error> {
        if vulnerabilities.is_empty() {
            return Err(Error::Vulnerability("vulnerabilities_empty".to_string()));
        }

        let provider = vulnerabilities[0].provider.clone();

        match std::fs::create_dir_all(&self.out_dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::create_dir_all::{}", e)));
            }
        }

        let file_name = format!(
            "vulnerabilities-{}-{}",
            provider,
            Package::format_file_name(purl)
        );
        let file_path = format!("{}/{}", self.out_dir, file_name);

        let json_raw = serde_json::to_string(vulnerabilities)
            .map_err(|e| Error::Serde(format!("write::to_string::{}", e)))?;

        match std::fs::write(file_path.as_str(), json_raw) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e)));
            }
        }

        // TODO: Add checksum to vulnerabilities files and relate Sboms to Findings.
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
        vulnerabilities: &[Vulnerability],
        xrefs: &[Xref],
    ) -> Result<String, Error> {
        if vulnerabilities.is_empty() {
            return Err(Error::Vulnerability("vulnerabilities_empty".to_string()));
        }

        let provider = vulnerabilities[0].provider.clone();

        let mut metadata = HashMap::<String, String>::new();

        for xref in xrefs {
            metadata.extend(xrefs::flatten(xref));
        }

        // TODO: Probably want to inject these values.
        let s3_store = s3::Store::new_from_env().await?;
        let bucket_name = config::harbor_bucket()?;
        let object_key = format!(
            "vulnerabilities-{}-{}",
            provider,
            Package::format_file_name(purl)
        );

        let json_raw = serde_json::to_vec(vulnerabilities)
            .map_err(|e| Error::Serde(format!("write::to_string::{}", e)))?;

        // TODO: Add checksum to vulnerabilities files and relate Sboms to Vulnerabilities.
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
