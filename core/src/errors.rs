use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error.
    #[error("config error: {0}")]
    Config(String),
    /// Entity error.
    #[error("entity error: {0}")]
    Entity(String),
    /// Enrichment provider error.
    #[error("enrichment provider error: {0}")]
    Enrichment(String),
    /// Finding provider error.
    #[error("finding provider error: {0}")]
    Finding(String),
    /// Invalid format.
    #[error("invalid format: {0}")]
    InvalidFormat(String),
    /// Error during db migrations.
    #[error("migrations error: {0}")]
    Migrations(String),
    /// Error calling remote resource.
    #[error("remote error: {0}")]
    Remote(String),
    /// Runtime error.
    #[error("runtime error: {0}")]
    Runtime(String),
    /// Sbom provider error.
    #[error("sbom provider error: {0}")]
    Sbom(String),
    #[error("scan provider error: {0}")]
    Scan(String),
    /// Serialization error.
    #[error("serialization error: {0}")]
    Serde(String),
    /// Snyk provider error.
    #[error("snyk error: {0}")]
    Snyk(String),
}

impl From<platform::Error> for Error {
    fn from(value: platform::Error) -> Self {
        Error::Runtime(format!("{:?}", value))
    }
}

impl From<platform::hyper::Error> for Error {
    fn from(value: platform::hyper::Error) -> Self {
        Error::Runtime(format!("{:?}", value))
    }
}
