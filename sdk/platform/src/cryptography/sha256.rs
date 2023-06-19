use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

use crate::Error;

/// Calculates a fixed size numeric representation of the contents of a buffer using the SHA256
/// algorithm.
pub fn digest_sha256<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|e| Error::Cryptography(format!("digest_sha256::{}", e)))?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

/// Calculates and encodes a digital signature for the the contents of a buffer using the SHA256
/// algorithm.
pub fn reader_checksum_sha256<R: Read>(reader: R) -> Result<String, Error> {
    let digest: Digest = digest_sha256(reader)
        .map_err(|e| Error::Cryptography(format!("reader_checksum_256::{}", e)))?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}

/// Calculates and encodes a digital signature for the the contents of a file using the SHA256
/// algorithm.
pub fn file_checksum_sha256(file_path: String) -> Result<String, Error> {
    let input = File::open(file_path)
        .map_err(|e| Error::Cryptography(format!("file_checksum_sha256::{}", e)))?;

    let reader = BufReader::new(input);
    let digest: Digest = digest_sha256(reader)
        .map_err(|e| Error::Cryptography(format!("file_checksum_sha256::{}", e)))?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}
