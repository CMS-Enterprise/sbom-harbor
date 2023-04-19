mod service;
pub use service::*;

use std::fmt::Debug;

use crate::entities::sboms::{Finding, Sbom};
use crate::Error;
use async_trait::async_trait;
use flatten_json_object::ArrayFormatting;
use flatten_json_object::Flattener;

/// Service that is capable of creating and storing one or more SBOMs.
#[async_trait]
pub trait FindingProvider {
    /// Sync an external Findings source with Harbor.
    async fn sync(&self) -> Result<(), Error>;
    // TODO
    // async fn sync_one<T>(&self, opts: T) -> Result<(), Error>;
}

#[async_trait]
pub trait StorageProvider: Debug + Send + Sync {
    async fn write(&self, purl: &str, findings: &Vec<Finding>) -> Result<(), Error>;
}

/// Saves SBOMs to the local filesystem.
#[derive(Debug)]
pub struct FileSystemStorageProvider {
    out_dir: String,
}

impl FileSystemStorageProvider {
    pub fn new(out_dir: Option<String>) -> FileSystemStorageProvider {
        let out_dir = match out_dir {
            None => "/tmp/harbor/findings".to_string(),
            Some(out_dir) => out_dir,
        };

        FileSystemStorageProvider { out_dir }
    }
}

#[async_trait]
impl StorageProvider for FileSystemStorageProvider {
    async fn write(&self, purl: &str, findings: &Vec<Finding>) -> Result<(), Error> {
        let purl = purl.replace("/", "_");

        match std::fs::create_dir_all(&self.out_dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "write::create_dir_all::{}",
                    e.to_string()
                )));
            }
        }

        let file_name = format!("findings-{}", purl);
        let file_path = format!("{}/{}", self.out_dir, file_name);

        let json_raw = serde_json::to_string(findings)
            .map_err(|e| Error::Serde(format!("write::to_string::{}", e)))?;

        match std::fs::write(file_path.as_str(), json_raw) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e.to_string())));
            }
        }

        // let checksum = platform::cryptography::sha256::file_checksum(file_path)?;
        // let checksum = platform::encoding::base64::standard_encode(checksum.as_str());
        // sbom.checksum_sha256 = checksum;

        // Now flatten it
        let file_name = format!("findings-flat-{}", purl);
        let file_path = format!("{}/{}", self.out_dir, file_name);

        let json_value = serde_json::to_value(findings)
            .map_err(|e| Error::Serde(format!("write::to_value::{}", e)))?;

        let flattened = Flattener::new()
            .set_key_separator(".")
            .set_array_formatting(ArrayFormatting::Surrounded {
                start: "[".to_string(),
                end: "]".to_string(),
            })
            .set_preserve_empty_arrays(false)
            .set_preserve_empty_objects(false)
            .flatten(&json_value)
            .map_err(|e| Error::Runtime(format!("write::flatten::{}", e.to_string())))?;

        let flattened = serde_json::to_string(&flattened)
            .map_err(|e| Error::Serde(format!("write::to_string::{}", e)))?;

        match std::fs::write(file_path.as_str(), flattened) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "write:flat_write::{}",
                    e.to_string()
                )));
            }
        }

        Ok(())
    }
}
