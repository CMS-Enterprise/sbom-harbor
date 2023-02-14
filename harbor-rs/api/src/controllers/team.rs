use std::sync::Arc;
use aqum::dynamo::Service as AqumService;
use axum::{debug_handler, Json};
use axum::extract::{Path, Query, State};
use tracing::instrument;
use uuid::Uuid;

use harbor_core::models::Team;
use harbor_core::entities::Team as TeamEntity;
use harbor_core::services::{TeamContext, TeamService};
use aqum::dynamo::Store;

use crate::auth::Claims;
use crate::Error;

pub type DynTeamService<'a, Team, TeamEntity, TeamContext> = Arc<dyn AqumService<'a, Team, TeamEntity, TeamContext>>;

pub fn new_service<'a>(store: Arc<Store>) -> DynTeamService<'a, Team, TeamEntity, TeamContext> {
    Arc::new(TeamService::new(store))
}

#[instrument]
#[debug_handler]
pub async fn get(
    _claims: Claims,
    Path(id): Path<Uuid>,
    State(service): State<DynTeamService<'static, Team, TeamEntity, TeamContext>>) -> Result<Json<Team>, Error> {

    let children = false;

    let ctx = TeamContext {
        id: id.to_string(),
        children,
    };

    let team = service
        .get(&ctx)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    match team {
        None => Err(Error::DoesNotExist(format!("team not found: {}", id))),
        Some(t) => Ok(t.into()),
    }
}
