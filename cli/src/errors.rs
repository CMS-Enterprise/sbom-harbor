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
    /// Sbom runtime error.
    #[error("sbom: {0}")]
    Sbom(String),
    /// System error.
    #[error("system: {0}")]
    System(String),
}

impl From<platform::Error> for Error {
    fn from(error: platform::Error) -> Self {
        Error::System(error.to_string())
    }
}
