use clap::Parser;
use harbor_cli::{commands, Cli, Commands, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Enrich(enrich)) => commands::enrich::execute(enrich).await,
        Some(Commands::Ingest(ingest)) => commands::ingest::execute(ingest).await,
        Some(Commands::Analyze(analyze)) => commands::analyze::execute(analyze).await,
        Some(Commands::Health(health)) => commands::health::execute(health).await,
        _ => {
            println!("command not found");
            std::process::exit(1);
        }
    }
}
