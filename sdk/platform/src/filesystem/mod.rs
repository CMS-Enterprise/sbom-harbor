use crate::Error;
use regex::Regex;

/// Function to make the file name safe
pub fn make_file_name_safe(purl: &str) -> Result<String, Error> {
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    Ok(re.replace_all(purl, "-").to_string())
}
