use crate::services::purl2cpe::service::Error as Purl2CpeError;
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
    /// Serialization error
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    /// Serialization error.
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
    /// GitHub provider error.
    #[error("github error: {0}")]
    GitHub(String),
    /// Snyk provider error.
    #[error("snyk error: {0}")]
    Snyk(String),
    /// Task provider error.
    #[error("task provider error: {0}")]
    Task(String),
    /// Vulnerability provider error.
    #[error("vulnerability provider error: {0}")]
    Vulnerability(String),
    /// Analytic provider error.
    #[error("analytic provider error: {0}")]
    Analytic(String),
    /// Sbom Scorecard processing error
    #[error("Sbom Scorecard processing error: {0}")]
    SbomScorecard(String),
    /// Error from IO
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Error from platform Git service
    #[error(transparent)]
    Purl2Cpe(#[from] Purl2CpeError),
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
