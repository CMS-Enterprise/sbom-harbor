use std::collections::HashMap;
use std::fmt::Result as StdResult;
use std::fmt::{Display, Formatter};

use anyhow::{anyhow, bail, Result};
use aqum::hyper::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use crate::entities::{Project, Team};

fn join_url(base: &str, route: &str) -> String {
    let mut url = base.to_owned();
    url.push_str(route);
    url
}

// This string will be owned by the client instance.
fn base_url(cloud_front_domain: &str) -> String {
    join_url(cloud_front_domain, "/api/v1")
}

fn login_url(base: &str) -> String {
    join_url(base, "/login")
}

async fn login(base: &str, username: String, password: String) -> Result<String> {
    let url = login_url(base);

    let response: Option<LoginResponse> =
        post(url.as_str(), "", Some(LoginRequest { username, password })).await?;

    let token = response.unwrap().token;
    Ok(token)
}

/// A Harbor API Client.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    token: String,
}

impl Client {
    /// Constructor function that authenticates on instantiation, or fails.
    pub async fn new(
        cloud_front_domain: String,
        username: String,
        password: String,
    ) -> Result<Client> {
        let base_url = base_url(&cloud_front_domain);
        let token = login(&base_url, username.clone(), password).await;

        match token {
            Ok(t) => Ok(Client { base_url, token: t }),
            Err(err) => {
                bail!("error logging in: {}", err);
            }
        }
    }

    fn create_team_url(&self) -> String {
        join_url(&self.base_url, "/team?children=true")
    }

    fn get_team_url(&self, id: String) -> String {
        join_url(&self.base_url, &format!("/team/{}?children=true", id))
    }

    fn delete_team_url(&self, id: String) -> String {
        join_url(&self.base_url, &format!("/team/{}?children=true", id))
    }

    fn get_teams_url(&self) -> String {
        join_url(&self.base_url, "/teams?children=true")
    }

    /// Deletes a team by id.
    pub async fn delete_team(&self, id: String) -> Result<()> {
        let url = self.delete_team_url(id);

        let _: Option<Team> = delete(url.as_str(), &self.token, None::<Team>).await?;

        Ok(())
    }

    // TODO: This a convenience method used by tests and the importer.
    // Assess if it can be moved be moved elsewhere.
    /// Gets a team by id if specified, else gets or creates a team by name.
    pub async fn get_or_create_team(&self, id: String, name: String) -> Result<Team> {
        if id.is_empty() && name.is_empty() {
            bail!("either id or org must be specified")
        }

        if id.is_empty() {
            return self.get_or_create_team_by_name(name).await;
        }

        self.get_team(id).await
    }

    /// Gets a team by id.
    pub async fn get_team(&self, id: String) -> Result<Team> {
        let url = self.get_team_url(id);

        let team: Option<Team> = get(url.as_str(), &self.token, None::<Team>).await?;

        Ok(team.unwrap())
    }

    async fn get_or_create_team_by_name(&self, name: String) -> Result<Team> {
        let get_teams_url = self.get_teams_url();

        let teams: Option<Vec<Team>> =
            get(get_teams_url.as_str(), &self.token, None::<Vec<Team>>).await?;

        let team = teams.unwrap().into_iter().find(|t| t.name == name);

        if team.is_some() {
            return team.ok_or_else(|| anyhow!("error iterating team by name"));
        }

        let create_team_url = self.create_team_url();
        let team = Team::new(name);
        let team: Option<Team> =
            post(create_team_url.as_str(), &self.token, Some(team)).await?;

        let team = team.unwrap();
        if team.tokens.is_empty() {
            bail!("unexpected create team response: tokens expected")
        }

        Ok(team)
    }

    fn create_project_url(&self, team_id: String) -> String {
        join_url(
            &self.base_url,
            &format!("/project?teamId={}&children=true", team_id),
        )
    }

    /// Creates a project with a single codebase child.
    pub async fn create_project_with_codebase(
        &self,
        team_id: String,
        project_name: &str,
        codebase_name: &str,
    ) -> Result<Project> {
        // TODO: Find a way to get the build tool from the GitHub API.
        let codebase = crate::api::Codebase {
            id: "".to_string(),
            name: codebase_name.to_string(),
            language: "".to_string(),
            build_tool: "".to_string(),
            clone_url: "".to_string(),
        };

        let project = crate::api::Project{
            id: "".to_string(),
            name: project_name.to_string(),
            fisma: "".to_string(),
            codebases: vec![codebase],
        };


        let create_project_url = self.create_project_url(team_id);

        let project: Option<Project> =
            post(create_project_url.as_str(), &self.token, Some(project)).await?;

        let project = project.unwrap();

        if project.id.is_empty() {
            bail!("error creating project: invalid project id");
        }

        if project.codebases.is_empty() {
            bail!("error creating project: expected codebases");
        }

        let project_name = &project.name;
        let project_id = &project.id.to_string();
        info!(
            "{}",
            format!("project created: name {} - id {}", project_name, project_id)
        );

        Ok(project)
    }

    fn create_upload_url(
        cloud_front_domain: &str,
        team_id: String,
        project_id: String,
        codebase_id: String,
    ) -> String {
        join_url(
            cloud_front_domain,
            &format!("/api/v1/{}/{}/{}/sbom", team_id, project_id, codebase_id),
        ) // /{teamID}/{projectID}/{codebaseID}/sbom
    }

    /// Uploads an SBOM to the Enrichment Engine.
    pub async fn upload_sbom(
        cloud_front_domain: &str,
        sbom_token: &str,
        team_id: String,
        project_id: String,
        codebase_id: String,
        sbom: HashMap<String, Value>,
    ) -> Result<SBOMUploadResponse> {
        let url = Client::create_upload_url(cloud_front_domain, team_id, project_id, codebase_id);

        let response: Option<SBOMUploadResponse> =
            post(url.as_str(), sbom_token, Some(sbom)).await?;

        Ok(response.unwrap())
    }
}

#[derive(Serialize)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    pub token: String,
}

/// Metadata returned by the Enrichment Engine indicating the status of the upload request.
#[derive(Debug, Deserialize, Serialize)]
pub struct SBOMUploadResponse {
    /// Flag indicating whether the request was considered valid.
    pub valid: bool,
    /// The S3 bucket name the SBOM was uploaded to.
    #[serde(rename = "s3BucketName")]
    pub bucket_name: String,
    /// The S3 object key for the uploaded SBOM.
    #[serde(rename = "s3ObjectKey")]
    pub object_key: String,
}

impl Display for SBOMUploadResponse {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> StdResult {
        write!(
            f,
            "valid: {}, bucket_name: {}, object_key: {}",
            self.valid, self.bucket_name, self.object_key
        )
    }
}
