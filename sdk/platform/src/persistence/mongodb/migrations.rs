use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::persistence::mongodb::Context;
use crate::Error;

/// LogEntry represents an applied DB Migration operation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogEntry {
    /// The unique id of the log entry.
    pub id: String,
    /// The name of the migration that was applied.
    pub name: String,
    /// The effect of the operation (e.g. Commit, Rollback)
    pub effect: Effect,
    /// The timestamp of when the operation was applied.
    timestamp: String,
}

impl LogEntry {
    /// Factory method for creating a new LogEntry instance.
    pub fn new(name: String, effect: Effect) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            effect,
            timestamp: chrono::Utc::now().to_string(),
        }
    }
}

/// Effect indicates whether to commit a Migration or whether to roll it back.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Effect {
    /// Indicates a migration should be committed.
    Commit,
    /// Indicates a migration should be rolled back.
    Rollback,
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Commit => write!(f, "commit"),
            Effect::Rollback => write!(f, "rollback"),
        }
    }
}

/// A Migration encapsulates a set of operations to perform on a database instance.
/// A Migration can include both schema and data updates. Migrations are expected
/// to be idempotent.
#[async_trait]
pub trait Migration {
    /// The human readable name of the migration.
    fn name(&self) -> String;
    /// The set of operations to apply to a database to migrate it to a desired new state.
    async fn commit(&self, service: &MigrationService) -> Result<LogEntry, Error>;
    /// The set of operations to undo a previously committed migration.
    async fn rollback(&self, service: &MigrationService) -> Result<LogEntry, Error>;
}

/// The `MigrationService` applies migrations against a MongoDB or DocumentDB instance.
pub struct MigrationService {
    /// The [Store] context.
    pub ctx: Context,
}

impl MigrationService {
    /// Factory method for creating a new `MigrationService` instance.
    pub async fn new(ctx: &Context) -> Result<MigrationService, Error> {
        Ok(MigrationService { ctx: ctx.clone() })
    }

    /// Applies a migration with the specified effect.
    pub async fn apply(
        &self,
        migration: &impl Migration,
        effect: Effect,
    ) -> Result<LogEntry, Error> {
        match effect {
            Effect::Commit => migration.commit(self).await,
            Effect::Rollback => migration.rollback(self).await,
        }
    }
}
