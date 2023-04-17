use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;

use async_std::fs::OpenOptions;
use async_std::io::WriteExt;
use async_trait::async_trait;
use platform::config::from_env;
use platform::mongodb::{Context, Service};
use platform::persistence::s3;
use regex::Regex;
use tracing::debug;

use crate::entities::cyclonedx::{Bom, Component, Metadata};
use crate::entities::sboms::{CdxFormat, Sbom};
use crate::entities::xrefs;
use crate::entities::xrefs::{Xref, XrefKind};
use crate::{config, Error};

/// Invoke [sbom-scorecard](https://github.com/eBay/sbom-scorecard) and return the results.
pub fn score(_path: &str) -> Result<String, Error> {
    Ok("not implemented".to_string())
}

/// Compare 2 SBOM scores.
pub fn compare(first_path: &str, second_path: &str) -> Result<String, Error> {
    let first_score = score(first_path)?;
    let second_score = score(second_path)?;

    let mut result = format!("----------------{} score-------------------", first_path);
    result.push_str(first_score.as_str());
    result.push_str(format!("----------------{} score-------------------", second_path).as_str());
    result.push_str(second_score.as_str());

    Ok(result)
}

/// Provides SBOM related capabilities.
#[derive(Debug)]
pub struct SbomService {
    cx: Context,
}

impl Service<Sbom> for SbomService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl SbomService {
    /// Factory method for creating new instance of type.
    pub fn new(cx: Context) -> Self {
        Self { cx }
    }

    /// Saves the SBOM to the configured persistence providers using the Purl as the unique
    /// identifier.
    pub async fn insert_by_purl(&self, raw: &str, sbom: &mut Sbom) -> Result<(), Error> {
        let purl = sbom.purl()?;

        sbom.timestamp = platform::time::timestamp()?;

        self.set_instance(sbom, purl).await?;
        self.save_to_s3(raw, sbom).await?;
        self.insert(sbom).await?;

        Ok(())
    }

    async fn set_instance(&self, sbom: &mut Sbom, purl: String) -> Result<(), Error> {
        let existing = self.query(HashMap::from([("purl", purl.as_str())])).await?;

        sbom.instance = match existing.iter().max_by_key(|s| s.instance) {
            None => 1,
            Some(most_recent) => most_recent.instance + 1,
        };

        Ok(())
    }

    pub async fn save_to_s3(&self, raw: &str, sbom: &mut Sbom) -> Result<(), Error> {
        let purl = &sbom.purl()?;

        let metadata = match &sbom.xrefs {
            None => None,
            Some(xrefs) => Some(xrefs::flatten(xrefs.clone())),
        };

        // let s3_store = s3::Store::new_from_env().await?;
        // let bucket_name = config::sbom_upload_bucket()?;
        let object_key = format!("sbom-{}-{}", purl, sbom.instance);
        match fs::write(
            format!("/Users/derek/code/scratch/debug/fake/{}", object_key),
            raw,
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Runtime(e.to_string()));
            }
        }

        // sbom.checksum_sha256 = Some(
        //     s3_store
        //         .insert(bucket_name, object_key, raw.as_bytes().to_vec(), metadata)
        //         .await?,
        // );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_compare_sboms() -> Result<(), Error> {
        Ok(())
    }
}
