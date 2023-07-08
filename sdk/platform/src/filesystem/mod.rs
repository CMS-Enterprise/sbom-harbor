use crate::Error;
use regex::Regex;

/// Sanitize a string so that it can be used as a filename.
pub fn to_safe_file_name(purl: &str) -> Result<String, Error> {
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    let result = re.replace_all(purl, "-");
    let mut result = result.as_ref();
    result = result.trim_end_matches('-');

    let mut result = result.trim_start_matches('-').to_string();

    while result.contains("--") {
        result = result.replace("--", "-");
    }

    Ok(result)
}
