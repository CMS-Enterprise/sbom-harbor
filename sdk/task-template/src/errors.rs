use thiserror::Error;

/// Example Error enumeration for extension.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error.
    #[error("config: {0}")]
    Config(String),
    /// Invalid argument.
    #[error("invalid argument: {0}")]
    InvalidArg(String),
    /// Invalid subcommand.
    #[error("invalid subcommand: {0}")]
    InvalidSubcommand(String),
    /// Task provider error.
    #[error("task provider: {0}")]
    Task(String),
}
