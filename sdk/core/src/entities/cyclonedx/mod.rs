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

    /// Extracts the Purl for the Bom. If the Purl is not set in the incoming SBOM, Harbor will
    /// make a best effort attempt to derive a valid Purl from data elements contained within the
    /// SBOM itself.
    pub fn purl(&self) -> Option<String> {
        let component = self.metadata.clone()?.component?;

        if let Some(purl) = component.purl {
            return Some(purl);
        }

        let name = component.name;
        let version = component.version.unwrap_or("0.0.0".to_string());

        let components = self.components.clone()?;

        let component_purl = components
            .iter()
            .next()
            .map(|component| component.purl.clone().unwrap());

        let component_purl = component_purl.unwrap();
        let component_purl_parts = component_purl.split('/').collect::<Vec<&str>>();
        let package_manager = component_purl_parts[0].clone();

        let purl = format!("{package_manager}/{name}@{version}");

        // TODO: Figure out if we can/should set this in the raw SBOM.
        // self.metadata?.component?.purl = Some(purl.clone());

        Some(purl)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn can_infer_purl_from_sbom() -> Result<(), Error> {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("cannot access CARGO_MANIFEST_DIR");
        println!("{}", manifest_dir);

        let manifest_dir =
            manifest_dir.replace("sdk/core", "tests/fixtures/core/entities/cyclonedx");
        println!("{}", manifest_dir);

        struct TestCase {
            file_name: String,
            expected: String,
        }

        let test_cases = vec![
            TestCase {
                file_name: "sbom-with-version.json".to_string(),
                expected: "pkg:cargo/harbor@0.1.0".to_string(),
            },
            TestCase {
                file_name: "sbom-without-version.json".to_string(),
                expected: "pkg:cargo/harbor@0.0.0".to_string(),
            },
        ];

        for test_case in test_cases {
            let file_name = test_case.file_name;
            let sbom_fixture_path = format!("{manifest_dir}/{file_name}");

            let raw = std::fs::read_to_string(sbom_fixture_path)
                .map_err(|e| Error::Runtime(format!("error reading SBOM test fixture: {}", e)))?;

            let bom = Bom::parse(raw.as_str(), CdxFormat::Json)?;
            let expected_purl = test_case.expected;
            let bom_purl = bom.purl().unwrap();

            println!("{}", bom_purl);
            assert_eq!(bom_purl, expected_purl);
        }

        Ok(())
    }
}
