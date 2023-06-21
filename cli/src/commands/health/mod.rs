use crate::Error;

use clap::Parser;

/// The CommandFactory function for the `health` command.
pub async fn execute(_args: &HealthArgs) -> Result<(), Error> {
    println!("health");
    Ok(())
}

/// Specifies the CLI args for the `analyze` command.
#[derive(Debug, Parser)]
pub struct HealthArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,
}
