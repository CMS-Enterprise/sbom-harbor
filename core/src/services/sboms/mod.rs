mod service;

use serde_json::json;

pub use service::*;
use std::fmt::Debug;
use std::io::BufReader;

use crate::{config, Error};

use crate::entities::packages::Purl;
use crate::entities::sboms::Sbom;
use crate::entities::xrefs;
use async_trait::async_trait;
use platform::persistence::s3;

/// Abstract storage provider for [Sboms].
#[async_trait]
pub trait StorageProvider<'a>: Debug + Send + Sync {
    /// Write Sbom to storage provider and return output path.
    async fn write(&self, raw: &str, sbom: &mut Sbom) -> Result<String, Error>;
}

/// Saves SBOMs to the local filesystem.
#[derive(Debug)]
pub struct FileSystemStorageProvider {
    out_dir: String,
}

impl FileSystemStorageProvider {
    pub fn new(out_dir: Option<String>) -> FileSystemStorageProvider {
        let out_dir = match out_dir {
            None => "/tmp/harbor/sboms".to_string(),
            Some(out_dir) => out_dir,
        };

        FileSystemStorageProvider { out_dir }
    }
}

#[async_trait]
impl StorageProvider<'_> for FileSystemStorageProvider {
    async fn write(&self, raw: &str, sbom: &mut Sbom) -> Result<String, Error> {
        let purl = &sbom.purl()?;
        let purl = Purl::format_file_name(purl.as_str());

        match std::fs::create_dir_all(&self.out_dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e.to_string())));
            }
        }

        // TODO: This area likely needs to be dynamically invoked when Quinn handles storage.
        let file_name = format!("sbom-{}-{}", purl, sbom.instance);
        let file_path = format!("{}/{}", self.out_dir, file_name);
        match std::fs::write(file_path.as_str(), raw) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e.to_string())));
            }
        }

        let checksum = platform::cryptography::sha256::file_checksum(file_path)?;
        let checksum = platform::encoding::base64::standard_encode(checksum.as_str());
        sbom.checksum_sha256 = checksum;

        Ok(file_name)
    }
}

fn debug_sbom(out_dir: &str, sbom: &Sbom) -> Result<(), Error> {
    let file_name = "debug.json".to_string();
    let file_path = format!("{}/{}", out_dir, file_name);

    let json_raw = match serde_json::to_string(sbom) {
        Ok(j) => j,
        Err(e) => {
            let msg = format!("debug_sbom::{}", e.to_string());
            return Err(Error::Runtime(msg));
        }
    };

    match std::fs::write(file_path.as_str(), json_raw) {
        Ok(()) => Ok(()),
        Err(e) => {
            return Err(Error::Runtime(format!("debug_sbom::{}", e.to_string())));
        }
    }
}

/// Save SBOMs to an S3 bucket.
#[derive(Debug)]
pub struct S3StorageProvider {}

#[async_trait]
impl StorageProvider<'_> for S3StorageProvider {
    async fn write(&self, raw: &str, sbom: &mut Sbom) -> Result<String, Error> {
        let purl = &sbom.purl()?;

        let metadata = match &sbom.xrefs {
            None => None,
            Some(xrefs) => Some(xrefs::flatten(xrefs.clone())),
        };

        // TODO: Probably want to inject these values.
        let s3_store = s3::Store::new_from_env().await?;
        let bucket_name = config::sbom_upload_bucket()?;
        let object_key = format!("sbom-{}-{}", purl, sbom.instance);

        let mut reader = BufReader::new(raw.clone().as_bytes());

        let checksum = platform::cryptography::sha256::reader_checksum(reader)?;
        let checksum = platform::encoding::base64::standard_encode(checksum.as_str());

        sbom.checksum_sha256 = checksum.clone();

        s3_store
            .insert(
                bucket_name,
                object_key.clone(),
                Some(checksum),
                raw.as_bytes().to_vec(),
                metadata,
            )
            .await?;

        Ok(object_key)
    }
}
