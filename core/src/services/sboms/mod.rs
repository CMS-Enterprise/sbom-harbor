mod service;
/// Supports generating [Sbom] instances from the Snyk API.
pub mod snyk;

pub use service::*;
use std::fmt::Debug;
use std::io::BufReader;

use crate::{config, Error};

use crate::entities::sboms::Sbom;
use crate::entities::xrefs;
use crate::entities::xrefs::Xref;
use async_trait::async_trait;
use regex::Regex;
use platform::persistence::s3;

fn make_safe(purl: &str) -> Result<String, Error> {
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    Ok(re.replace_all(purl, "-").to_string())
}

/// Abstract storage provider for [Sboms].
#[async_trait]
pub trait StorageProvider: Debug + Send + Sync {
    /// Write Sbom to storage provider and return output path.
    async fn write(
        &self,
        raw: Vec<u8>,
        sbom: &mut Sbom,
        xref: &Option<Xref>,
    ) -> Result<String, Error>;
}

/// Saves SBOMs to the local filesystem.
#[derive(Debug)]
pub struct FileSystemStorageProvider {
    out_dir: String,
}

impl FileSystemStorageProvider {
    /// Factory method to create new instances of type.
    pub fn new(out_dir: String) -> FileSystemStorageProvider {
        let out_dir = match out_dir.is_empty() {
            true => "/tmp/harbor/sboms".to_string(),
            false => out_dir,
        };
        FileSystemStorageProvider { out_dir }
    }
}

#[async_trait]
impl StorageProvider for FileSystemStorageProvider {
    async fn write(
        &self,
        raw: Vec<u8>,
        sbom: &mut Sbom,
        _xref: &Option<Xref>,
    ) -> Result<String, Error> {

        let purl = make_safe(&sbom.purl()?)?;

        match std::fs::create_dir_all(&self.out_dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e)));
            }
        }

        // TODO: This area likely needs to be dynamically invoked when Quinn handles storage.
        let file_name = format!("sbom-{}-{}", purl, sbom.instance);
        let file_path = format!("{}/{}", self.out_dir, file_name);
        match std::fs::write(file_path.as_str(), raw) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(format!("write::{}", e)));
            }
        }

        let checksum = platform::cryptography::sha256::file_checksum_sha256(file_path)?;
        let checksum = platform::encoding::base64::standard_encode(checksum.as_str());
        sbom.checksum_sha256 = checksum;

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
        raw: Vec<u8>,
        sbom: &mut Sbom,
        xref: &Option<Xref>,
    ) -> Result<String, Error> {

        let purl = make_safe(&sbom.purl()?)?;

        let metadata = xref.as_ref().map(xrefs::flatten);

        // TODO: Probably want to inject these values.
        let s3_store = s3::Store::new_from_env().await?;
        let bucket_name = config::harbor_bucket()?;
        let object_key = format!("sbom-{}-{}", purl, sbom.instance);

        let reader = BufReader::new(raw.as_slice());

        let checksum = platform::cryptography::sha256::reader_checksum_sha256(reader)?;
        let checksum = platform::encoding::base64::standard_encode(checksum.as_str());

        sbom.checksum_sha256 = checksum.clone();

        s3_store
            .put(
                bucket_name,
                object_key.clone(),
                Some(checksum),
                raw,
                metadata,
            )
            .await?;

        Ok(object_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::sboms::Sbom;
    use crate::Error;

    #[allow(dead_code)]
    #[ignore = "debug"]
    fn debug_sbom(out_dir: &str, sbom: &Sbom) -> Result<(), Error> {
        let file_name = "debug.json".to_string();
        let file_path = format!("{}/{}", out_dir, file_name);

        let json_raw = match serde_json::to_string(sbom) {
            Ok(j) => j,
            Err(e) => {
                let msg = format!("debug_sbom::{}", e);
                return Err(Error::Runtime(msg));
            }
        };

        match std::fs::write(file_path.as_str(), json_raw) {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::Runtime(format!("debug_sbom::{}", e))),
        }
    }
}
