## Migrations

Sometimes, it is necessary to refactor a data model after it has already been release to 
production. The `migrations` module provides a mechanism to both `apply` and `rollback` a migration.
This mechanism is limited in its opinions and feature set, since migrations are highly 
contextual by their nature. Migration script authors are responsible for ensuring that their 
migrations are either idempotent and safe to re-run, or for checking the migration log to 
determine if a script has already been applied.

### Migration Component Model

A migration script can be defined by implement the `Migration` trait which specifies the following 
interface.

```rust
pub trait Migration {
    /// The human readable name of the migration.
    fn name(&self) -> String;
    /// The set of operations to apply to a database to migrate it to a desired new state.
    async fn commit(&self, service: &MigrationService) -> Result<LogEntry, Error>;
    /// The set of operations to undo a previously committed migration.
    async fn rollback(&self, service: &MigrationService) -> Result<LogEntry, Error>;
}
```

A `Migration` implementation can then be passed to the `MigrationService` along with an `Effect` 
enum. The `Effect` enum specifies whether to apply or rollback the migration.

```rust
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
```

User code is required to coordinate the application of migrations, in which order, and to what 
effect.




