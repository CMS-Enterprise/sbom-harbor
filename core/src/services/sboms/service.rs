use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
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
use crate::services::findings::FindingService;
use crate::services::sboms::StorageProvider;
use crate::services::xrefs::XrefService;
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

// Implement Xref Service so that xrefs can be managed for Sboms.
impl XrefService<Sbom> for SbomService {}

/// Provides SBOM related capabilities.
#[derive(Debug)]
pub struct SbomService {
    cx: Context,
    storage: Box<dyn for<'a> StorageProvider<'a>>,
}

impl Service<Sbom> for SbomService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl SbomService {
    /// Factory method for creating new instance of type.
    pub fn new(cx: Context, storage: Box<dyn for<'a> StorageProvider<'a>>) -> Self {
        Self { cx, storage }
    }

    /// Stores the SBOM to the configured persistence provider using the Purl as the unique
    /// identifier.
    pub async fn write_to_storage(
        &self,
        raw: Vec<u8>,
        sbom: &mut Sbom,
        xref: Option<Xref>,
    ) -> Result<(), Error> {
        // Persist to some sort of permanent storage.
        self.storage.write(raw, sbom, &xref).await?;

        Ok(())
    }

    /// Sets the forward only instance counter using the Purl as the unique identifier.
    pub async fn set_instance_by_purl(&self, sbom: &mut Sbom) -> Result<(), Error> {
        // TODO: As more enrichment sources are added this may need to constrain by Xref too.
        let existing = self.find_by_purl(&sbom.purl).await?;

        sbom.instance = match existing.iter().max_by_key(|s| s.instance) {
            None => 1,
            Some(most_recent) => most_recent.instance + 1,
        };

        Ok(())
    }

    /// Find an [Sbom] by it's Package URL.
    pub async fn find_by_purl(&self, purl: &Option<String>) -> Result<Vec<Sbom>, Error> {
        match purl {
            None => {
                return Err(Error::Entity("sbom_purl_none".to_string()));
            }
            Some(purl) => self
                .query(HashMap::from([("purl", purl.as_str())]))
                .await
                .map_err(|e| Error::Entity(e.to_string())),
        }
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
