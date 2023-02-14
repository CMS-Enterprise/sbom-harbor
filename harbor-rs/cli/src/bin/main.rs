use std::io;
use clap::{Arg, ArgAction, Command, ArgMatches};
use std::process::{Command as SysCommand, Output};
use io::Result;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::env;
use octocrab::params;


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
    .subcommand(
        Command::new("start")
            .about("Start a Pilot Execution")
    )
    .get_matches();
}

// async fn create_team() -> Result<&str> {
//
//     let cf_domain = get_cf_domain();
//
//
//
//     let resp = reqwest::get(cf_domain)
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok("")
// }

fn get_ctkey_output() -> Result<Output> {
    return SysCommand::new("ctkey")
        .arg("viewjson")
        .arg("--username")
        .arg("")
        .arg("--account")
        .arg("")
        .arg("--cloud-access-role")
        .arg("")
        .arg("--url")
        .arg("https://cloudtamer.cms.gov")
        .arg("--idms")
        .arg("2")
        .arg("--password")
        .arg("")
        .output();
}

fn get_ct_key_data() -> CtKeyData {
    let output = get_ctkey_output();
    return match output {
        Ok(output) => {
            let stdout_results = String::from_utf8_lossy(&output.stdout);
            let ct_key_data = serde_json::from_str::<CtKeyResults>(stdout_results.as_ref());
            let json = match ct_key_data {
                Ok(v) => v,
                Err(e) => {
                    panic!("ERROR: {}", e);
                },
            };

            json.data
        },
        Err(e) => {
            panic!("`ctkey` was not found! Check your PATH! {}", e)
        },
    }
}

fn get_gh_token() -> String {
    return match env::var("GH_FETCH_TOKEN") {
        Ok(v) => v,
        Err(e) => panic!("$GH_FETCH_TOKEN is not set ({})", e)
    }
}

fn get_cf_domain() -> String {
    return match env::var("CF_DOMAIN") {
        Ok(v) => v,
        Err(e) => panic!("$CF_DOMAIN is not set ({})", e)
    }
}

async fn list_repos() {

    let token = get_gh_token();

    let octocrab = octocrab::OctocrabBuilder::new()
        .personal_token(token)
        .build()
        .unwrap();

    let current_page = octocrab
        .orgs("cmsgov")
        .list_repos()
        .repo_type(params::repos::Type::Sources)
        .sort(params::repos::Sort::Pushed)
        .direction(params::Direction::Descending)
        .per_page(100)
        .page(1u32)
        .send()
        .await;

    let mut current_page_value = match current_page {
        Ok(v) => v,
        Err(e) => panic!("Error trying to get page: {}", e)
    };

    let prs = current_page_value.take_items();

    for pr in prs.iter() {
        println!("Value: {}", pr.url.as_str());
    }
}

#[derive(Serialize, Deserialize)]
struct CtKeyResults {
    data: CtKeyData
}

#[derive(Serialize, Deserialize)]
struct CtKeyData {
    access_key: String,
    secret_access_key: String,
    session_token: String,
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

                    let ct_key_data = get_ct_key_data();
                    print!("DATA: {}", ct_key_data.access_key);

                    list_repos().await;

                }
                None => println!("Nothing"),
                Some((&_, _)) => todo!()
            }
        }
    }
}
