#[allow(missing_docs)]
/// Data structures generated from the CycloneDx JSON Spec.
pub(crate) mod models;
use crate::entities::cyclonedx::bom::BomFormat;
use crate::entities::sboms::CdxFormat;
use crate::Error;
pub use models::*;

impl Bom {
    /// Factory method to create new instance of type.
    pub fn new(bom_format: BomFormat, spec_version: String, version: i32) -> Bom {
        Bom {
            dollar_schema: None,
            bom_format,
            spec_version,
            serial_number: None,
            version,
            metadata: None,
            components: None,
            services: None,
            external_references: None,
            dependencies: None,
            compositions: None,
            vulnerabilities: None,
            signature: None,
        }
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
        if let Some(component) = self.metadata.as_deref()?.component.as_deref() {
            return Some(component.clone());
        }

        None
    }

    /// Extracts the CPE for the BOM if available.
    pub fn cpe(&self) -> Option<String> {
        if let Some(cpe) = self.component()?.cpe {
            return Some(cpe);
        }

        None
    }

    /// Extracts the Purl for the Bom.
    pub fn purl(&self) -> Option<String> {
        self.metadata.clone()?.component?.purl
    }
}
