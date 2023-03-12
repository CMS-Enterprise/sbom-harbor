use std::convert::TryFrom;
use std::env;
use std::pin::Pin;

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
struct Org {
    public_repos: Option<u32>
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

    async fn get_num_pub_repos(org: String) -> AnyhowResult<Option<u32>> {
        let token: String = String::from("Bearer ") + &get_gh_token();
        let org_url: String = format!("https://api.github.com/orgs/{org}");

        let response: AnyhowResult<Option<Org>> = get(
            org_url.as_str(),
            ContentType::Json,
            token.as_str(),
            None::<String>,
        ).await;

        match response {
            Ok(option) => match option {
                Some(value) => return Ok(value.public_repos),
                None => panic!("Nothing in here!"),
            },
            Err(err) => panic!("Error in the response: {}", err),
        }
    }

    async fn get_repos(org: String) -> AnyhowResult<Vec<Repo>> {

        let num_repos = match GitHubProvider::get_num_pub_repos(org.clone()).await {
            Ok(option) => match option {
                Some(num) => num,
                None => panic!("No Repos in the cmsgov ORG!!!")
            },
            Err(err) => panic!("Error Attempting to get num Repos: {}", err)
        };

        println!("Number of Repositories in {org}: {num_repos}");

        let num_calls = ((num_repos/100) as i8) + 1;
        let num_last_call = num_repos % 100;
        let mut pages = vec![100; usize::try_from(num_calls).unwrap()];
        *pages.last_mut().unwrap() = num_last_call;

        let token: String = String::from("Bearer ") + &get_gh_token();

        let mut repo_vec: Vec<Repo> = Vec::new();

        for page in pages.iter() {

            let github_org_url: String =
                format!("https://api.github.com/orgs/{org}/repos?type=sources&per_page={page}");

            let response: AnyhowResult<Option<Vec<Repo>>> = get(
                github_org_url.as_str(),
                ContentType::Json,
                token.as_str(),
                None::<String>,
            ).await;

            repo_vec.extend(
                match response {
                    Ok(option) => match option {
                        Some(value) => value,
                        None => panic!("Nothing in here!"),
                    },
                    Err(err) => panic!("Error in the response: {}", err),
                }
            );
        }

        Ok(repo_vec)
    }
}

#[async_trait]
impl Provider for GitHubProvider {

    async fn scan(&self) {

        println!("Scanning GitHub...");

        let org_name: String = String::from("cmsgov");

        let repos_result = GitHubProvider::get_repos(org_name).await;

        let repos: Vec<Repo> = match repos_result {
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

