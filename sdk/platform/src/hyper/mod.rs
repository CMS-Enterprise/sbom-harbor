use hyper::header::InvalidHeaderValue;
use hyper::http::uri::InvalidUri;
use std::fmt::{Display, Formatter};
use thiserror::Error;

mod client;

pub use client::Client;
pub use hyper::{Method, StatusCode};

/// Utility functions for interacting with a Hyper HttpBody.
pub mod body;

const CONTENT_TYPE: &str = "content-type";

/// HTTP Content Types.
pub enum ContentType {
    /// Form data is sent in a single block in the HTTP message body.
    FormUrlEncoded,
    /// Content sent in JSON format encoded in the UTF-8 character encoding.
    Json,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::FormUrlEncoded => write!(f, "application/x-www-form-urlencoded"),
            ContentType::Json => write!(f, "application/json"),
        }
    }
}

/// Represents all handled Errors for this module.
#[derive(Error, Debug)]
pub enum Error {
    /// Error parsing [Body].
    #[error("error parsing body: {0}")]
    Body(String),
    /// Invalid [Header].
    #[error("invalid header: {0}")]
    InvalidHeader(String),
    /// Invalid [URI].
    #[error("invalid uri: {0}")]
    InvalidUri(String),
    /// Error in [Hyper] runtime.
    #[error("error in hyper runtime: {0}")]
    Hyper(String),
    /// Error calling remote resource.
    #[error("error from remote resource: {0}")]
    Remote(u16, String),
    /// Error serializing types.
    #[error("error serializing types: {0}")]
    Serde(String),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper(err.to_string())
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Error::Hyper(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        Error::Serde(err.to_string())
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Self {
        Error::InvalidHeader(err.to_string())
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Self {
        Error::InvalidUri(err.to_string())
    }
}
