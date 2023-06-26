use crate::commands::analyze::sbom_detail::DetailArgs;
use crate::commands::analyze::sbom_vulnerability::VulnerabilityArgs;
use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

mod sbom_detail;
mod sbom_vulnerability;

/// The CommandFactory function for the `analyze` command.
pub async fn execute(args: &AnalyzeArgs) -> Result<(), Error> {
    match args.provider {
        AnalyticProviderKind::SbomDetail => sbom_detail::execute(args).await,
        AnalyticProviderKind::SbomVulnerability => sbom_vulnerability::execute(args).await,
    }
}

/// Enumerates which analysis provider to employ.
#[derive(Clone, Debug)]
pub(crate) enum AnalyticProviderKind {
    /// Generate a Detailed report from SBOM data.
    SbomDetail,
    /// Generate an export of SBOM Vulnerability data.
    SbomVulnerability,
}

impl ValueEnum for AnalyticProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::SbomDetail, Self::SbomVulnerability]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            AnalyticProviderKind::SbomDetail => PossibleValue::new("sbom-detail")
                .help("Generates a detailed analysis of all SBOMs and related enrichment data."),
            AnalyticProviderKind::SbomVulnerability => PossibleValue::new("sbom-vulnerability")
                .help("Generates an export of all SBOMs and their related vulnerability data."),
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

    /// Flattened args for use with the `sbom-detail` command.
    #[command(flatten)]
    pub detail_args: Option<DetailArgs>,

    /// Flattened args for use with the `sbom-vulnerability` command.
    #[command(flatten)]
    pub vulnerability_args: Option<VulnerabilityArgs>,
}
