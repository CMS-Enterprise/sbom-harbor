
use serde_derive::{
    Deserialize,
    Serialize
};
use thiserror::Error;

/// Represents all exposed Errors for this crate.
#[derive(Error, Debug, Deserialize, Serialize)]
pub enum Error {
    #[error("invalid format: {0}")]
    InvalidFormat(String)
}