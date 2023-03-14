use std::any::type_name;
use std::env;
use std::format;
use std::process::{Command as SysCommand, Output};

use clap::{Arg, ArgAction, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use anyhow::{anyhow, Result as AnyhowResult};
use harbor_cli::commands::OutputFormat;
use harbor_cli::commands::pilot::{PilotCommand, PilotKind, PilotOpts};
use harbor_cli::http::{ContentType, get};

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

                    println!("Start matched, lets get it on");

                    PilotCommand::execute(
                        PilotOpts {
                            provider: PilotKind::GITHUB,
                            output_format: Some(OutputFormat::Text),
                            org: Some(String::from("cmsgov")),
                            account_num: Some(aws_account.to_string()),
                            env: Some(env.to_string()),
                        }
                    ).await.unwrap();
                }
                None => println!("Nothing"),
                Some((&_, _)) => todo!(),
            }
        }
    }
}
