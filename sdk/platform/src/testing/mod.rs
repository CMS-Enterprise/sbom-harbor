use crate::Error;
use std::path::Path;

/// Utility functions and types for testing persistence features.
pub mod persistence;

/// Allows replacing a segment of the manifest directory path with a user specified path. Useful
/// for things like locating source code artifacts.
///
/// # Example
///
/// ```rust
/// use platform::Error;
/// use platform::testing;
///
/// fn print_replace_dir() -> Result<(), Error> {
///     let dir = testing::replace_dir("/sdk/core", "tests/fixtures/sbom-fixture.json")?;
///     println!("{}", dir);
///     Ok(())
/// }
/// ```
pub fn replace_dir(old_path: &str, new_path: &str) -> Result<String, Error> {
    let workspace_dir = workspace_dir()?;

    Ok(workspace_dir.replace(old_path, new_path))
}

/// Gets the path to the workspace root.
pub fn workspace_dir() -> Result<String, Error> {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    Ok(cargo_path
        .parent()
        .unwrap()
        .to_path_buf()
        .display()
        .to_string())
}

/// Gets the path to the well-known test fixture directory.
pub fn fixture_dir() -> Result<String, Error> {
    let mut fixture_dir = workspace_dir()?;

    fixture_dir = format!("{}/tests/fixtures", fixture_dir);

    Ok(fixture_dir)
}

/// Appends a path to the well test-fixture directory.
pub fn fixture_path(path: &str) -> Result<String, Error> {
    let fixture_dir = fixture_dir()?;
    let path = path.trim_start_matches('/');

    Ok(format!("{}/{path}", fixture_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_fixture_dir() -> Result<(), Error> {
        let fixture_dir = fixture_dir()?;

        assert!(fixture_dir.ends_with("tests/fixtures"));

        Ok(())
    }

    #[test]
    fn can_get_fixture_path() -> Result<(), Error> {
        let fixture_path = fixture_path("/path/to/fixture")?;

        assert!(fixture_path.ends_with("tests/fixtures/path/to/fixture"));

        Ok(())
    }
}
