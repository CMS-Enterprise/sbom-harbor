use clap::Parser;
use task_template::{commands, Cli, Commands, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Example(args)) => commands::example::execute(args).await,
        _ => {
            println!("command not found");
            std::process::exit(1);
        }
    }
}
