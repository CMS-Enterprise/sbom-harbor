use std::env;

use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result as AnyhowResult};
use async_trait::async_trait;
use crate::commands::Provider;

use crate::http::{ContentType, get};

fn get_gh_token() -> String {
    return match env::var("GH_FETCH_TOKEN") {
        Ok(v) => v,
        Err(e) => panic!("$GH_FETCH_TOKEN is not set ({})", e),
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    name: Option<String>,
    ssh_url: Option<String>,
    language: Option<String>,
    archived: Option<bool>,
    disabled: Option<bool>,
}

pub struct GitHubProvider {}

impl GitHubProvider {

    async fn get_repos() -> AnyhowResult<Vec<Repo>> {
        // TODO We must go through all the pages to get all the repos!

        let token: String = String::from("Bearer ") + &get_gh_token();
        let org_name: &str = "cmsgov";
        let github_org_url: String =
            format!("https://api.github.com/orgs/{org_name}/repos?type=sources&per_page=100");

        let response: AnyhowResult<Option<Vec<Repo>>> = get(
            github_org_url.as_str(),
            ContentType::Json,
            token.as_str(),
            None::<String>,
        ).await;

        match response {
            Ok(option) => match option {
                Some(value) => return Ok(value),
                None => panic!("Nothing in here!"),
            },
            Err(err) => panic!("Error in the response: {}", err),
        }
    }
}

#[async_trait]
impl Provider for GitHubProvider {

    async fn scan(&self) {

        println!("Scanning GitHub...");

        let repos_result = GitHubProvider::get_repos().await;

        let repos = match repos_result {
            Ok(value) => value,
            Err(err) => panic!("Panic trying to extract value from Result: {}", err),
        };

        for repo in repos.iter() {
            match &repo.ssh_url {
                Some(url) => println!("New Repo URL Location, value: {}", &url),
                None => panic!("No URL for this one I guess"),
            }
        }
    }
}