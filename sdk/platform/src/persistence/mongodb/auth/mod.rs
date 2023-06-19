/// Contains a default authorization configuration based on AWS IAM Policies and Roles model.
pub mod init_default_auth;

mod authorization;
pub use authorization::*;
