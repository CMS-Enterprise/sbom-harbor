use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::info;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid parameters.
    #[error("invalid parameters")]
    InvalidParameters(String),
    /// Invalid token.
    #[error("invalid token")]
    InvalidToken(String),
    /// Missing credentials.
    #[error("missing credentials")]
    MissingCredentials(String),
    /// Wrong credentials.
    #[error("wrong credentials")]
    WrongCredentials(String),
    /// Failure creating token.
    #[error("failed to create token")]
    TokenCreation(String),
    /// Internal Server Error.
    #[error("an internal server error occurred")]
    InternalServerError(String),
    /// Resource does not exist.
    #[error("resource does not exist")]
    DoesNotExist(String),
    /// Resource already exists.
    #[error("resource already exists")]
    AlreadyExists(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            Error::InvalidParameters(m) => (StatusCode::UNPROCESSABLE_ENTITY, m),
            Error::InvalidToken(m) => (StatusCode::BAD_REQUEST, m),
            Error::WrongCredentials(m) => (StatusCode::UNAUTHORIZED, m),
            Error::MissingCredentials(m) => (StatusCode::BAD_REQUEST, m),
            Error::TokenCreation(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
            Error::InternalServerError(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
            Error::DoesNotExist(m) => (StatusCode::BAD_REQUEST, m),
            Error::AlreadyExists(m) => (StatusCode::BAD_REQUEST, m),
        };

        info!("{}: {}", status_code, message);

        (status_code, message).into_response()
    }
}
