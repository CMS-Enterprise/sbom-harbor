use crate::Error;
use regex::Regex;
use std::path::Path;
use tempfile::TempDir;
use uuid::Uuid;

/// Function to make the file name safe
pub fn make_file_name_safe(purl: &str) -> Result<String, Error> {
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    Ok(re.replace_all(purl, "-").to_string())
}

/// Creates a temp directory in an OS agnostic way. Callers are required to ensure the directory
/// is deleted when no longer needed.
pub fn create_temp_dir() -> Result<String, Error> {
    let temp_dir = TempDir::new().map_err(|e| Error::FileSystem(e.to_string()))?;
    Ok(temp_dir.into_path().to_string_lossy().to_string())
}

/// Writes the bytes to the specified folder.
pub fn write_temp_file(bytes: &[u8], path: &str) -> Result<String, Error> {
    let buf = Path::new(path);
    if !buf.is_dir() {
        return Err(Error::FileSystem("path must be a directory".to_string()));
    }

    let file_name = Uuid::new_v4().to_string();
    let path = std::path::Path::new(path).join(file_name);

    std::fs::write(path.as_os_str(), bytes).map_err(|e| Error::FileSystem(e.to_string()))?;

    Ok(path.as_os_str().to_string_lossy().to_string())
}
