use thiserror::Error;

/// Example Error enumeration for extension.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error.
    #[error("config: {0}")]
    Config(String),
    /// Export provider error.
    #[error("fisma provider: {0}")]
    Export(String),
    /// Fisma provider error.
    #[error("fisma provider: {0}")]
    Fisma(String),
    /// Invalid argument.
    #[error("invalid argument: {0}")]
    InvalidArg(String),
    /// Invalid subcommand.
    #[error("invalid subcommand: {0}")]
    InvalidSubcommand(String),
    /// Snyk service error.
    #[error("snyk provider: {0}")]
    Snyk(String),
}
