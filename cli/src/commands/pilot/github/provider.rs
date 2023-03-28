use std::collections::HashMap;
use async_trait::async_trait;
use mongodb::bson::doc;
use uuid::Uuid;
use anyhow::{Result as AnyhowResult};
use mongodb::Collection;
use tokio::count;
use harbcore::services::{clone_path, clone_repo, remove_clone, syft};
use harborclient::client::{Client as V1HarborClient, SBOMUploadResponse};
use platform::hyper::{Error as HyperError, ContentType, get};

use crate::commands::{get_env_var, Provider};
use crate::commands::pilot::github::{
    CODEBASE_ID_KEY,
    COLLECTION,
    PROJECT_ID_KEY,
    TEAM_ID_KEY,
    Counter,
    GhProviderError,
    HarborConfig,
    get_harbor_config,
};
use crate::commands::pilot::github::mongo::{get_mongo_db, GitHubCrawlerMongoDocument, update_last_hash_in_mongo};
use crate::commands::pilot::github::repo::{authize, Commit, get_last_commit_url, get_page_of_repos, get_pages, Repo, should_skip};

/// Definition of the GitHubProvider
///
pub struct GitHubProvider {}

/// GitHubProvider's own implementation
///
impl GitHubProvider {

    async fn get_repos(org: String) -> AnyhowResult<Vec<Repo>> {

        let mut pages = get_pages(&org).await;
        let gh_fetch_token = match get_env_var("GH_FETCH_TOKEN") {
            Some(value) => value,
            None => panic!("Missing GitHub Token. export GH_FETCH_TOKEN=<token>")
        };

        let token: String = String::from("Bearer ") + &gh_fetch_token;
        let mut repo_vec: Vec<Repo> = Vec::new();

        for (page, per_page) in pages.iter_mut().enumerate() {

            let mut gh_org_rsp = get_page_of_repos(&org, (page+1), per_page, &token).await;

            for (repo_num, repo) in gh_org_rsp.iter_mut().enumerate() {

                let github_last_commit_url = get_last_commit_url(repo);

                println!("({}) Making call to {}", repo_num, github_last_commit_url);

                let response: Result<Option<Commit>, HyperError> = get(
                    github_last_commit_url.as_str(),
                    ContentType::Json,
                    token.as_str(),
                    None::<String>,
                ).await;

                let gh_commits_rsp = match response {
                    Ok(option) => match option {
                        Some(value) => value,
                        None => panic!("Nothing in here!"),
                    },
                    Err(err) => {
                        if let HyperError::Remote(status, _msg) = err {

                            if status == 409 {
                                repo.mark_repo_empty();
                            }

                            Commit { sha: Some(String::from("<empty repo>")) }
                        } else {
                            panic!("No matching Error Type: {}", err)
                        }
                    },
                };

                match gh_commits_rsp.sha {
                    Some(val) => repo.add_last_hash(val),
                    None => panic!("No value for commit found!")
                }
            }
            repo_vec.extend(gh_org_rsp);
        }

        Ok(repo_vec)
    }
}

/// The implementation of Provider for
/// GitHub Provider
///
#[async_trait]
impl Provider for GitHubProvider {

    async fn scan(&self) {

        let harbor_config = get_harbor_config().unwrap();

        let mut counter = Counter {
            archived: 0,
            disabled: 0,
            empty: 0,
            processed: 0,
            hash_matched: 0,
            upload_errors: 0,
        };

        println!("Scanning GitHub...");
        let org_name: String = String::from("cmsgov");
        let repos_result = GitHubProvider::get_repos(org_name).await;
        let repos: Vec<Repo> = match repos_result {
            Ok(value) => value,
            Err(err) => panic!("Panic trying to extract value from Result: {}", err),
        };

        for repo in repos.iter() {

            let name = repo.full_name.clone().unwrap();
            let url = repo.ssh_url.clone().unwrap();

            if !should_skip(repo, name, url, &mut counter) {
                process_repo(repo, &harbor_config, &mut counter).await;
            }
        }

        println!("Collection Run Complete: {:#?}", counter);
    }
}

/// Create the entities like project and codebase
/// in Harbor V1.
///
async fn create_harbor_entities(
    github_url: String,
    harbor_config: &HarborConfig,
    language: String,
) -> Result<HashMap<String, String>, GhProviderError> {

    let client = V1HarborClient::new(
        harbor_config.cf_domain.clone(),
        harbor_config.cognito_username.clone(),
        harbor_config.cognito_password.clone(),
    ).await.unwrap();

    let result = client.create_project(
        harbor_config.cms_team_id.clone(),
        github_url,
        language,
    ).await;

    let project = match result {
        Ok(project) => project,
        Err(err) => {
            let err_msg = format!("Error trying to create a project: {}", err);
            return Err(GhProviderError::EntityCreation(err_msg))
        }
    };

    // Each project will have only one codebase in this implementation
    let codebase = project.codebases.into_iter().nth(0);

    let mut test_map = HashMap::new();

    test_map.insert(
        String::from(TEAM_ID_KEY),
        harbor_config.cms_team_id.to_string(),
    );

    test_map.insert(
        String::from(PROJECT_ID_KEY),
        project.id
    );

    test_map.insert(
        String::from(CODEBASE_ID_KEY),
        codebase.unwrap().id
    );

    Ok(test_map)
}

