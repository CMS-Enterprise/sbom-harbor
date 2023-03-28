use clap::{Arg, ArgAction, Command, ArgMatches};
use std::env;

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

    let _token = get_gh_token();


    // curl -X GET -u <UserName>:<Generated-Token>https://api.github.com/orgs/<Org-Name>/repos | grep -w clone_url

    // let project: Option<Project> =
    //     post(create_project_url.as_str(), token.as_str(), Some(project)).await?;
    // let team: Option<Team> = get(url.as_str(), token.as_str(), None::<Team>).await?;

    // Ok(team.unwrap());
    // let octocrab = octocrab::OctocrabBuilder::new()
    //     .personal_token(token)
    //     .build()
    //     .unwrap();
    //
    // let current_page = octocrab
    //     .orgs("cmsgov")
    //     .list_repos()
    //     .repo_type(params::repos::Type::Sources)
    //     .sort(params::repos::Sort::Pushed)
    //     .direction(params::Direction::Descending)
    //     .per_page(100)
    //     .page(1u32)
    //     .send()
    //     .await;

    // let mut current_page_value = match current_page {
    //     Ok(v) => v,
    //     Err(e) => panic!("Error trying to get page: {}", e)
    // };
    //
    // let prs = current_page_value.take_items();
    //
    // for pr in prs.iter() {
    //     println!("Value: {}", pr.url.as_str());
    // }
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
                }
                None => println!("Nothing"),
                Some((&_, _)) => todo!()
            }
        }
    }
}
