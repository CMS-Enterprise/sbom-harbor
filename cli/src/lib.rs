#![warn(missing_docs)]
//! The [CLI] crate contains command handling logic for the Harbor CLI application. It handles
//! translation of input from stdin, instantiation of the appropriate command handler and the
//! formatting of output for stdout. They should contain no business logic, and instead invoke
//! [Services] from the [Harbcore] crate.

use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};

/// Commands supported by the [CLI].
pub mod commands;

/// Errors defined for this crate.
pub mod errors;

/// Error exposed by this crate.
pub use errors::Error;

/// Parses subcommands and args.
#[derive(Debug, Parser)]
#[clap(name = "harbor-cli")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The command to run.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// The set of supported Commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Construct a data set
    Construct(commands::construct::ConstructArgs),
    /// Enrich an SBOM.
    Enrich(commands::enrich::EnrichArgs),
    /// Ingest one or more SBOMs from a directory or from an external SBOM Provider.
    Ingest(commands::ingest::IngestArgs),
    /// Generate reports from SBOM data
    Analyze(commands::analyze::AnalyzeArgs),
    /// Test db and internet connections and report success or error
    Health(commands::health::HealthArgs),
}

/// Allows specifying the output format.
#[derive(Clone, Debug)]
pub enum OutputFormat {
    /// Output as JSON.
    Json,
    /// Output as plaintext.
    Text,
}

impl ValueEnum for OutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Json, Self::Text]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            OutputFormat::Json => PossibleValue::new("json").help("print output as json to stdout"),
            OutputFormat::Text => {
                PossibleValue::new("text").help("print output as plain text to stdout")
            }
        })
    }
}
