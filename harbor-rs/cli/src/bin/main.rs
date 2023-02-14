

use clap::{Arg, ArgAction, Command, ArgMatches};
use std::result::Result;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::env;
use octocrab::params;
use aws_sdk_dynamodb::{Client as DynamoClient, Error as DynamoError};
use aws_sdk_dynamodb::output::ListTablesOutput;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::error::ListTablesError;

use aws_config::meta::region::RegionProviderChain;

use harbor_cli::commands::PilotCommand;

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
        Command::new("pilot")
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

// Result<Vec<String>, dyn Error>
// pub async fn list_tables() -> Result<(), dyn Error> {
//     let shared_config = aws_config::load_from_env().await;
//     let client = DynamoClient::new(&shared_config);
//     let result = client.list_tables().send().await;
//
//     let result_value = match result {
//         Ok(v) => v,
//         Err(e) => panic!(e)
//     };
//
//     let table_names = result_value.
//
//     let table_names = match result_value {
//         Some(v) => v,
//         None => panic!("There was NONE when looking for table names")
//     };
//
//     Ok(table_names);
// }

pub async fn list_tables_one() ->  Result<(), aws_sdk_dynamodb::Error> {

    println!("Extracting Config");
    let shared_config = aws_config::load_from_env().await;

    println!("Creating Client");
    let client = DynamoClient::new(&shared_config);

    println!("Creating Request");
    let req = client.list_tables();

    println!("Waiting on Response");
    let resp = req.send().await?;
    println!("Received Response, printing tables:");
    println!("Current DynamoDB tables: {:?}", resp.table_names.unwrap());

    Ok(())
}

async fn list_tables_three() -> Result<(), DynamoError> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = DynamoClient::new(&config);

    // println!("Config: {:#?}", config);

    let resp = client.list_tables().send().await;

    println!("Response: {:#?}", resp);

    let value = match resp {
        Ok(v) => v,
        Err(sdk_err) => panic!("{}", sdk_err)
    };

    println!("Tables:");

    let names = value.table_names().unwrap_or_default();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());

    Ok(())
}

// async fn list_tables_two() -> Result<(), DynamoError> {
//     let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
//     let config = aws_config::from_env().region(region_provider).load().await;
//     let client = DynamoClient::new(&config);
//
//     println!("Listing tables: ");
//
//     let resp = client.list_tables().send().await;
//
//     let value = match resp {
//         Ok(v) => v,
//         Err(sdk_err) => match sdk_err {
//             ConstructionFailure(f) => println!("ConstructionFailure {}", f),
//             TimeoutError(f) => println!("TimeoutError {}", f),
//             DispatchFailure(f) => println!("DispatchFailure {}", f),
//             ResponseError(f) => println!("ResponseError {}", f),
//             ServiceError(f) => println!("ServiceError {}", f),
//         },
//     };
//
//     println!("Tables:");
//
//     let names = value.table_names().unwrap_or_default();
//
//     for name in names {
//         println!("  {}", name);
//     }
//
//     println!();
//     println!("Found {} tables", names.len());
//
//     Ok(())
// }

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

                    // list_repos().await;

                    let result = list_tables_three().await;

                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:#?}", e);
                            return
                        }
                    }

                    // dbg!(val).expect("TODO: panic message");

                    // for table in list_tables().await? {
                    //     println!("  {}", table);
                    // }
                }
                None => println!("Nothing"),
                Some((&_, _)) => todo!()
            }
        }
    }
}