/// Sends the data in the document to Harbor
///
async fn send_to_pilot(
    document: &GitHubCrawlerMongoDocument,
    harbor_config: &HarborConfig,
) -> Result<SBOMUploadResponse, GhProviderError> {

    /// Clones a repo, generates an SBOM, and then uploads to the Enrichment Engine.

    let clone_path = clone_path(&document.repo_url);
    let authed_url = match authize(&document.repo_url) {
        Ok(authed_url) => authed_url,
        Err(err) => panic!("Unable to fix the URL: {}", err)
    };

    match clone_repo(&clone_path, authed_url.as_str()) {
        Ok(()) => println!("{} Cloned Successfully", document.repo_url),
        Err(err) => panic!("Unable to clone Repo {}, {}", document.repo_url, err)
    }

    let syft_result = match syft(&clone_path) {
        Ok(map) => map,
        Err(err) => panic!("Unable to Syft the Repo we cloned [{}]??", err)
    };

    match remove_clone(&clone_path) {
        Ok(()) => println!("Clone removed successfully"),
        Err(err) => panic!("Unable to blow away the clone [{}]??", err)
    };

    match V1HarborClient::upload_sbom(
        harbor_config.cf_domain.as_str(),
        harbor_config.cms_team_token.as_str(),
        harbor_config.cms_team_id.clone(),
        document.project_id.clone(),
        document.codebase_id.clone(),
        syft_result,
    ).await {
        Ok(response) => Ok(response),
        Err(err) => return Err(
            GhProviderError::Pilot(
                String::from(format!("Pilot Error: {}", err))
            )
        )
    }
}

/// Given the Mongo Collection we want to put a document into
/// and the Harbor Entities, put this document in Mongo
///
async fn create_document_in_db(
    entities: HashMap<String, String>,
    collection: Collection<GitHubCrawlerMongoDocument>,
    url: String,
    last_hash: String,
) -> Result<GitHubCrawlerMongoDocument, GhProviderError> {

    let document = GitHubCrawlerMongoDocument {
        id: Uuid::new_v4().to_string(),
        repo_url: url.to_string(),
        last_hash: last_hash.to_string(),
        team_id: entities.get(TEAM_ID_KEY).unwrap().to_string(),
        project_id: entities.get(PROJECT_ID_KEY).unwrap().to_string(),
        codebase_id: entities.get(CODEBASE_ID_KEY).unwrap().to_string(),
    };

    match collection.insert_one(document.clone(), None).await {
        Ok(result) => result,
        Err(err) => return Err(
            GhProviderError::MongoDb(
                format!("Error inserting into mongo: {}", err)
            )
        ),
    };

    println!("PROCESSING> Added NEW Document to MongoDB: {:#?}", document);

    Ok(document)
}

async fn process_repo(repo: &Repo, harbor_config: &HarborConfig, counter: &mut Counter) {

    let url: &String = match &repo.ssh_url {
        Some(url) => url,
        None => panic!("No URL for Repository"),
    };

    let repo_name: &String = match &repo.full_name {
        Some(name) => name,
        None => panic!("No Full Name for Repository"),
    };

    let last_hash: &String = &repo.last_hash;

    println!("PROCESSING> Will be processing {}@{}", repo_name, url);

    let db = match get_mongo_db().await {
        Ok(db) => db,
        Err(err) => panic!("Problem getting DB: {}", err)
    };

    let collection = db.collection::<GitHubCrawlerMongoDocument>(COLLECTION);
    let filter = doc! { "repo_url":  url.as_str() };

    println!("PROCESSING> Looking in Mongo for this document: {:#?}", filter);

    let mut doc_option: Option<GitHubCrawlerMongoDocument> = match collection.find_one(filter, None).await {
        Ok(cursor) => cursor,
        Err(cursor_err) => panic!("Cursor - Error: {}", cursor_err)
    };

    match doc_option {
        Some(document) => {

            println!("PROCESSING> Found document in mongo, comparing hashes");

            // This arm is executed when something is in the database
            // with the specified repo_url.

            if last_hash.to_string() != document.last_hash {

                println!("PROCESSING> Hashes are not equal, sending to pilot");

                // Use the document to construct a request to Pilot
                match send_to_pilot(&document, &harbor_config).await {
                    Ok(upload_resp) => {

                        // Upload is OK, update Mongo

                        println!("PROCESSING> One SBOM Down! {:#?}", upload_resp);
                        update_last_hash_in_mongo(document, collection, last_hash.clone());
                        counter.processed += 1;
                    },
                    Err(err) => {
                        counter.upload_errors += 1;
                        println!("Error Uploading SBOM!! {}", err)
                    }
                }
            } else {
                counter.hash_matched += 1;
                println!("PROCESSING> Hashes are equal, skipping pilot");
            }
        },
        None => { // Nothing is in Mongo

            println!("PROCESSING> No Document found document in mongo. Creating entities in harbor");

            // This arm executes when nothing is found in the database associated
            // to the given repo_url.  This means we need to create the project and codebase
            // in Harbor before we can send SBOMs to that target.

            let entities: HashMap<String, String> = match create_harbor_entities(
                url.clone(),
                harbor_config,
                match &repo.language {
                    Some(language) => language.to_string(),
                    None => String::from("None"),
                },
            ).await {
                Ok(entities) => entities,
                Err(err) => panic!("Unable to create Harbor entities, {}", err),
            };

            let document: GitHubCrawlerMongoDocument
                = match create_document_in_db(
                entities,
                collection,
                url.clone(),
                last_hash.clone()
            ).await {
                Ok(document) => document,
                Err(err) => panic!("Mongo Error: {}", err)
            };

            // Use the document to construct a request to Pilot
            match send_to_pilot(&document, &harbor_config).await {
                Ok(upload_resp) => {
                    println!("One SBOM Down! {:#?}", upload_resp);
                    counter.processed += 1;
                },
                Err(err) => println!("PROCESSING> Error Uploading SBOM!! {}", err)
            }
        }
    };
}
