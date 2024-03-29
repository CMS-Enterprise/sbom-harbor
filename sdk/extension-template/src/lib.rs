#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
#![warn(missing_docs)]
//! This crate contains a template that can be use to develop a new [Extension Task]. Extensions
//! provide a way for operators to define custom behaviors that are specific to their Harbor
//! operating environment, and aren't usable in other Harbor installations.
//!
//! For an example, see `extensions/cms`. We develop those extensions as in the open to provide an
//! example of how to extend Harbor without the need to maintain a fork of the core project.

use clap::{Parser, Subcommand};

/// Commands supported by the Extension.
pub mod commands;

/// Errors defined for this crate.
pub mod errors;

/// Tasks that can be performed by this CLI.
pub mod tasks;

/// Error exposed by this crate.
pub use errors::Error;

/// Parses subcommands and args.
#[derive(Debug, Parser)]
#[clap(name = "extension-cli")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The command to run.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// The set of supported Commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Example command - lists Packages.
    Example(commands::example::ExampleArgs),
}
