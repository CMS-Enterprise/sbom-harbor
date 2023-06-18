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
    /// Ingest runtime error.
    #[error("ingest: {0}")]
    Ingest(String),
    /// Invalid argument.
    #[error("invalid argument: {0}")]
    InvalidArg(String),
    /// Invalid subcommand.
    #[error("invalid subcommand: {0}")]
    InvalidSubcommand(String),
}
