use std::{fs, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, Number};

use crate::http::{ContentType, get as httpGet, post as httpPost};
use anyhow::{anyhow, Result as AnyhowResult};
use async_trait::async_trait;
use crate::commands::Provider;
//DONE: Add method call
//DONE: See if it runs
//DONE: Pull in HTTP library
//DONE: Get org count from snyk repo
//DONE: Get projects for each org
//DONE: Get SBOMS for each project
//DONE: Save SBOMS to temp location
//TODO: Restructure and cleanup pass 1
//TODO: Event logging
//TODO: Import Quinns updates
//TODO: Send SBOM somewhere?
//TODO: Restructure and cleanup pass 2



#[derive(Debug, Serialize, Deserialize)]
struct SnykData {
    orgs: Vec<Option<Org>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Org {
    id: Option<String>,
    name: Option<String>,
    projects: Option<ProjectList>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ProjectList {
    projects: Vec<Option<ProjectDetails>>
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectDetails {
    id: Option<String>,
    name: Option<String>,
    origin: Option<String>,
    r#type: Option<String>,
    sbom_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sbom {
    #[serde(flatten)]
    inner: HashMap<String, Value>,
}

impl Org {
    fn add_project(&mut self, projects: Option<ProjectList>) {
        self.projects = projects;
    }
}

pub struct SnykProvider {}

//const VALID_ORIGINS: &'static [&'static str] = &["cli", "World", "!"];
const VALID_ORIGINS: &'static [&'static str] = &["cli", "github", "github-enterprise", "gitlab"];

impl SnykProvider {
    async fn get_orgs() -> SnykData{
        println!("Getting list of Orgs from Snyk...");
        let snyk_url: String = String::from("https://snyk.io/api/v1/orgs");
        let token: String = String::from("");

        let response: AnyhowResult<Option<SnykData>> = httpGet(
            snyk_url.as_str(),
            ContentType::Json,
            token.as_str(),
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
        let snyk_url: String = String::from("https://snyk.io/api/v1");
        let url = format!("{}/org/{}/projects", snyk_url, org_id);
        let token: String = String::from("");

        println!("Using URL: {}", url);
        
        let response: AnyhowResult<Option<ProjectList>> = httpPost(
            url.as_str(),
            ContentType::Json,
            token.as_str(),
            None::<String>,
        ).await;

        match response {
            Ok(option) =>  return Self::remove_invalid_projects(option, org_id),
            Err(err) => panic!("Error in the response: {}", err),
        }
    }

    fn remove_invalid_projects(mut project_list: Option<ProjectList>, org_id: String) -> Option<ProjectList>{

        let mut valid_projects: Vec<Option<ProjectDetails>> = Vec::new();
        //let VALID_ORIGINS = vec!["cli".to_string(), "github".to_string(), "github-enterprise".to_string(), "gitlab".to_string()];
        let valid_types = vec![
            "npm".to_string(),
            "nuget".to_string(),
            "gradle".to_string(),
            "hex".to_string(),
            "pip".to_string(),
            "poetry".to_string(),
            "rubygems".to_string(),
            "maven".to_string(),
            "yarn".to_string(),
            "yarn-workspace".to_string(),
            "composer".to_string(),
            "gomodules".to_string(),
            "govendor".to_string(),
            "golang".to_string(),
            "golangdep".to_string(),
            "gradle".to_string(),
            "paket".to_string(),
            "cocoapods".to_string(),
            "cpp".to_string(),
            "sbt".to_string(),
        ];
        let apiV3Url="https://api.snyk.io/rest";
        let apiV3Ver="2023-02-15~beta";
        let sbomFormat="cyclonedx%2Bjson";

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
                                if  (VALID_ORIGINS.contains(&proj_origin.as_str()) &&  valid_types.contains(&proj_type)){
                                    //TODO: create new project detail and add to valid project list
                                    //${apiV3Url}/orgs/${orgId}/projects/${projId}/sbom?version=${apiV3Ver}&format=${sbomFormat}
                                    let sbom_url = format!("{}/orgs/{}/projects/{}/sbom?version={}&format={}", apiV3Url, org_id, proj_id, apiV3Ver, sbomFormat);

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
        let token: String = String::from("");

        for org in snyk_data.orgs.iter() {
            let current_org = org.as_ref().unwrap();
            let current_org_id = org.as_ref().unwrap().id.as_ref().unwrap();
            for project in current_org.projects.as_ref().unwrap().projects.iter() {
                match project {
                    Some(project_details) => {
                        let sbom_url = project_details.sbom_url.clone().unwrap();
                        let proj_name = project_details.name.clone().unwrap().replace("/", "-");
                        println!("Building Sbom for project: {}",proj_name.as_str());
                        break;
                        let response: AnyhowResult<Option<Sbom>> = httpGet(
                            sbom_url.as_str(),
                            ContentType::Json,
                            token.as_str(),
                            None::<String>,
                        ).await;
                
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