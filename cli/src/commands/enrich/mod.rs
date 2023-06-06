use crate::commands::enrich::epss::EpssProvider;
use crate::commands::enrich::snyk::{SnykArgs, SnykProvider};
use crate::commands::enrich::sbom_scorecard::{SbomScorecardArgs, SbomScorecardProvider};
use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use std::str::FromStr;

/// EPSS enrichment command handler.
pub mod epss;
/// Snyk enrichment command handler.
pub mod snyk;
/// Sbom Scorecard enrichment command handler.
pub mod sbom_scorecard;

/// Enumerates the supported enrichment providers.
#[derive(Clone, Debug)]
pub enum EnrichmentProviderKind {
    /// Use the EPSS enrichment provider.
    Epss,
    /// Use the Snyk enrichment provider.
    Snyk,
    /// Use the sbom-scorecard enrichment provider
    SbomScorecard,
}

/// The Enrich Command handler.
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    match args.provider {
        EnrichmentProviderKind::Epss => EpssProvider::execute(args).await,
        EnrichmentProviderKind::Snyk => SnykProvider::execute(args).await,
        EnrichmentProviderKind::SbomScorecard => SbomScorecardProvider::execute(args).await,
    }
}

impl ValueEnum for EnrichmentProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Epss, Self::Snyk, Self::SbomScorecard]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Epss => PossibleValue::new("epss").help("Run EPSS enrichment"),
            Self::Snyk => PossibleValue::new("snyk").help("Run Snyk enrichment"),
            Self::SbomScorecard => PossibleValue::new("sbom-scorecard").help("Run Sbom Scorecard"),
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
            "sbom-scorecard" | "score" => Ok(EnrichmentProviderKind::SbomScorecard),
            _ => Err(()),
        }
    }
}

/// Specifies the CLI args for the Enrich command.
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

    /// Flattened args for use with the Sbom Scorecard enrichment
    #[command(flatten)]
    pub sbom_scorecard_args: Option<SbomScorecardArgs>,
}
