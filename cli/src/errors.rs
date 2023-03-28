use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Pilot runtime error.
    #[error("error running pilot")]
    Pilot(String),
}
