use crate::Error;
use crate::commands::{Opts, OutputFormat};

/// Options for running SBOM Pilot commands.
#[derive(Clone)]
pub struct PilotOpts {
    /// Format to return output in.
    pub output_format: Option<OutputFormat>,
    /// Organization name for the source control provider (e.g. github organization).
    pub org: Option<String>,
}

impl Opts for PilotOpts {
    fn format(&self) -> OutputFormat {
        let format = self.output_format.clone();
        match format {
            None => OutputFormat::Text,
            Some(format) => format,
        }
    }
}

/// Command handler for [Pilot] operations.
pub struct PilotCommand {}

impl PilotCommand {
    /// Runs the command handler.
    pub fn execute(_opts: PilotOpts) -> Result<(), Error> {
        Ok(())
    }
}
