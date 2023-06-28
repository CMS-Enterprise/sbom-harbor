use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Analytic runtime error.
    #[error("sbom: {0}")]
    Analyze(String),
    /// Configuration error.
    #[error("config: {0}")]
    Config(String),
    /// Enrich runtime error.
    #[error("enrich: {0}")]
    Enrich(String),
    /// Invalid argument.
    #[error("invalid argument: {0}")]
    InvalidArg(String),
    /// Invalid subcommand.
    #[error("invalid subcommand: {0}")]
    InvalidSubcommand(String),
    /// Runtime error.
    #[error("Runtime error: {0}")]
    Runtime(String),
    /// Sbom Scorecard processing error
    #[error("Sbom Scorecard command error: {0}")]
    SbomScorecard(String),
    /// Sbom runtime error.
    #[error("sbom: {0}")]
    Sbom(String),
}

impl From<harbcore::Error> for Error {
    fn from(value: harbcore::Error) -> Self {
        Error::Runtime(value.to_string())
    }
}

impl From<platform::Error> for Error {
    fn from(value: platform::Error) -> Self {
        Error::Runtime(value.to_string())
    }
}
