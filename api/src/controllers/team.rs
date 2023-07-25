use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

use harbcore::entities::teams::Team;
use harbcore::services::teams::TeamService;
use platform::auth::User;
use platform::persistence::mongodb::Service;

use crate::auth::Claims;
use crate::Error;

/// Type alias for the [TeamService] instance.
pub type DynTeamService = Arc<TeamService>;

// WATCH: Trying to get by without a custom extractor.
/// Get a [Team] by id.
#[instrument]
#[debug_handler]
pub async fn get(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynTeamService>,
) -> Result<Json<Team>, Error> {
    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    let team = service
        .find(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    match team {
        None => Err(Error::DoesNotExist(format!("team not found: {}", id))),
        Some(t) => Ok(Json(t)),
    }
}

/// List all [Teams].
#[instrument]
#[debug_handler]
pub async fn list(
    _claims: Claims,
    State(service): State<DynTeamService>,
) -> Result<Json<Vec<Team>>, Error> {
    let teams = service
        .list()
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(teams))
}

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct TeamInsert {
    /// The name of the team.
    pub name: Option<String>,

    /// [User] instances that represent users that are members of the Team.
    pub members: Option<HashMap<String, User>>,
}

impl TeamInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<Team, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::InvalidParameters("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        Team::new(name, self.members.clone()).map_err(|e| Error::InvalidParameters(e.to_string()))
    }
}

/// Post a new [Team].
#[instrument]
#[debug_handler]
pub async fn post(
    _claims: Claims,
    State(service): State<DynTeamService>,
    Json(team): Json<Team>,
) -> Result<Json<Team>, Error> {
    if !team.id.is_empty() {
        return Err(Error::InvalidParameters(
            "client generated id invalid".to_string(),
        ));
    }

    let mut team = team;

    service
        .insert(&mut team)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(team))
}

/// Put an updated [Team].
#[instrument]
#[debug_handler]
pub async fn put(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynTeamService>,
    Json(team): Json<Team>,
) -> Result<Json<Team>, Error> {
    if id != team.id {
        return Err(Error::InvalidParameters("id mismatch".to_string()));
    }

    let team = team;

    service
        .update(&team)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(team))
}

/// Delete and existing [Team].
#[instrument]
#[debug_handler]
pub async fn delete(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynTeamService>,
) -> Result<Json<()>, Error> {
    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    service
        .delete(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(()))
}
