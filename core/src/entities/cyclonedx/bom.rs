use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::{Purl, SourceKind};
use crate::entities::sboms::CdxFormat;
use crate::models::sbom::CycloneDxFormat;
use crate::services::cyclonedx::models::{Bom, Component};
use crate::Error;
use tracing::debug;

impl Bom {
    /// Compares Bom instances for equality.
    pub fn eq(&self, _other: &mut Bom) -> Result<bool, Error> {
        // This function has to remove known variable values before comparing, for example timestamps.
        // first.metadata?.timestamp = None;
        // second.metadata?.timestamp = None;
        //
        // first_raw = serde_json::to_string(first);
        // second_raw = serde_json::to_string(second);

        // TODO
        Ok(false)
    }

    /// Parses a raw string into a CycloneDx Bom instance.
    pub fn parse(raw: &str, format: CdxFormat) -> Result<Bom, Error> {
        match format {
            CdxFormat::Json => {
                let bom = serde_json::from_str::<Bom>(raw).map_err(|e| {
                    Error::Serde(format!("error serializing CycloneDx SBOM - {}", e))
                })?;

                Ok(bom)
            }
            CdxFormat::Xml => Err(Error::Runtime("CycloneDx XML not supported".to_string())),
        }
    }

    /// Extracts the Component for the Bom.
    pub fn component(&self) -> Option<Component> {
        self.metadata.clone()?.component
    }

    /// Extracts the Purl for the Bom.
    pub fn purl(&self) -> Option<String> {
        bom.metadata.clone()?.component?.purl
    }

    /// Extract the purls from the Components and Dependencies in a CycloneDx SBOM.
    pub fn extract_purls(&self, snyk_ref: SnykXRef) -> Option<Vec<Purl>> {
        let mut purls = Vec::<Purl>::new();

        let components: &Vec<Component> = match &self.components {
            None => &vec![],
            Some(c) => c,
        };

        components.iter().for_each(|c| {
            match Purl::from_snyk(c, SourceKind::Dependency, snyk_ref) {
                Ok(purl) => match purl {
                    None => {}
                    Some(purl) => purls.push(purl),
                },
                Err(e) => debug!("bom::extract_purls::{}", e),
            }
        });

        if purls.is_empty() {
            return None;
        }

        Some(purls)
    }
}
