use aqum::dynamo::{Context as DynamoContext, Service as DynamoService, Store, to_entity};
use aqum::Error;
use std::sync::Arc;

use crate::entities::Team as TeamEntity;
use crate::models::Team;

#[derive(Debug)]
pub struct TeamService {
    store: Arc<Store>,
}

impl TeamService {
    pub fn new(store: Arc<Store>) -> TeamService {
        TeamService { store }
    }
}

impl DynamoService<'_, Team, TeamEntity> for TeamService {
    fn store(&self) -> &Store {
        &self.store
    }
}

#[derive(Debug)]
pub struct TeamsContext {
    pub children: bool,
}

impl DynamoContext<'_, TeamEntity> for TeamsContext {
    fn as_dynamo_entity(&self) -> Result<TeamEntity, Error> {
        // List queries, by convention rely on blank entities.
        // This is not ideal.
        Ok(TeamEntity::new("".to_string()))
    }

    fn is_aggregate_root(&self) -> bool {
        self.children
    }
}

impl TeamsContext {
    pub fn new(children: bool) -> Self {
        Self {children}
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

/// CreateTeamContext
#[derive(Debug)]
pub struct CreateTeamContext {
    /// Team model
    pub team: Team,
    /// Flag indicating whether to handle children in the operation.
    pub children: bool,
}

impl DynamoContext<'_, TeamEntity> for CreateTeamContext {
    fn as_dynamo_entity(&self) -> Result<TeamEntity, Error> {
        let entity = to_entity::<Team, TeamEntity>(&self.team)?;
        Ok(entity)
    }

    fn is_aggregate_root(&self) -> bool {
        self.children
    }
}

/// UpdateTeamContext
#[derive(Debug)]
pub struct UpdateTeamContext {
    /// Id query constraint.
    pub id: String,
    /// Team model
    pub team: Team,
    /// Flag indicating whether to handle children in the operation.
    pub children: bool,
}

impl DynamoContext<'_, TeamEntity> for UpdateTeamContext {
    fn as_dynamo_entity(&self) -> Result<TeamEntity, Error> {
        if self.id.is_empty() {
            return Err(Error::Entity("identity invalid".to_string()));
        }

        let entity = to_entity::<Team, TeamEntity>(&self.team)?;
        if entity.id != self.id {
            return Err(Error::Entity("identity mismatch".to_string()));
        }

        Ok(entity)
    }

    fn is_aggregate_root(&self) -> bool {
        self.children
    }
}
