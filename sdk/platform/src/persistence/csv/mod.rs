use crate::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

/// Generates a string CSV format from the input.
pub fn to_csv(headers: Vec<&'static str>, rows: Vec<Vec<String>>) -> Result<String, Error> {
    let debug_file = "/Users/derek/code/scratch/debug/sbom-vulnerability.csv";
    let path_buf = PathBuf::from(debug_file);
    if path_buf.is_file() {
        std::fs::remove_file(debug_file).map_err(|e| Error::Runtime(e.to_string()))?;
    }

    std::fs::File::create(debug_file).expect("file create failed");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(debug_file)
        .unwrap();

    writeln!(file, "{}", headers.join(",")).map_err(|e| Error::Runtime(e.to_string()))?;

    for row in rows.iter() {
        for inner in row.iter() {
            writeln!(file, "{}", inner).map_err(|e| Error::Runtime(e.to_string()))?;
        }
    }

    let mut writer = csv::Writer::from_writer(Vec::new());

    writer
        .write_record(headers)
        .map_err(|e| Error::Runtime(e.to_string()))?;

    for row in rows.iter() {
        writer
            .write_record(row)
            .map_err(|e| Error::Runtime(e.to_string()))?;
    }

    let inner = writer
        .into_inner()
        .map_err(|e| Error::Runtime(e.to_string()))?;

    String::from_utf8(inner).map_err(|e| Error::Runtime(e.to_string()))
}
