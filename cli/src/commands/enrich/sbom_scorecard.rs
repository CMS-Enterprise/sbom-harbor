use std::path::PathBuf;

use clap::Parser;
use harbcore::services::sboms::sbom_scorecard::{show_sbom_scorecard, compare_sbom_scorecards};
use crate::commands::enrich::EnrichArgs;
use crate::Error;


/// Args for use with the Sbom scorecard enrichment.
#[derive(Clone, Debug, Parser)]
pub struct SbomScorecardArgs {
        /// The file path to create a scorecard from
        #[arg(long)]
        pub sbom_file_path_1: Option<String>,
        /// Second file path to compare with the first
        #[arg(long)]
        pub sbom_file_path_2: Option<String>,
}

/// Handles Sbom Scorecard enrichment commands
pub struct SbomScorecardProvider {}

impl SbomScorecardProvider {

    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
        let mut test_sbom = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_sbom.push("src/services/sboms/sbom_scorecard/test_files/dropwizard.json");

        match &args.sbom_scorecard_args {
            Some(scorecard_args) => {
                match (scorecard_args.sbom_file_path_1.clone(), scorecard_args.sbom_file_path_2.clone()) {
                    (None, None) => {
                        let error_string = "No Paths provided, please use --sbom-file-path-1 <path>, and --sbom-file-path-2 <path>";
                        return Err(Error::SbomScorecard(format!("Failed to compare scorecards due to errors: {}", error_string)));
                    },
                    (None, Some(path)) => {
                        let results = show_sbom_scorecard(path);
                        match results {
                            Ok(valid_result) => println!("\n{}", valid_result),
                            Err(error) => println!("Failed with errors: \n{}", error),
                        }
                    },
                    (Some(path), None) => {
                        let results = show_sbom_scorecard(path);
                        match results {
                            Ok(valid_result) => println!("\n{}", valid_result),
                            Err(error) => println!("Failed with errors: \n{}", error),
                        }
                    },
                    (Some(path_1), Some(path_2)) => {
                        let results = compare_sbom_scorecards(path_1, path_2);
                        match results {
                            Ok(valid_result) => println!("\n{}", valid_result),
                            Err(error) => println!("Failed with errors: \n{}", error),
                        }
                    },
                }
            },
            None => {
                let error_string = "A path to an Sbom file must be provided, please use --sbom-file-path-1 <path>";
                return Err(Error::SbomScorecard(format!("Failed to compare scorecard due to errors: {}", error_string)))
            }
        }
        return Ok(());
    }
}