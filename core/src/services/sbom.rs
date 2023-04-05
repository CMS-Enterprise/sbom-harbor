use std::fs::File;
use std::io::Write;
use std::path::Path;
use async_std::fs::OpenOptions;

use async_std::io::WriteExt;
use regex::Regex;
use tracing::debug;
use platform::config::from_env;

use crate::Error;
use crate::models::cyclonedx::{Bom, Component};
use crate::models::sboms::CycloneDxFormat;

// TODO: Move this to the SnykService so that these can become &str and avoid a highly inefficient String.clone().
pub async fn sync_debug(sbom: String, identifier: String) -> Result<(), Error> {
    let sanitizer = regex::Regex::new(r#"([^\p{L}\s\d_~,;:\[\]\(\).'])"#).unwrap();
    let sanitized_identifier = sanitizer.replace_all(identifier.as_str(), "_");

    let debug_dir = from_env("DEBUG_DIR").unwrap();
    let sbom_dir = Path::new(&debug_dir).join("sboms");
    let sbom_path = sbom_dir.join(format!("{}.json", sanitized_identifier));

    match std::fs::create_dir_all(sbom_dir.as_path()) {
        Ok(_) => {}
        Err(e) => {
            let msg = format!("error creating debug directory: {}", e);
            debug!(msg);
            return Err(Error::Runtime(msg.to_string()));
        }
    }

    let mut file = match File::create(sbom_path.as_path()) {
        Ok(f) => f,
        Err(e) => {
            let msg = format!("error opening debug file: {}", e);
            debug!(msg);
            return Err(Error::Runtime(msg.to_string()));
        }
    };

    match file.write_all(sbom.as_ref()) {
        Ok(_) => Ok(()),
        Err(e) => {
            let msg = format!("error writing debug file: {}", e);
            debug!(msg);
            return Err(Error::Runtime(msg.to_string()));
        }
    }
}

/// Provides SBOM related capabilities.
pub struct SbomService {}

impl SbomService {
    pub async fn sync_v1(&self, _sbom: String, _identifier: String) -> Result<(), Error> {
        todo!()
    }

    pub async fn sync_v2(&self, _sbom: String, _identifier: String) -> Result<(), Error> {
        todo!()
    }

    pub fn parse_cyclonedx_bom(raw: &str, format: CycloneDxFormat) -> Result<Bom, Error> {
        match format {
            CycloneDxFormat::Json => {
                let bom = serde_json::from_str::<Bom>(raw)
                    .map_err(|e| Error::Serde(format!("error serializing CycloneDx SBOM - {}", e)))?;

                Ok(bom)
            }
            CycloneDxFormat::Xml => Err(Error::Runtime("CycloneDx XML not supported".to_string()))
        }
    }

    /// Extract the purls from the Components in a CycloneDx SBOM.
    pub fn extract_purls(bom: &Bom) -> Option<Vec<String>> {

        let components = match &bom.components {
            None => {
                return None;
            },
            Some(c) => {
                if c.is_empty() {
                    return None;
                }
                c
            },
        };

        let mut result = Vec::<String>::new();

        components
            .iter()
            .for_each(|c| {
                match &c.purl {
                    None => {}
                    Some(purl) => if !purl.is_empty() {result.push(purl.to_string())}
                }
            });

        if result.is_empty() {
            return None;
        }

        Some(result)
    }
}

/// Invoke [sbom-scorecard](https://github.com/eBay/sbom-scorecard) and return the results.
pub fn score(_path: &str) -> Result<String, Error> {

    Ok("not implemented".to_string())
}

pub fn compare(first_path: &str, second_path: &str) -> Result<String, Error> {
    let first_score = score(first_path)?;
    let second_score = score(second_path)?;

    let mut result = format!("----------------{} score-------------------", first_path);
    result.push_str(first_score.as_str());
    result.push_str(format!("----------------{} score-------------------", second_path).as_str());
    result.push_str(second_score.as_str());

    Ok(result)
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