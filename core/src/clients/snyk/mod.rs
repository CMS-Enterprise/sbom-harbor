use thiserror::Error;

mod snyk_data_model;
pub use snyk_data_model::*;

mod snyk_client;
pub use snyk_client::*;

#[derive(Error, Debug)]
pub enum SnykProviderError {
    #[error("error connecting to Snyk: {0}")]
    SnykConnection(String),

    #[error("Invalid data recieved from Snyk: {0}")]
    SnykDataValidationError(String),

    #[error("No projects found: {0}")]
    SnykNoProjectsFoundError(String),

    #[error("No SBOM found: {0}")]
    SnykSBOMError(String),
}