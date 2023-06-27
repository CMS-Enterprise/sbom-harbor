use crate::Error;
use clap::Parser;
use harbcore::config::*;

/// The CommandFactory function for the `health` command.
pub async fn execute(args: &HealthArgs) -> Result<(), Error> {
    // ensure snyk token is set
    print!("SNYK_TOKEN:");
    match snyk_token() {
        Ok(_) => {
            print!("OK\n")
        }
        Err(e) => {
            return Err(Error::Runtime(e.to_string()));
        }
    }

    harbcore::health::check(args.debug)
        .await
        .map_err(Error::from)
}

/// Specifies the CLI args for the `health` command.
#[derive(Debug, Parser)]
pub struct HealthArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,
}
