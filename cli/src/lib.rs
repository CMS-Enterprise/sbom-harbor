#![warn(missing_docs)]
//! The [CLI] crate contains command handling logic for the Harbor CLI application. It handles
//! translation of input from stdin, instantiation of the appropriate command handler and the
//! formatting of output for stdout. They should contain no business logic, and instead invoke
//! [Services] from the [Harbcore] crate.

/// Commands supported by the [CLI].
pub mod commands;

/// Errors defined for this crate.
pub mod errors;

/// Error exposed by this crate.
pub use errors::Error;
