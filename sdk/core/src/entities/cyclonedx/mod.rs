#[allow(missing_docs)]
/// Data structures generated from the CycloneDx JSON Spec.
pub(crate) mod models;

use crate::entities::cyclonedx::bom::BomFormat;
use crate::entities::sboms::CdxFormat;
use crate::services::syft::try_extract_package_manager;
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

    /// Extracts the Component Name for the BOM if available.
    pub fn component_name(&self) -> Option<String> {
        match self.component() {
            None => None,
            Some(component) => Some(component.name),
        }
    }

    /// Extracts the Component Version for the BOM if available.
    pub fn component_version(&self) -> Option<String> {
        if let Some(version) = self.component()?.version {
            return Some(version);
        }

        None
    }

    /// Extracts the Supplier Name for the BOM if available.
    pub fn supplier_name(&self) -> Option<String> {
        if let Some(name) = &self.component()?.supplier.as_deref()?.name {
            return Some(name.clone());
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

    /// Best effort algorithm to build a valid Purl for the Sbom by extracting values from the
    /// package metadata.
    pub fn try_build_purl_from_metadata(
        &self,
        default_name: Option<String>,
        default_version: Option<String>,
    ) -> Option<String> {
        // Try to get the name from the Sbom directly.
        let component_name = match &self.metadata.clone()?.component {
            None => "".to_string(),
            Some(component) => component.name.clone(),
        };

        // If no component name, use name if passed. If name not resolvable, exit.
        let name = match component_name.is_empty() {
            false => component_name,
            true => match default_name {
                None => {
                    return None;
                }
                Some(n) => n,
            },
        };

        // set a default version if unable to resolve.
        let version = match self.metadata.clone()?.component?.version {
            None => match default_version {
                None => "not-set".to_string(),
                Some(c) => {
                    if c.is_empty() {
                        "not-set".to_string();
                    }
                    c
                }
            },
            Some(v) => v,
        };

        let package_manager = try_extract_package_manager(self);
        Some(format!("{}/{}@{}", package_manager, name, version))
    }
}
