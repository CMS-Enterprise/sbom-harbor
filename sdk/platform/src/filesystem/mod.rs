use crate::str::get_random_string;
use crate::Error;
use regex::Regex;

/// Generates a valid location for operations on data
pub fn get_tmp_location() -> String {
    format!("/tmp/harbor-debug/{}", get_random_string())
}

/// A function to remove a directory
pub fn remove_directory(directory: String) -> Result<(), Error> {
    match std::fs::remove_dir_all(directory) {
        Ok(_) => {
            println!("==> Successfully removed directory");
            Ok(())
        }
        Err(err) => Err(Error::Delete(format!("Error Removing Directory {}", err))),
    }
}

/// Function to make the file name safe
pub fn make_file_name_safe(purl: &str) -> Result<String, Error> {
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
