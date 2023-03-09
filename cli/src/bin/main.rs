use std::env;
use std::format;
use std::process::{Command as SysCommand, Output};

use anyhow::{anyhow, Result as AnyhowResult};
use clap::{Arg, ArgAction, ArgMatches, Command};
use harbor_cli::commands::{PilotFactory, PilotKind, PilotOps, PilotOpts};
use harbor_cli::http::{get, ContentType};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::any::type_name;

fn get_matches() -> ArgMatches {
    return Command::new("harbor-cli")
        .about("SBOM Harbor Runtime CLI")
        .version("0.0.1")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("SBOM Harbor Team")
        .arg(
            Arg::new("account")
                .short('a')
                .required(true)
                .long("account")
                .help("aws account id")
                .action(ArgAction::Set)
                .num_args(1),
        )
        .arg(
            Arg::new("env")
                .short('e')
                .required(true)
                .long("env")
                .help("environment, ephemeral or permanent.")
                .action(ArgAction::Set)
                .num_args(1),
        )
        .subcommand(Command::new("start").about("Start a Pilot Execution"))
        .get_matches();
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[tokio::main]
async fn main() {
    let matches = get_matches();
    if let Some(aws_account) = matches.get_one::<String>("account") {
        println!("Account Number: {}", aws_account);
        if let Some(env) = matches.get_one::<String>("env") {
            println!("Environment: {}", env);
            match matches.subcommand() {
                Some(("start", _)) => {
                    let provider = PilotFactory::new(PilotOpts {
                        provider: PilotKind.GITHUB,
                    });

                    provider.scan();
                }
                None => println!("Nothing"),
                Some((&_, _)) => todo!(),
            }
        }
    }
}
