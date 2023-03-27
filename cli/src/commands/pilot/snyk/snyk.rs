use std::env::var;
use std::{fs};
use anyhow::{Result as AnyhowResult};
use async_trait::async_trait;
use crate::commands::Provider;
use crate::commands::pilot::snyk::snyk_data_model::{SnykData, ProjectList, ProjectDetails, Sbom};
use platform::hyper::{ContentType, Error as HyperError, get, post};
use platform::auth::get_secret;
//DONE: Add method call
//DONE: See if it runs
//DONE: Pull in HTTP library
//DONE: Get org count from snyk repo
//DONE: Get projects for each org
//DONE: Get SBOMS for each project
//DONE: Save SBOMS to temp location
//DONE: Restructure and cleanup pass 1
//DONE: Merge Quinns code
//TODO: Event logging
//TODO: Send SBOM somewhere?
//TODO: Documentation
//TODO: Restructure and cleanup pass 2

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

    fn get_snyk_access_token() -> String {
        let buts = get_secret("dev-harbor-snyk-token-use1");
        let snyk_token = "SnykToken";
        
        match var(snyk_token) {
            Ok(token) => return token,
            Err(_) => panic!("Environment variable {} is not set", snyk_token),
        }
    }
    
    // Retrieves a list of all Orgs from Snyk and adds them into a new SnykData object
    async fn get_orgs() -> SnykData{
        println!("Getting list of Orgs from Snyk...");
        let url = format!("{}/orgs", Self::API_V1_URL);

        let response: Result<Option<SnykData>, HyperError> = get(
            url.as_str(),
            ContentType::Json,
            Self::get_snyk_access_token().as_str(),
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
    //Returns the SnykData object with each Org updated with a list of valid projects
    async fn add_projects_to_orgs(mut snyk_data: SnykData) -> SnykData{
        println!("Adding projects to each Org....");

        for item in snyk_data.orgs.iter_mut(){
            match item {
                Some(org) => {
                    let org_id = org.id.clone().unwrap_or_else(|| "Missing".to_string());
                    let org_name = org.name.clone().unwrap_or_else(|| "Missing".to_string());
                    let project = Self::get_project_list_from_snyk(org_id, org_name).await;
                    org.add_project(project);
                },
                None => todo!(),
            }
        }

        return snyk_data;
    }

    // Gets a list of projects and their info for a specified org from Snyk
    async fn get_project_list_from_snyk(org_id: String, org_name: String) -> Option<ProjectList>{
        println!("Retrieving project info for Org: {}, with ID: {}", org_name, org_id);
        let url = format!("{}/org/{}/projects", Self::API_V1_URL, org_id);

        let response: Result<Option<ProjectList>, HyperError> = post(
            url.as_str(),
            ContentType::Json,
            Self::get_snyk_access_token().as_str(),
            None::<String>,
        ).await;

        match response {
            Ok(option) =>  return Self::remove_invalid_projects(option, org_id),
            Err(err) => panic!("Error in the response: {}", err),
        }
    }

    // Iterates over list of projects and creates a new project list that only has projects with a valid Origin and Type
    fn remove_invalid_projects(project_list: Option<ProjectList>, org_id: String) -> Option<ProjectList>{

        let mut valid_projects: Vec<Option<ProjectDetails>> = Vec::new();

        match project_list {
            Some(list) => {
                list.projects.iter().for_each(|item|{
                    match item {
                        Some(project_details) => {
                            //TODO: there some better way to compare all of these?
                            let proj_id = project_details.id.clone().unwrap_or_else(||"".to_string());
                            let proj_name = project_details.name.clone().unwrap_or_else(||"".to_string());
                            let proj_origin = project_details.origin.clone().unwrap_or_else(||"".to_string());
                            let proj_type = project_details.r#type.clone().unwrap_or_else(||"".to_string());
                            
                            if(!proj_id.is_empty()  && !proj_origin.is_empty() && !proj_type.is_empty()) {
                                if  (Self::VALID_ORIGINS.contains(&proj_origin.as_str()) &&  Self::VALID_TYPES.contains(&proj_type.as_str())){
                                    let sbom_url = format!("{}/orgs/{}/projects/{}/sbom?version={}&format={}", Self::API_V3_URL, org_id, proj_id, Self::API_V3_VERSION, Self::SBOM_FORMAT);

                                    let valid_project = ProjectDetails{
                                        id: Some(proj_id),
                                        name: Some(proj_name),
                                        origin: Some(proj_origin),
                                        r#type: Some(proj_type),
                                        sbom_url: Some(sbom_url)
                                    };

                                    valid_projects.push(Some(valid_project));
                                }
                            }
                        },
                        None => {},
                    }
                })
            },
            None => todo!(),
        }

        let updated_project_list: ProjectList = ProjectList { projects: (valid_projects) };
        
        return Some(updated_project_list);
    }

    async fn publish_sboms(snyk_data: &SnykData) {
        
        println!("Publishing SBOMS...");

        for org in snyk_data.orgs.iter() {
            let current_org = org.as_ref().unwrap();
            let current_org_id = org.as_ref().unwrap().id.as_ref().unwrap();
            for project in current_org.projects.as_ref().unwrap().projects.iter() {
                match project {
                    Some(project_details) => {
                        let proj_name = project_details.name.clone().unwrap().replace("/", "-");
                        println!("Building Sbom for project: {}",proj_name.as_str());

                        let response: Result<Option<Sbom>, HyperError> = get(
                            project_details.sbom_url.clone().unwrap().as_str(),
                            ContentType::Json,
                            Self::get_snyk_access_token().as_str(),
                            None::<String>,
                        ).await;
                
                        match response {
                            Ok(option) =>  {
                                match option {
                                    Some(sbom) => {
                                        //TODO: send sboms somewhere
                                        let data = format!("{:#?}", sbom);
                                        let file_path = format!("/home/jshattjr/sboms/project-{}", proj_name.as_str());
                                        println!("Writing file to location: {}", file_path.as_str());
                                        fs::write(file_path, data).expect("Unable to write file");
                                    },
                                    None => todo!(),
                                }
                            },
                            Err(err) => {
                                println!("ERROR -> Failed to find sbom for project: {} in org: {}", proj_name.as_str(), current_org_id.as_str())
                            },
                        }
                    },
                    None => todo!(),
                }
            }
        }
    }

}

#[async_trait]
impl Provider for SnykProvider {

    async fn scan(&self) {
        println!("Scanning Snyk for SBOMS...");

        let snyk_data = SnykProvider::get_orgs().await;

        let snyk_data = SnykProvider::add_projects_to_orgs(snyk_data).await;

        let data = format!("{:#?}", snyk_data); //TODO: Remove this when done debugging
        fs::write("/home/jshattjr/tmp_output.json", data).expect("Unable to write file"); //TODO: Remove this when done debugging

        SnykProvider::publish_sboms(&snyk_data).await;
    }
}

#[tokio::test]
async fn test_get_snyk_data() {
    let response = get_secret("dev-harbor-snyk-token-use1").await;
    match response {
        Ok(secret) => {
            match secret {
                Some(s) => println!("{}",s),
                None => panic!("something went wrong?"),
            }
        },
        Err(err) => println!("{}",err),
    }
    // match response {

    // }
    // let provider = SnykProvider{};
    // provider.scan().await;
}