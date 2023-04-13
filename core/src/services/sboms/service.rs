use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;

use crate::entities::sboms::Sbom;
use async_std::fs::OpenOptions;
use async_std::io::WriteExt;
use async_trait::async_trait;
use platform::config::from_env;
use platform::mongodb::Context;
use regex::Regex;
use tracing::debug;

use crate::models::cyclonedx::{Bom, Component, Metadata};
use crate::models::sboms::{CycloneDxFormat, Sbom};
use crate::Error;

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
pub struct SbomService {
    cx: Context,
    config: SdkConfig,
}

impl SbomService {
    /// Factory method for creating new instance of type.
    pub fn new(cx: Context, config: SdkConfig) -> Self {
        Self { cx, config }
    }

    /// Saves the SBOM to the configured persistence providers.
    pub async fn upload_raw(purl: &str, raw: &str) -> Result<Sbom, Error> {}
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
