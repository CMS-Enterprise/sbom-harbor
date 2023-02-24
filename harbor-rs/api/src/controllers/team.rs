use std::sync::Arc;
use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use tracing::instrument;

use harbor_core::models::Team;
use harbor_core::services::TeamService;
use aqum::mongodb::{Service, Store};

use crate::auth::Claims;
use crate::Error;

pub type DynTeamService = Arc<TeamService>;

pub fn new_service<'a>(store: Arc<Store>) -> Arc<TeamService> {
    Arc::new(TeamService::new(store))
}

// WATCH: Trying to get by without a custom extractor.
#[instrument]
#[debug_handler]
pub async fn get(
    _claims: Claims,
    // Query(query): Query<HashMap<String, String>>,
    Path(id): Path<String>,
    State(service): State<DynTeamService>) -> Result<Json<Team>, Error> {

    let team = service
        .find(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    match team {
        None => Err(Error::DoesNotExist(format!("team not found: {}", id))),
        Some(t) => Ok(t.into()),
    }
}

#[instrument]
#[debug_handler]
pub async fn list(
    _claims: Claims,
    // Query(children): Query<HashMap<String, String>>,
    State(service): State<DynTeamService>) -> Result<Json<Vec<Team>>, Error> {

    // TODO: Does a children flag still hold?
    // let ctx = ListTeamsContext {
    //     children: resolve_children(query)
    // };

    let teams = service
        .list()
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(teams.into())
}
