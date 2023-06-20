use crate::Error;

/// Gets the manifest directory for the currently executing binary.
pub fn manifest_dir() -> Result<String, Error> {
    std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| Error::Runtime("cannot resolve CARGO_MANIFEST_DIR".to_string()))
}

/// Allows replacing a segment of the manifest directory path with a user specified path. Useful
/// for things like locating source code artifacts.
///
/// # Example
///
/// ```rust
/// use platform::testing;
///
/// fn print_replace_dir() {
///     let dir = testing::replace_dir("/sdk/core", "tests/fixtures/sbom-fixture.json");
///     println!("{}", dir);
/// }
/// ```
pub fn replace_dir(old_path: &str, new_path: &str) -> Result<String, Error> {
    let manifest_dir = manifest_dir()?;

    Ok(manifest_dir.replace(old_path, new_path))
}

/// Gets the path to the well-known test fixture directory.
pub fn fixture_dir() -> Result<String, Error> {
    let manifest_dir = manifest_dir()?;

    // strip all possible source paths to get to workspace root.
    let mut root = manifest_dir.replace("/sdk/core", "");
    root = root.split("api").next().unwrap().to_string();
    root = root.split("cli").next().unwrap().to_string();
    root = root.split("extensions").next().unwrap().to_string();
    root = root.split("sdk").next().unwrap().to_string();

    root = format!("{}tests/fixtures", root);

    Ok(root)
}

/// Appends a path to the well test-fixture directory.
pub fn fixture_path(path: &str) -> Result<String, Error> {
    let fixture_dir = fixture_dir()?;
    let path = path.trim_start_matches("/");

    Ok(format!("{}/{path}", fixture_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_fixture_dir() -> Result<(), Error> {
        let fixture_dir = fixture_dir()?;

        assert!(fixture_dir.ends_with("sbom-harbor/tests/fixtures"));

        Ok(())
    }

    #[test]
    fn can_get_fixture_path() -> Result<(), Error> {
        let fixture_path = fixture_path("/path/to/fixture")?;

        assert!(fixture_path.ends_with("sbom-harbor/tests/fixtures/path/to/fixture"));

        Ok(())
    }
}
