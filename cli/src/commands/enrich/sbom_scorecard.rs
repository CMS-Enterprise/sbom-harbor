use crate::commands::enrich::EnrichArgs;
use crate::Error;
use clap::Parser;
use harbcore::services::sboms::scorecard::{compare, show};

/// Args for use with the Sbom scorecard enrichment.
#[derive(Clone, Debug, Parser)]
pub struct SbomScorecardArgs {
    /// The file path to create a scorecard from
    #[arg(short, long, num_args(0..=2))]
    pub file: Option<Vec<String>>,
}

/// Handles Sbom Scorecard enrichment commands
pub struct SbomScorecardProvider {}

impl SbomScorecardProvider {
    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
        let mut error_string = "".to_string();
        let mut files = vec![];
        match &args.sbom_scorecard_args {
            Some(args) => match &args.file {
                Some(file) => {
                    files = file.clone();
                }
                None => {
                    error_string = "No Paths provided. Please use --file <path>".to_string();
                }
            },
            None => {
                error_string = "No Paths provided. Please use --file <path>".to_string();
            }
        };

        match files.len() {
            1 => match show(files[0].clone()) {
                Ok(valid_result) => println!("\n{}", valid_result),
                Err(error) => error_string = error.to_string(),
            },
            2 => match compare(files[0].clone(), files[1].clone()) {
                Ok(valid_result) => println!("\n{}", valid_result),
                Err(error) => error_string = error.to_string(),
            },
            _ => error_string = "Invalid file path args. One or two paths supported.".to_string(),
        }

        match error_string.is_empty() {
            true => {}
            false => {
                return Err(Error::SbomScorecard(format!(
                    "Failed with errors: {}",
                    error_string
                )));
            }
        }

        Ok(())
    }
}
