use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

use crate::Error;

pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|e| Error::Cryptography(format!("sha256_digest::{}", e)))?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn reader_checksum<R: Read>(mut reader: R) -> Result<String, Error> {
    let digest: Digest = sha256_digest(reader)
        .map_err(|e| Error::Cryptography(format!("reader_checksum::{}", e)))?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}

pub fn file_checksum(file_path: String) -> Result<String, Error> {
    let input =
        File::open(file_path).map_err(|e| Error::Cryptography(format!("file_checksum::{}", e)))?;

    let reader = BufReader::new(input);
    let digest: Digest =
        sha256_digest(reader).map_err(|e| Error::Cryptography(format!("file_checksum::{}", e)))?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}
