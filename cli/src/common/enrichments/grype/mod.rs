use crate::Error;
use harbcore::entities::enrichments::Vulnerability;
use harbcore::services::grype::{scan, ScanKind};
use platform::filesystem::{create_temp_dir, write_temp_file};
use std::path::PathBuf;

/// Stores the raw sbom to a temporary file, and then runs Grype to scan it for vulnerabilities.
pub(crate) fn from_raw_sbom(bytes: &[u8]) -> Result<Vec<Vulnerability>, Error> {
    let dir = create_temp_dir().map_err(|e| Error::Enrich(e.to_string()))?;

    let file_path =
        write_temp_file(bytes, dir.as_str()).map_err(|e| Error::Enrich(e.to_string()))?;

    let results =
        scan(file_path.as_str(), ScanKind::Sbom).map_err(|e| Error::Enrich(e.to_string()))?;

    std::fs::remove_dir_all(dir).map_err(|e| Error::Enrich(e.to_string()))?;

    Ok(results)
}

/// Runs Grype to scan a source control repository for vulnerabilities.
#[allow(dead_code)]
pub(crate) fn from_directory(dir_path: &str) -> Result<Vec<Vulnerability>, Error> {
    let path_buf = PathBuf::from(dir_path);
    if !path_buf.is_dir() {
        return Err(Error::Enrich("path must be a directory".to_string()));
    }

    scan(dir_path, ScanKind::Directory).map_err(|e| Error::Enrich(e.to_string()))
}
