use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug)]
pub enum Error {
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
    /// SBOM runtime error.
    #[error("sbom: {0}")]
    Sbom(String),
}
