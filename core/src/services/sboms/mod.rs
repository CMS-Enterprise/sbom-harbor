mod service;
pub mod snyk;

use serde_json::json;

pub use service::*;
use std::fmt::Debug;
use std::io::BufReader;

use crate::{config, Error};

use crate::entities::packages::Purl;
use crate::entities::sboms::{Sbom, SbomProviderKind};
use crate::entities::scans::{Scan, ScanKind, ScanStatus};
use crate::entities::xrefs;
use crate::entities::xrefs::Xref;
use crate::services::packages::PackageService;
use crate::services::scans::ScanProvider;
use async_trait::async_trait;
use platform::persistence::s3;
use tracing::log::debug;

/// Service that is capable of synchronizing one or more SBOMs from a dynamic source.
#[async_trait]
pub trait SbomProvider<'a>: ScanProvider<'a> {
    /// Sync an external Sbom source with Harbor.
    async fn sync(&self, provider: SbomProviderKind) -> Result<(), Error> {
        let mut scan = match self.init_scan(provider, None).await {
            Ok(scan) => scan,
            Err(e) => {
                return Err(Error::Sbom(format!("sbom::init_scan::{}", e)));
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

    async fn scan(&self, scan: &mut Scan) -> Result<(), Error>;

    async fn init_scan(
        &self,
        provider: SbomProviderKind,
        count: Option<u64>,
    ) -> Result<Scan, Error> {
        let mut scan = match Scan::new(ScanKind::Sbom(provider), ScanStatus::Started, count) {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("init_scan::new_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        };

        match self.insert(&mut scan).await {
            Ok(_) => Ok(scan),
            Err(e) => {
                let msg = format!("init_scan::store_failed::{}", e);
                debug!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        }
    }
}

/// Abstract storage provider for [Sboms].
#[async_trait]
pub trait StorageProvider<'a>: Debug + Send + Sync {
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
    pub fn new(out_dir: String) -> FileSystemStorageProvider {
        let out_dir = match out_dir.is_empty() {
            true => "/tmp/harbor/sboms".to_string(),
            false => out_dir,
        };
        FileSystemStorageProvider { out_dir }
    }
}

#[async_trait]
impl StorageProvider<'_> for FileSystemStorageProvider {
    async fn write(
        &self,
        raw: Vec<u8>,
        sbom: &mut Sbom,
        _xref: &Option<Xref>,
    ) -> Result<String, Error> {
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
    async fn write(
        &self,
        raw: Vec<u8>,
        sbom: &mut Sbom,
        xref: &Option<Xref>,
    ) -> Result<String, Error> {
        let purl = &sbom.purl()?;

        let metadata = match xref {
            None => None,
            Some(xref) => Some(xrefs::flatten(xref)),
        };

        // TODO: Probably want to inject these values.
        let s3_store = s3::Store::new_from_env().await?;
        let bucket_name = config::sbom_upload_bucket()?;
        let object_key = format!("sbom-{}-{}", purl, sbom.instance);

        let mut reader = BufReader::new(raw.as_slice());

        let checksum = platform::cryptography::sha256::reader_checksum(reader)?;
        let checksum = platform::encoding::base64::standard_encode(checksum.as_str());

        sbom.checksum_sha256 = checksum.clone();

        s3_store
            .insert(
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
