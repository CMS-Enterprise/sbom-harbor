use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

mod purl2cpe;
mod nvd;

/// The CommandFactory function for the `construct` command.
pub async fn execute(args: &ConstructArgs) -> Result<(), Error> {
    match args.provider {
        ConstructionProviderKind::Purl2Cpe => purl2cpe::execute(args).await,
        ConstructionProviderKind::NvdCve => nvd::execute(args).await,
    }
}

/// Enumerates which construction provider to employ.
#[derive(Clone, Debug)]
pub(crate) enum ConstructionProviderKind {
    /// Generate a Detailed report from SBOM data.
    Purl2Cpe,
    /// Enum value to build a Collection to support NVD CVE lookups.
    NvdCve,
}

impl ValueEnum for ConstructionProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Purl2Cpe, Self::NvdCve]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            ConstructionProviderKind::Purl2Cpe => PossibleValue::new("purl2cpe")
                .help("Generates a dataset from the purl2cpe GitHub Repository: https://github.com/scanoss/purl2cpe."),
            ConstructionProviderKind::NvdCve => PossibleValue::new("nvd")
                .help("Generates a dataset from the purl2cpe GitHub Repository: https://github.com/scanoss/purl2cpe."),
        })
    }
}

/// Specifies the CLI args for the `construct` command.
#[derive(Debug, Parser)]
pub struct ConstructArgs {
    /// Specifies the kind of provider
    provider: ConstructionProviderKind,
}
