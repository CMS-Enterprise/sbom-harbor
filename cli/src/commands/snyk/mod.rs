use thiserror::Error;

pub mod snyk;
pub use snyk::*;

#[derive(Error, Debug)]
pub enum SnykProviderError {
    #[error("error connecting to Snyk: {0}")]
    SnykConnection(String),
}