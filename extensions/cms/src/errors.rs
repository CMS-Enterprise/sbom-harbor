use thiserror::Error;

/// Example Error enumeration for extension.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error.
    #[error("config: {0}")]
    Config(String),
    /// Fisma provider error.
    #[error("fisma provider: {0}")]
    Fisma(String),
    /// Invalid argument.
    #[error("invalid argument: {0}")]
    InvalidArg(String),
    /// Ion Channel provider error.
    #[error("ion channel provider: {0}")]
    IonChannel(String),
    /// Invalid subcommand.
    #[error("invalid subcommand: {0}")]
    InvalidSubcommand(String),
    /// Snyk service error.
    #[error("snyk provider: {0}")]
    Snyk(String),
}
