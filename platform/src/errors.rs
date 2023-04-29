use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug, Deserialize, Serialize)]
pub enum Error {
    /// Access denied.
    #[error("access denied: {0}")]
    AccessDenied(String),
    /// Error in Cognito provider.
    #[error("error in cognito provider: {0}")]
    Cognito(String),
    /// Invalid configuration.
    #[error("invalid configuration: {0}")]
    Config(String),
    /// Error in cryptography provider.
    #[error("error in cryptography provider: {0}")]
    Cryptography(String),
    /// Error executing delete.
    #[error("error executing delete: {0}")]
    Delete(String),
    /// Error in encoding provider.
    #[error("error in encoding provider: {0}")]
    Encoding(String),
    /// Error with entity specification.
    #[error("error with entity specification: {0}")]
    Entity(String),
    /// Error making HTTP Request.
    #[error("error in HTTP request: {0}")]
    Http(String),
    /// Error executing insert.
    #[error("error executing insert: {0}")]
    Insert(String),
    /// Error in db migration.
    #[error("error in db migration: {0}")]
    Migration(String),
    /// Error in Mongo provider.
    #[error("error in mongo provider: {0}")]
    Mongo(String),
    /// Error in OpenTelemetry provider.
    #[error("error in opentelemetry provider: {0}")]
    OpenTelemetry(String),
    /// Error executing query.
    #[error("error executing query: {0}")]
    Query(String),
    /// Error serializing item.
    #[error("error serializing item: {0}")]
    Serde(String),
    /// Error in S3 provider.
    #[error("error in S3 provider: {0}")]
    S3(String),
    /// Error in Time module.
    #[error("error in time modoule: {0}")]
    Time(String),
    /// Error executing update.
    #[error("error executing update: {0}")]
    Update(String),
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Error::Mongo(format!("{:?}", value))
    }
}

impl From<mongodb::bson::oid::Error> for Error {
    fn from(value: mongodb::bson::oid::Error) -> Self {
        Error::Mongo(format!("{:?}", value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(format!("{:?}", value))
    }
}

impl From<crate::hyper::Error> for Error {
    fn from(value: crate::hyper::Error) -> Self {
        Error::Http(format!("{:?}", value))
    }
}
