use std::sync::Arc;
use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use tracing::instrument;

use harbcore::models::Team;
use harbcore::services::TeamService;
use platform::mongodb::{Service, Store};

use crate::auth::Claims;
use crate::Error;

/// Type alias for the [TeamService] instance.
pub type DynTeamService = Arc<TeamService>;

/// Factory method for a new instance of a TeamService.
pub fn new_service<'a>(store: Arc<Store>) -> Arc<TeamService> {
    Arc::new(TeamService::new(store))
}

// WATCH: Trying to get by without a custom extractor.
/// Get a [Team] by id.
#[instrument]
#[debug_handler]
pub async fn get(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynTeamService>) -> Result<Json<Team>, Error> {

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
    State(service): State<DynTeamService>) -> Result<Json<Vec<Team>>, Error> {

    let teams = service
        .list()
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(teams))
}

/// Post a new [Team].
#[instrument]
#[debug_handler]
pub async fn post(
    _claims: Claims,
    State(service): State<DynTeamService>,
    Json(team): Json<Team>) -> Result<Json<Team>, Error> {

    if !team.id.is_empty() {
        return Err(Error::InvalidParameters("client generated id invalid".to_string()));
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
    Json(team): Json<Team>) -> Result<Json<Team>, Error> {

    if id != team.id {
        return Err(Error::InvalidParameters("id mismatch".to_string()));
    }

    let mut team = team;

    service
        .update(&mut team)
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
    State(service): State<DynTeamService>) -> Result<Json<()>, Error> {

    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    service
        .delete(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(()))
}
