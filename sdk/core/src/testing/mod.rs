use crate::Error;
use platform::testing::fixture_path;

/// Returns the path to the SBOM test fixture.
pub fn sbom_fixture_path() -> Result<String, Error> {
    fixture_path("sbom-fixture.json").map_err(|e| Error::Config(e.to_string()))
}

/// Returns the SBOM test fixture as a String in memory.
pub fn sbom_raw() -> Result<String, Error> {
    let sbom_path = sbom_fixture_path()?;
    std::fs::read_to_string(sbom_path).map_err(|e| Error::Config(e.to_string()))
}
