use clap::Parser;
use harbor_cms::{commands, Cli, Commands, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Fisma(args)) => commands::fisma::execute(args).await,
        _ => {
            println!("command not found");
            std::process::exit(1);
        }
    }
}
