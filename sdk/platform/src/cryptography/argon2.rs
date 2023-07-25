use crate::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hashes a string with the default Argon2 function.
pub fn hash_string(input: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(input.as_bytes(), &salt)
        .map_err(|e| Error::Cryptography(e.to_string()))?
        .to_string())
}

/// Verifies a hashed string
pub fn verify(hash: &str, input: &str) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| Error::Cryptography(e.to_string()))?;
    Argon2::default()
        .verify_password(input.as_bytes(), &parsed_hash)
        .map_err(|e| Error::Cryptography(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn can_verify_hashed_string() -> Result<(), Error> {
        let test_case = "test_case";
        let hash = hash_string(test_case)?;

        verify(hash.as_str(), test_case)
    }
}
