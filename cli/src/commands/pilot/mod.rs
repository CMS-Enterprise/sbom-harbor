use std::env;
use crate::commands::{Opts, OutputFormat};
use crate::commands::pilot::github::GitHubProvider;
use crate::Error;
use async_trait::async_trait;
mod github;
mod snyk;

fn get_cf_domain() -> String {
    return match env::var("CF_DOMAIN") {
        Ok(v) => v,
        Err(e) => panic!("$CF_DOMAIN is not set ({})", e),
    };
}

#[async_trait]
pub trait Provider {
    async fn scan(&self);
}

pub enum PilotKind {
    GITHUB,
    SNYK,
}

// #[derive(Clone)]
pub struct PilotOpts {
    pub provider: PilotKind,
    pub output_format: Option<OutputFormat>,
    pub org: Option<String>,
    pub env : Option<String>,
    pub account_num: Option<String>,
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

// TODO Should this be a trait?
pub struct PilotCommand {}

impl PilotCommand {
    pub async fn execute(_opts: PilotOpts) -> Result<(), Error> {

        let provider = PilotFactory::new(
            _opts
        );

        provider.scan().await;

        Ok(())
    }
}

pub struct PilotFactory {}

impl PilotFactory {
    pub fn new(pilot_ops: PilotOpts) -> Box<dyn Provider> {
        return match pilot_ops.provider {
            PilotKind::GITHUB => Box::new(GitHubProvider {}),
            PilotKind::SNYK => panic!("Jon, return SnykProvider implementation"),
        };
    }
}
