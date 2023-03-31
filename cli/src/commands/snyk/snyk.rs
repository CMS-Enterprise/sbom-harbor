use std::collections::HashMap;
use std::{fs, env};
use async_trait::async_trait;
use serde_json::Value;
use crate::commands::Provider;
use crate::commands::snyk::snyk_data_model::{SnykData, ProjectJson, ProjectDetails};
use platform::hyper::{ContentType, Error as HyperError, get, post};
use platform::auth::get_secret;
use harbor_client::client::simple_upload_sbom;
use anyhow::Result;
//DONE: Add method call
//DONE: See if it runs
//DONE: Pull in HTTP library
//DONE: Get org count from snyk repo
//DONE: Get projects for each org
//DONE: Get SBOMS for each project
//DONE: Save SBOMS to temp location
//DONE: Restructure and cleanup pass 1
//DONE: Merge Quinns code
//DONE: Restructure and cleanup pass 2
//DONE: Rebase main
//TODO: Event logging
//TODO: Send SBOM somewhere and review solution works...
//TODO: Documentation
//TODO: Final Restructure and cleanup pass

pub struct SnykProvider {}

impl SnykProvider {

    //Origins that can have associated SBOM data in Snyk
    const VALID_ORIGINS: &'static [&'static str] = &["cli", "github", "github-enterprise", "gitlab"];
    //Types that can have associated SBOM data in Snyk
    const VALID_TYPES: &'static [&'static str] = &["npm", "nuget", "gradle", "hex", "pip", "poetry", "rubygems", 
    "maven", "yarn", "yarn-workspace", "composer", "gomodules", "govendor", "golang", "golangdep", "gradle", "paket", "cocoapods", "cpp", "sbt"];
    //URL used for getting org and project details
    const API_V1_URL: &'static str = "https://snyk.io/api/v1";
    //URL used for getting SBOMS
    const API_V3_URL: &'static str = "https://api.snyk.io/rest";
    const API_V3_VERSION: &'static str = "2023-02-15~beta";
    //Format of SBOMs returned from Snyk
    const SBOM_FORMAT: &'static str = "cyclonedx%2Bjson";
    const AWS_SECRET_NAME: &'static str = "dev-harbor-snyk-token-use1";

    //TODO: delete this
    const TEMP_TEST_DIR: &'static str = "/home/jshattjr";

    // Retrieves a list of all Orgs from Snyk and adds them into a new SnykData object
    async fn get_orgs(snyk_token: &str) -> SnykData{
        println!("Getting list of Orgs from Snyk...");
        let url = format!("{}/orgs", Self::API_V1_URL);

        let response: Result<Option<SnykData>, HyperError> = get(
            url.as_str(),
            ContentType::Json,
            snyk_token,
            None::<String>,
        ).await;
    
        match response {
            Ok(option) => 
                match option {
                Some(value) => {return value},
                None => panic!("empty data"),
            },
            Err(err) => panic!("Error in the response: {}", err),
        }
    }

    //Iterates over the list of Orgs in a passed SnykData object 
    //Returns the a filtered list of projects from all orgs
    async fn add_projects_to_orgs(mut snyk_data: SnykData, snyk_token: &str) -> Vec<ProjectDetails>{
        println!("Adding projects to each Org....");

        let mut valid_projects: Vec<ProjectDetails> = Vec::new();
        
        for item in snyk_data.orgs.iter_mut(){
            match item {
                Some(org) => {
                    let org_id = org.id.clone().unwrap_or_else(|| "Missing".to_string());
                    let org_name = org.name.clone().unwrap_or_else(|| "Missing".to_string());

                    println!("Retrieving project info for Org: {}, with ID: {}", org_name, org_id);
                    let url = format!("{}/org/{}/projects", Self::API_V1_URL, org_id);
                    let response: Result<Option<ProjectJson>, HyperError> = post(
                        url.as_str(),
                        ContentType::Json,
                        snyk_token,
                        None::<String>,
                    ).await;
            
                    match response {
                        Ok(option) =>  {
                            match option {
                                Some(project_json) => Self::find_valid_projects(project_json, org_id, org_name.clone(), &mut valid_projects),
                                None => todo!(),
                            }
                        },
                        Err(err) => panic!("Error in the response: {}", err),
                    }
                },
                None => todo!(),
            }
        }
        return valid_projects;
    }

    // Iterates over list of projects and creates a new project list that only has projects with a valid Origin and Type
    fn find_valid_projects(project_list: ProjectJson, org_id: String, org_name:String, valid_projects: &mut Vec<ProjectDetails>) {

        for project in project_list.projects.iter() {
            if(!project.id.is_empty()  && !project.origin.is_empty() && !project.r#type.is_empty()) {
                if  (Self::VALID_ORIGINS.contains(&project.origin.as_str()) &&  Self::VALID_TYPES.contains(&project.r#type.as_str())){
                    let sbom_url = format!("{}/orgs/{}/projects/{}/sbom?version={}&format={}", Self::API_V3_URL, org_id.clone(), project.id, Self::API_V3_VERSION, Self::SBOM_FORMAT);

                    let valid_project = ProjectDetails{
                        org_id: org_id.clone(),
                        org_name: org_name.clone(),
                        id: project.id.clone(),
                        name: project.name.clone(),
                        origin: project.origin.clone(),
                        r#type: project.r#type.clone(),
                        browseUrl: project.browseUrl.clone(),
                        sbom_url: sbom_url
                    };

                    valid_projects.push(valid_project);
                }
            }
        }
    }

    async fn publish_sboms(valid_projects: &Vec<ProjectDetails>, snyk_token: &str) {

        //TODO: delete this
        let tmp_test_dir = Self::TEMP_TEST_DIR;
        
        println!("Publishing SBOMS...");

        for project in valid_projects.iter() {
            let proj_name = project.name.clone().replace("/", "-");
            println!("Getting Sbom for project: {}",proj_name);

            let response: Result<Option<HashMap<String, Value>>, HyperError> = get(
                project.sbom_url.clone().as_str(),
                ContentType::Json,
                snyk_token,
                None::<String>,
            ).await;

            match response {
                Ok(option) =>  {
                    match option {
                        Some(sbom) => {
                            //TODO: review this with Quinn
                            let cloud_front_domain = env::var("CF_DOMAIN").unwrap_or(String::from(""));
                            let sbom_token = env::var("V1_CMS_TEAM_TOKEN").unwrap_or(String::from(""));
                            let team_id = env::var("V1_CMS_TEAM_ID").unwrap_or(String::from(""));

                            simple_upload_sbom(cloud_front_domain, sbom_token, team_id, project.browseUrl.clone(), project.r#type.clone(), sbom).await;
                        },
                        None => todo!(),
                    }
                },
                Err(err) => {
                    println!("ERROR -> Failed to find sbom for project: {} in org: {}", proj_name, project.org_id)
                },
            }
        }
    }

}

#[async_trait]
impl Provider for SnykProvider {

    async fn scan(&self) {
        println!("Scanning Snyk for SBOMS...");
        let response = get_secret(Self::AWS_SECRET_NAME).await;
        let snyk_token = match response {
            Ok(secret) => {
                match secret {
                    Some(s) => s,
                    None => panic!("No AWS token retrieved for secret: {}", Self::AWS_SECRET_NAME),
                }
            },
            Err(err) => panic!("Failed to retrieve token for secret: {}, with error: {}", Self::AWS_SECRET_NAME, err),
        };

        println!("{}", snyk_token);

        let snyk_data = SnykProvider::get_orgs(snyk_token.as_str()).await;

        let valid_projects = SnykProvider::add_projects_to_orgs(snyk_data, snyk_token.as_str()).await;

        let data = format!("{:#?}", valid_projects); //TODO: Remove this when done debugging
        let output_path = format!("{}/snyk_data.json", Self::TEMP_TEST_DIR);
        fs::write(output_path, data).expect("Unable to write file"); //TODO: Remove this when done debugging

        SnykProvider::publish_sboms(&valid_projects, snyk_token.as_str()).await;
    }
}

#[tokio::test]
async fn test_get_snyk_data() {
    let provider = SnykProvider{};
    provider.scan().await;
}