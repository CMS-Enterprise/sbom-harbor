use crate::Error;
use crate::commands::{Opts, OutputFormat};

#[derive(Clone)]
pub struct PilotOpts {
    pub output_format: Option<OutputFormat>,
    // Organization name for the source control provider (e.g. github organization).
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

pub struct PilotCommand {}

impl PilotCommand {
    pub fn execute(_opts: PilotOpts) -> Result<(), Error> {
        Ok(())
    }
}
