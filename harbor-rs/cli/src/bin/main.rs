

use clap::{Arg, ArgAction, Command, ArgMatches};
use std::result::Result;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::env;
use octocrab::params;
use aws_sdk_dynamodb::{Client as DynamoClient, Error as DynamoError};

// use std::collections::HashMap;
// use aws_sdk_dynamodb::model::{
//     AttributeDefinition, KeySchemaElement,
//     KeyType, ProvisionedThroughput,
//     ScalarAttributeType,
// };

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

// async fn create_team() -> Result<()> {
//
//     let cf_domain = get_cf_domain();
//     let resp = reqwest::get(cf_domain)
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     // Ok("")
// }


fn get_gh_token() -> String {
    return match env::var("GH_FETCH_TOKEN") {
        Ok(v) => v,
        Err(e) => panic!("$GH_FETCH_TOKEN is not set ({})", e)
    }
}

// async fn create_table() -> Result<(), DynamoError> {
//
//     let shared_config = aws_config::load_from_env().await;
//     let client = DynamoClient::new(&shared_config);
//
//     let new_table = client
//         .create_table()
//         .table_name("test-table")
//         .key_schema(
//             KeySchemaElement::builder()
//                 .attribute_name("k")
//                 .key_type(KeyType::Hash)
//                 .build(),
//         )
//         .attribute_definitions(
//             AttributeDefinition::builder()
//                 .attribute_name("k")
//                 .attribute_type(ScalarAttributeType::S)
//                 .build(),
//         )
//         .provisioned_throughput(
//             ProvisionedThroughput::builder()
//                 .write_capacity_units(10)
//                 .read_capacity_units(10)
//                 .build(),
//         )
//         .send()
//         .await?;
//     println!(
//         "new table: {:#?}",
//         &new_table.table_description().unwrap().table_arn().unwrap()
//     );
//
//     Ok(())
// }

pub async fn list_tables() -> Result<Vec<String>, dyn Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = DynamoClient::new(&shared_config);
    let paginator = client.list_tables().into_paginator().items().send();
    let table_names = paginator.collect::<Result<Vec<String>, dyn Error>>().await?;
    Ok(table_names)
}

// fn get_cf_domain() -> String {
//     return match env::var("CF_DOMAIN") {
//         Ok(v) => v,
//         Err(e) => panic!("$CF_DOMAIN is not set ({})", e)
//     }
// }

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

                    list_repos().await;

                    for table in list_tables().await? {
                        println!("  {}", table);
                    }
                }
                None => println!("Nothing"),
                Some((&_, _)) => todo!()
            }
        }
    }
}
