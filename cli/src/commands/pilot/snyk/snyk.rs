use std::{fs};
use anyhow::{Result as AnyhowResult};
use async_trait::async_trait;
use crate::commands::Provider;
use crate::commands::pilot::snyk::snyk_data_model::{SnykData, ProjectList, ProjectDetails, Sbom};
use crate::commands::pilot::snyk::snyk_http_helper;

//DONE: Add method call
//DONE: See if it runs
//DONE: Pull in HTTP library
//DONE: Get org count from snyk repo
//DONE: Get projects for each org
//DONE: Get SBOMS for each project
//DONE: Save SBOMS to temp location
//DONE: Restructure and cleanup pass 1
//TODO: Merge Quinns code
//TODO: Event logging
//TODO: Send SBOM somewhere?
//TODO: Documentation
//TODO: Restructure and cleanup pass 2

pub struct SnykProvider {}

impl SnykProvider {

    const VALID_ORIGINS: &'static [&'static str] = &["cli", "github", "github-enterprise", "gitlab"];
    const VALID_TYPES: &'static [&'static str] = &["npm", "nuget", "gradle", "hex", "pip", "poetry", "rubygems", 
    "maven", "yarn", "yarn-workspace", "composer", "gomodules", "govendor", "golang", "golangdep", "gradle", "paket", "cocoapods", "cpp", "sbt"];
    const API_V1_URL: &'static str = "https://snyk.io/api/v1";
    const API_V3_URL: &'static str = "https://api.snyk.io/rest";
    const API_V3_VERSION: &'static str = "2023-02-15~beta";
    const SBOM_FORMAT: &'static str = "cyclonedx%2Bjson";

    async fn get_orgs() -> SnykData{
        println!("Getting list of Orgs from Snyk...");

        let response = snyk_http_helper(format!("{}/orgs", Self::API_V1_URL), "GET".to_string()).await;

        //TODO: remove below whenever I can test and verify generics work
        // let response: AnyhowResult<Option<SnykData>> = httpGet(
        //     &format!("{}/orgs", API_V1_URL),
        //     ContentType::Json,
        //     token.as_str(),
        //     None::<String>,
        // ).await;
    
        match response {
            Ok(option) => 
                match option {
                Some(value) => {return value},
                None => panic!("empty data"),
            },
            Err(err) => panic!("Error in the response: {}", err),
        }
    }

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

    async fn get_project_list_from_snyk(org_id: String, org_name: String) -> Option<ProjectList>{
        println!("Retrieving project info for Org: {}, with ID: {}", org_name, org_id);

        let response = snyk_http_helper(format!("{}/org/{}/projects", Self::API_V1_URL, org_id), "POST".to_string()).await;
        //TODO: remove below whenever I can test and verify generics work
        // let response: AnyhowResult<Option<ProjectList>> = httpPost(
        //     url.as_str(),
        //     ContentType::Json,
        //     token.as_str(),
        //     None::<String>,
        // ).await;

        match response {
            Ok(option) =>  return Self::remove_invalid_projects(option, org_id),
            Err(err) => panic!("Error in the response: {}", err),
        }
    }

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
        //TODO: send sboms somewhere
        println!("Publishing SBOMS...");

        for org in snyk_data.orgs.iter() {
            let current_org = org.as_ref().unwrap();
            let current_org_id = org.as_ref().unwrap().id.as_ref().unwrap();
            for project in current_org.projects.as_ref().unwrap().projects.iter() {
                match project {
                    Some(project_details) => {
                        let proj_name = project_details.name.clone().unwrap().replace("/", "-");
                        println!("Building Sbom for project: {}",proj_name.as_str());

                        let response: AnyhowResult<Option<Sbom>> = snyk_http_helper(project_details.sbom_url.clone().unwrap(), "GET".to_string()).await;
                        //TODO: remove below whenever I can test and verify generics work
                        // let response: AnyhowResult<Option<Sbom>> = httpGet(
                        //     sbom_url.as_str(),
                        //     ContentType::Json,
                        //     token.as_str(),
                        //     None::<String>,
                        // ).await;
                
                        match response {
                            Ok(option) =>  {
                                match option {
                                    Some(sbom) => {
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

    let provider = SnykProvider{};
    provider.scan().await;
}