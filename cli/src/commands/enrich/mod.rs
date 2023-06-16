use crate::commands::enrich::epss::EpssProvider;
use crate::commands::enrich::snyk::{SnykArgs, SnykProvider};
use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use std::str::FromStr;

/// EPSS enrichment command handler.
pub mod epss;
/// Snyk enrichment command handler.
pub mod snyk;

/// Enumerates the supported enrichment providers.
#[derive(Clone, Debug)]
pub enum EnrichmentProviderKind {
    /// Use the EPSS enrichment provider.
    Epss,
    /// Use the Snyk enrichment provider.
    Snyk,
}

/// The Enrich Command handler.
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    match args.provider {
        EnrichmentProviderKind::Epss => EpssProvider::execute(args).await,
        EnrichmentProviderKind::Snyk => SnykProvider::execute(args).await,
    }
}

impl ValueEnum for EnrichmentProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Epss, Self::Snyk]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Epss => PossibleValue::new("epss").help("Run EPSS enrichment"),
            Self::Snyk => PossibleValue::new("snyk").help("Run Snyk enrichment"),
        })
    }
}

impl FromStr for EnrichmentProviderKind {
    type Err = ();

    fn from_str(s: &str) -> Result<EnrichmentProviderKind, Self::Err> {
        let value = s.to_lowercase();
        let value = value.as_str();
        match value {
            "epss" => Ok(EnrichmentProviderKind::Epss),
            "snyk" | "s" => Ok(EnrichmentProviderKind::Snyk),
            _ => Err(()),
        }
    }
}

/// Specifies the CLI args for the `enrich` command.
#[derive(Debug, Parser)]
pub struct EnrichArgs {
    /// Specifies with Enrichment Provider to invoke.
    #[arg(short, long)]
    pub provider: EnrichmentProviderKind,

    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,

    /// Flattened args for use with the Snyk enrichment provider.
    #[command(flatten)]
    pub snyk_args: Option<SnykArgs>,
}
