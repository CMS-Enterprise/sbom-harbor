/// Contains the types and functions to support the Enrich Command.
pub mod enrich;
/// Contains the types and functions to support the SBOM Command.
pub mod sbom;


/// Extracts the value of an environment variable
///
fn get_env_var(variable_name: &str) -> Option<String> {
    return match env::var(variable_name) {
        Ok(v) => Some(v),
        Err(e) => None,
    };
}