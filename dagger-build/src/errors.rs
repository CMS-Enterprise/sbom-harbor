use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error.
    #[error("config: {0}")]
    Config(String),
    /// Error connecting to dagger engine.
    #[error("failed to connect to dagger engine.")]
    Connect(#[source] dagger_sdk::errors::ConnectError),
    /// Error in dagger runtime.
    #[error("error executing dagger task")]
    Dagger(#[source] dagger_sdk::errors::DaggerError),
    /// Error in dagger query.
    #[error("error executing dagger graphql query")]
    Query(#[source] dagger_sdk::errors::DaggerUnpackError),
    /// Runtime error.
    #[error("Runtime error: {0}")]
    Runtime(String),
}

impl From<platform::Error> for Error {
    fn from(value: platform::Error) -> Self {
        Error::Runtime(value.to_string())
    }
}
