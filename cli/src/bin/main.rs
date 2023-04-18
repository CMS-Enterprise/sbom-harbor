use clap::Parser;
use harbor_cli::{Cli, commands, Commands, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Enrich(enrich)) => commands::enrich::execute(enrich).await,
        Some(Commands::Sbom(sbom)) => commands::sbom::execute(sbom).await,
        _ => {
            println!("command not found");
            std::process::exit(1);
        }
    }
}
