use std::sync::Arc;
use aqum::Error;
use aqum::dynamo::{
    Context as DynamoContext,
    Store,
    Service as DynamoService
};

use crate::entities::{Team as TeamEntity};
use crate::models::Team;

#[derive(Debug)]
pub struct TeamService {
    store: Arc<Store>,
}

impl TeamService {
    pub fn new(store: Arc<Store>) -> TeamService {
        TeamService {
            store,
        }
    }
}

impl DynamoService<'_, Team, TeamEntity, TeamContext> for TeamService {
    fn store(&self) -> &Store {
        &self.store
    }
}

/// TeamContext
#[derive(Debug)]
pub struct TeamContext {
    /// Id query constraint.
    pub id: String,
    /// Flag indicating whether to include children in the result.
    pub children: bool,
}

impl DynamoContext<'_, TeamEntity> for TeamContext {
    fn as_dynamo_entity(&self) -> Result<TeamEntity, Error> {
        let mut entity = TeamEntity::new("".to_string());
        entity.partition_key = self.id.clone();
        Ok(entity)
    }

    fn is_aggregate_root(&self) -> bool {
        self.children
    }
}
