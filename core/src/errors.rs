use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error.
    #[error("config error: {0}")]
    Config(String),
    /// Invalid format.
    #[error("invalid format: {0}")]
    InvalidFormat(String),
    /// Error during db migrations.
    #[error("migrations error: {0}")]
    Migrations(String),
    /// Runtime error.
    #[error("runtime error: {0}")]
    Runtime(String),
}


impl From<platform::Error> for Error {
    fn from(value: platform::Error) -> Self {
        Error::Runtime(format!("{:?}", value))
    }
}
