/// Module to support analysis on SBOMs
pub mod sboms;

use std::collections::HashMap;

use crate::{config, Error};
use async_trait::async_trait;
use platform::filesystem::make_file_name_safe;
use platform::persistence::s3;
use platform::persistence::s3::to_safe_object_key;
use serde_json::Value;
use std::fmt::Debug;
use std::io::{BufReader, Cursor};

/// Ensuring the file name is safe
fn get_file_name(purl: &str) -> Result<String, Error> {
    let safe_purl = make_file_name_safe(purl)?;
    Ok(format!("{}.json", safe_purl))
}

/// Ensuring the s3 key is safe
fn get_s3_key_name(purl: &str) -> Result<String, Error> {
    let safe_purl = to_safe_object_key(purl)?;
    Ok(format!("{}.json", safe_purl))
}

// TODO: This could maybe be generalized and combined with Sbom version.
/// Abstract storage provider for vulnerabilities.
#[async_trait]
pub trait StorageProvider: Debug + Send + Sync {
    /// Write vulnerabilities to storage provider and return output path.
    async fn write(&self, purl: &str, json: Value, provider_name: &str) -> Result<String, Error>;
}

/// Saves Analytics results to the local filesystem.
#[derive(Clone, Debug)]
pub struct FileSystemStorageProvider {
    out_dir: String,
}

impl FileSystemStorageProvider {
    /// Factory method for creating new instances of type.
    pub fn new(out_dir: String) -> FileSystemStorageProvider {
        let out_dir = match out_dir.is_empty() {
            true => "/tmp/harbor-debug/analyze/sbom-detail".to_string(),
            false => out_dir,
        };

        FileSystemStorageProvider { out_dir }
    }
}

#[async_trait]
impl StorageProvider for FileSystemStorageProvider {
    async fn write(&self, purl: &str, json: Value, provider_name: &str) -> Result<String, Error> {
        let target_dir = format!("{}/analytic-{}", self.out_dir, provider_name);
        let file_name = get_file_name(purl)?;
        let file_path = format!("{}/{}", target_dir, file_name);
        let json_raw = serde_json::to_string(&json).map_err(Error::Serde)?;

        match std::fs::create_dir_all(target_dir.clone()) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "write::create_dir_all:: {} - {}",
                    target_dir, e
                )));
            }
        }

        match std::fs::write(file_path.as_str(), json_raw) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e)));
            }
        }

        Ok(file_name)
    }
}

/// Save Analytics to an S3 bucket.
#[derive(Clone, Debug)]
pub struct S3StorageProvider {}

#[async_trait]
impl StorageProvider for S3StorageProvider {
    async fn write(&self, purl: &str, json: Value, provider_name: &str) -> Result<String, Error> {
        let metadata = HashMap::<String, String>::new();
        let s3_store = s3::Store::new_from_env().await?;
        let bucket_name = config::harbor_bucket()?;

        let mut object_key = get_s3_key_name(purl)?;
        object_key = format!("analytic-{}/{}", provider_name, object_key);

        let json_raw = serde_json::to_vec(&json).map_err(Error::Serde)?;

        let json_string = json.to_string();
        let cursor = Cursor::new(json_string.into_bytes());
        let reader = BufReader::new(cursor);
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
