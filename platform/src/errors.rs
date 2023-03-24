use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum Error {
    #[error("access denied: {0}")]
    AccessDenied(String),
    #[error("error in cognito provider: {0}")]
    Cognito(String),
    #[error("invalid configuration: {0}")]
    Config(String),
    #[error("error executing delete: {0}")]
    Delete(String),
    #[error("error with entity specification: {0}")]
    Entity(String),
    #[error("error executing insert: {0}")]
    Insert(String),
    #[error("error in migration: {0}")]
    Migration(String),
    #[error("error in mongo provider: {0}")]
    Mongo(String),
    #[error("error executing query: {0}")]
    Query(String),
    #[error("error serializing item: {0}")]
    Serde(String),
    #[error("error executing update: {0}")]
    Update(String),
    #[error("error writing: {0}")]
    Write(String),
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
