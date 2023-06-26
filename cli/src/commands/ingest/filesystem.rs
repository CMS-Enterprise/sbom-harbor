use crate::commands::ingest::IngestArgs;
use crate::common;
use crate::common::enrichments;
use crate::common::ingestion;
use crate::Error;
use clap::Parser;
use harbcore::entities::vendors::{Product, Vendor};

// TODO: Refactor to handle vendors.

/// Args for generating an SBOM from the filesystem.
#[derive(Clone, Debug, Parser)]
pub struct FileSystemArgs {
    /// Path to a pre-processed SBOM file. The CLI will process the SBOM and store the raw file
    /// using the configured storage provider. Requires the `source` flag also be specified.
    #[arg(long)]
    pub(crate) file: Option<String>,

    /// Source of the pre-processed SBOM file. Ignored if the `path` flag is specified.
    #[arg(long)]
    pub(crate) source: Option<String>,

    /// Indicates whether to enrich the SBOM once it has been ingested. Defaults to false if
    /// omitted.
    #[arg(short, long)]
    pub(crate) enrich: bool,
}

impl FileSystemArgs {
    pub fn to_opts(&self, debug: bool) -> Result<ingestion::VendorOpts, Error> {
        Ok(ingestion::VendorOpts {
            file: "".to_string(),
            vendor: Vendor {
                id: "".to_string(),
                name: "".to_string(),
                products: vec![],
            },
            product: Product {
                id: "".to_string(),
                name: "".to_string(),
                version: "".to_string(),
            },
            name: "".to_string(),
            debug,
            version: "".to_string(),
        })
    }
}

pub(crate) async fn execute(args: &IngestArgs) -> Result<(), Error> {
    // TODO: this debug passing is a recognized code smell. We need to get the StorageProviders
    // consolidated and then we can handle everything in one place in the CommandContext.
    let debug = args.debug;
    let args = match &args.filesystem_args {
        None => {
            return Err(Error::InvalidArg("filesystem args required".to_string()));
        }
        Some(args) => args,
    };

    let opts = match args.to_opts(debug) {
        Ok(args) => args,
        Err(e) => {
            return Err(Error::Ingest(format!(
                "error parsing filesystem args: {}",
                e
            )));
        }
    };

    let (raw, sbom) = ingestion::ingest_vendor_supplied(&opts).await?;
    if debug {
        common::pretty_print_json(raw.as_str());
    }

    println!("==> success: SBOM ingested {}", sbom.id);

    if args.enrich {
        let vulns = enrichments::grype::from_raw_sbom(raw.as_bytes())?;
        if debug {
            let json = match serde_json::to_string(&vulns) {
                Ok(json) => json,
                Err(e) => return Err(Error::Enrich(e.to_string())),
            };
            println!("==> success: SBOM scanned");
            common::pretty_print_json(json.as_str());
        }
    }
    Ok(())
}
