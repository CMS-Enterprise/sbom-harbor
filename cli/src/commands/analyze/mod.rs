use crate::commands::analyze::sbom_detail::DetailArgs;
use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

mod sbom_detail;

/// The CommandFactory function for the `analyze` command.
pub async fn execute(args: &AnalyzeArgs) -> Result<(), Error> {
    match args.provider {
        AnalyticProviderKind::SbomDetail => sbom_detail::execute(args).await,
    }
}

/// Enumerates which analysis provider to employ.
#[derive(Clone, Debug)]
pub(crate) enum AnalyticProviderKind {
    /// Generate a Detailed report from SBOM data.
    SbomDetail,
}

impl ValueEnum for AnalyticProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::SbomDetail]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            AnalyticProviderKind::SbomDetail => PossibleValue::new("sbom-detail")
                .help("Generates a detailed analysis of all SBOMs and related enrichment data."),
        })
    }
}

/// Specifies the CLI args for the `analyze` command.
#[derive(Debug, Parser)]
pub struct AnalyzeArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,

    /// Specifies the kind of provider
    provider: AnalyticProviderKind,

    /// Flattened args for use with the Detail Provider
    #[command(flatten)]
    pub detail_args: Option<DetailArgs>,
}
