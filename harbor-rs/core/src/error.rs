use thiserror::Error;

// TODO: These are API Specific errors.  Need to build Core Error Enum.
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid parameters")]
    InvalidParameters(String),
    #[error("invalid token")]
    InvalidToken(String),
    #[error("missing credentials")]
    MissingCredentials(String),
    #[error("wrong credentials")]
    WrongCredentials(String),
    #[error("failed to create token")]
    TokenCreation(String),
    #[error("an internal server error occurred")]
    InternalServerError(String),
    #[error("resource does not exist")]
    DoesNotExist(String),
    #[error("resource already exists")]
    AlreadyExists(String),
}
