// use std::collections::HashMap;
// use std::sync::Arc;
// use aqum::dynamo::{Service as AqumService};
// use axum::{debug_handler, Json};
// use axum::extract::{Path, Query, State};
// use tracing::instrument;
// use uuid::Uuid;
//
// use harbor_core::models::Team;
// use harbor_core::entities::Team as TeamEntity;
// use harbor_core::services::{CreateTeamContext, TeamContext, TeamService, UpdateTeamContext};
// use aqum::dynamo::Store;
//
// use crate::auth::Claims;
// use crate::controllers::resolve_children;
// use crate::Error;
//
// pub type DynTeamService<'a, Team, TeamEntity, TeamContext> = Arc<dyn AqumService<'a, Team, TeamEntity>>;
//
// pub fn new_service<'a>(store: Arc<Store>) -> DynTeamService<'a, Team, TeamEntity, TeamContext> {
//     Arc::new(TeamService::new(store))
// }
//
// // WATCH: Trying to get by without a custom extractor.
// #[instrument]
// #[debug_handler]
// pub async fn get(
//     _claims: Claims,
//     Query(query): Query<HashMap<String, String>>,
//     Path(id): Path<Uuid>,
//     State(store): State<Arc<Store>>) -> Result<Json<Team>, Error> {
//     // State(service): State<DynTeamService<'static, Team, TeamEntity, TeamContext>>) -> Result<Json<Team>, Error> {
//
//     let service = new_service(store);
//     let ctx = TeamContext {
//         id: id.to_string(),
//         children: resolve_children(query)
//     };
//
//     let team = service
//         .find(&ctx)
//         .await
//         .map_err(|e| Error::InternalServerError(e.to_string()))?;
//
//     match team {
//         None => Err(Error::DoesNotExist(format!("team not found: {}", id))),
//         Some(t) => Ok(t.into()),
//     }
// }
