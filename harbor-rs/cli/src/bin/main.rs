use std::any::Any;
use std::env;
use std::io;
use std::process::{Command as SysCommand, Output};
use std::result::Result;
use std::time::Duration;

use clap::{Arg, ArgAction, ArgMatches, Command};
use octocrab::params;
use serde::{Deserialize, Serialize};

use aqum::dynamo::{default_key_condition_expression, Entity, KeyContext, Schema};
use aqum::Error as AqumError;
use aws_config::environment::EnvironmentVariableRegionProvider;
use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    TableStatus,
};

pub async fn wait_for_ready_table(client: &DynamoClient) {
    let schema = pilot_table_schema();

    loop {
        if let Some(table) = client
            .describe_table()
            .table_name(schema.table.clone())
            .send()
            .await
            .expect("success")
            .table()
        {
            if !matches!(table.table_status, Some(TableStatus::Creating)) {
                break;
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

pub async fn create_table(client: &DynamoClient) -> Result<(), AqumError> {
    let schema = pilot_table_schema();

    let partition_ad = AttributeDefinition::builder()
        .attribute_name(schema.partition_attribute.clone().unwrap())
        .attribute_type(ScalarAttributeType::S)
        .build();

    let sort_ad = AttributeDefinition::builder()
        .attribute_name(schema.sort_attribute.clone().unwrap())
        .attribute_type(ScalarAttributeType::S)
        .build();

    let partition_key_schema = KeySchemaElement::builder()
        .attribute_name(schema.partition_attribute.unwrap())
        .key_type(KeyType::Hash)
        .build();

    let sort_key_schema = KeySchemaElement::builder()
        .attribute_name(schema.sort_attribute.unwrap())
        .key_type(KeyType::Range)
        .build();

    let throughput = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    let create_table_response = client
        .create_table()
        .table_name(schema.table)
        .key_schema(partition_key_schema)
        .key_schema(sort_key_schema)
        .attribute_definitions(partition_ad)
        .attribute_definitions(sort_ad)
        .provisioned_throughput(throughput)
        .send()
        .await;

    match create_table_response {
        Ok(_) => {
            println!("Added table");
            Ok(())
        }
        Err(e) => {
            let e = e.into_service_error();
            eprintln!("Got an error creating table:");
            eprintln!("{}", e.to_string());
            Err(AqumError::Dynamo(e.to_string()))
        }
    }
}

pub async fn config_from_env() -> Result<SdkConfig, AqumError> {
    let region_provider = RegionProviderChain::default_provider()
        .or_else(EnvironmentVariableRegionProvider::new());

    let config = aws_config::from_env()
        .region(region_provider)
        .load()
        .await;

    Ok::<SdkConfig, AqumError>(config)
}

fn pilot_table_schema() -> Schema {
    Schema {
        table: "PilotRuns".to_string(),
        partition_attribute: Some("GitHubUrl".to_string()),
        sort_attribute: Some("TeamId".to_string()),
    }
}

pub async fn ensure_table_exists() -> Result<(), AqumError> {
    let schema = pilot_table_schema();
    let config = config_from_env().await?;
    let client = DynamoClient::new(&config);

    let tables = client
        .list_tables()
        .clone()
        .send()
        .await?;

    let tables = tables.table_names().unwrap().to_vec();

    if !tables.contains(&schema.table) {
        let result = create_table(&client).await;

        if result.is_err() {
            let err = result.err().unwrap();
            if !err.to_string().contains("ResourceInUseException") {
                return Err(err);
            }
        }
    }

    wait_for_ready_table(&client).await;

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PilotRun {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

    #[serde(rename = "CompositeKey")]
    pub sort_key: String,

    /// The name of the item.
    pub name: String,
}

impl Entity for PilotRun {

    // Schema:
    // 1. PartitionKey(GitHubUrl)
    // 2. SortKey(TeamId)
    // 3. ProjectId
    // 4. CodebaseId
    // 5. UploadToken
    // 6. LastCommitHash
    fn schema(&self) -> Schema {
        return pilot_table_schema();
    }

    fn key_condition_expression(&self) -> Option<String> {
        default_key_condition_expression(&self.schema())
    }

    fn partition_key(&self) -> Option<String> {
        Some(self.partition_key.clone())
    }

    fn sort_key(&self) -> Option<String> {
        Some(self.sort_key.clone())
    }

    fn load(&mut self) -> Result<(), AqumError> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn key_context(&self) -> KeyContext {
        KeyContext {
            schema: self.schema(),
            key_condition_expression: self.key_condition_expression(),
            partition_key: self.partition_key(),
            sort_key: self.sort_key(),
        }
    }
}

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

fn get_gh_token() -> String {
    return match env::var("GH_FETCH_TOKEN") {
        Ok(v) => v,
        Err(e) => panic!("$GH_FETCH_TOKEN is not set ({})", e)
    }
}

#[allow(dead_code)]
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
                    let _result = match ensure_table_exists().await {
                        Ok(r) => r,
                        Err(e) => panic!("Error while doing the thing: {}", e),
                    };
                }
                None => println!("Nothing"),
                Some((&_, _)) => todo!()
            }
        }
    }
}
