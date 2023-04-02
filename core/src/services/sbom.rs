use crate::Error;
use crate::models::cyclonedx::{Bom, Component};
use crate::models::sboms::CycloneDxFormat;

/// Provides SBOM related capabilities.
pub struct SbomService {}

impl SbomService {
    pub fn parse_cyclonedx_bom(raw: String, format: CycloneDxFormat) -> Result<Bom, Error> {
        match format {
            CycloneDxFormat::Json => {
                let bom = serde_json::from_str::<Bom>(&raw)
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