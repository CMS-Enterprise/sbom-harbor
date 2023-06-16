use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

use crate::auth::{Action, Effect, Group, Policy, Resource, Role, ANY_RESOURCE_KIND};
use crate::mongodb::migrations::{
    Effect as MigrationEffect, LogEntry, Migration, MigrationService,
};
use crate::mongodb::Store;
use crate::Error;

/// Applies the default set of migrations.
pub async fn apply_all(service: &MigrationService) -> Result<LogEntry, Error> {
    let migration = InitDefaultAuth {};
    service
        .apply(&migration, MigrationEffect::Commit)
        .await
        .map_err(|e| Error::Migration(e.to_string()))
}

/// Initializes a store by applying the default authorization migration.
pub struct InitDefaultAuth {}

#[async_trait]
impl Migration for InitDefaultAuth {
    fn name(&self) -> String {
        "init_default_auth".to_string()
    }

    async fn commit(&self, service: &MigrationService) -> Result<LogEntry, Error> {
        let store = Store::new(&service.ctx).await?;

        // Build the minimal require authentication data.

        // Create an "any" resource so that global allow/deny can be configured.
        let any_resource = Self::ensure_any_resource(&store).await?;

        // Create administrator policy/role/group to enable global allows.
        let admin_policy = Self::ensure_allow_any_policy(&store, &any_resource).await?;
        let admin_role = Self::ensure_admin_role(&store, &admin_policy).await?;
        Self::ensure_admin_group(&store, &admin_role).await?;

        // Create deny policy/role/group to enable global deny.
        let disabled_policy = Self::ensure_disabled_policy(&store, &any_resource).await?;
        let disabled_role = Self::ensure_disabled_role(&store, &disabled_policy).await?;
        Self::ensure_disabled_group(&store, &disabled_role).await?;

        let log_entry = LogEntry::new(self.name(), MigrationEffect::Commit);
        store.insert(&log_entry).await?;

        Ok(log_entry)
    }

    async fn rollback(&self, _service: &MigrationService) -> Result<LogEntry, Error> {
        todo!()
    }
}

impl InitDefaultAuth {
    // The any resource allows for ergonomic assertion of policies for admin users.
    async fn ensure_any_resource(store: &Store) -> Result<Resource, Error> {
        // Check if it exists
        let filter = HashMap::from([("name", "*")]);
        let existing = store.query::<Resource>(filter).await?;
        if !existing.is_empty() {
            // throw exception if more than one exists.
            if existing.len() > 1 {
                return Err(Error::Migration("invalid db state: resources".to_string()));
            }
            let existing = existing.first().unwrap().clone();
            return Ok(existing);
        }

        let any_resource = Resource {
            id: Uuid::new_v4().to_string(),
            name: "*".to_string(),
            kind: ANY_RESOURCE_KIND.to_string(),
        };

        store.insert::<Resource>(&any_resource).await?;
        Ok(any_resource)
    }

    // The allow any policy allows any action on any resource
    async fn ensure_allow_any_policy(
        store: &Store,
        any_resource: &Resource,
    ) -> Result<Policy, Error> {
        let filter = HashMap::from([("name", "allow any")]);
        let existing = store.query::<Policy>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration(
                    "invalid db state: allow any policy".to_string(),
                ));
            }
            let existing = existing.first().unwrap();
            if !existing.resource_id.eq(&any_resource.id) {
                return Err(Error::Migration(
                    "invalid id state: allow any policy".to_string(),
                ));
            }
            return Ok(existing.clone());
        }

        let allow_any = Policy {
            id: Uuid::new_v4().to_string(),
            name: "allow any".to_string(),
            resource_id: any_resource.id.clone(),
            action: Action::Any,
            effect: Effect::Allow,
        };

        store.insert::<Policy>(&allow_any).await?;
        Ok(allow_any)
    }

    async fn ensure_admin_role(store: &Store, admin_policy: &Policy) -> Result<Role, Error> {
        let filter = HashMap::from([("name", "administrator")]);
        let existing = store.query::<Role>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration("invalid db state: admin role".to_string()));
            }
            let existing = existing.first().unwrap();
            if !existing.policies.contains(&admin_policy.id) {
                return Err(Error::Migration("invalid id state: admin role".to_string()));
            }

            return Ok(existing.clone());
        }

        let admin_role = Role {
            id: Uuid::new_v4().to_string(),
            name: "administrator".to_string(),
            policies: vec![admin_policy.id.clone()],
        };

        store.insert::<Role>(&admin_role).await?;
        Ok(admin_role)
    }

    async fn ensure_admin_group(store: &Store, admin_role: &Role) -> Result<(), Error> {
        let filter = HashMap::from([("name", "Administrators")]);
        let existing = store.query::<Group>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration(
                    "invalid db state: admin group".to_string(),
                ));
            }
            let existing = existing.first().unwrap();
            if !existing.roles.contains(&admin_role.id) {
                return Err(Error::Migration(
                    "invalid id state: admin group".to_string(),
                ));
            }

            return Ok(());
        }

        let admin_group = Group {
            id: Uuid::new_v4().to_string(),
            name: "Administrators".to_string(),
            users: vec![],
            roles: vec![admin_role.id.clone()],
        };

        store.insert::<Group>(&admin_group).await?;

        Ok(())
    }

    // The disabled policy allows for disabling users without deleting them.
    // This can even be applied to admins. Useful when an account should be
    // denied access, but retained for audit purposes.
    async fn ensure_disabled_policy(
        store: &Store,
        any_resource: &Resource,
    ) -> Result<Policy, Error> {
        let filter = HashMap::from([("name", "disabled")]);
        let existing = store.query::<Policy>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration(
                    "invalid db state: disabled policy".to_string(),
                ));
            }
            let existing = existing.first().unwrap();
            if !existing.resource_id.eq(&any_resource.id) {
                return Err(Error::Migration(
                    "invalid id state: disabled policy".to_string(),
                ));
            }

            return Ok(existing.clone());
        }

        let disabled_policy = Policy {
            id: Uuid::new_v4().to_string(),
            name: "disabled".to_string(),
            resource_id: any_resource.id.clone(),
            action: Action::Any,
            effect: Effect::Deny,
        };

        store.insert::<Policy>(&disabled_policy).await?;
        Ok(disabled_policy)
    }

    async fn ensure_disabled_role(store: &Store, disabled_policy: &Policy) -> Result<Role, Error> {
        let filter = HashMap::from([("name", "disabled")]);
        let existing = store.query::<Role>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration(
                    "invalid db state: disabled role".to_string(),
                ));
            }
            let existing = existing.first().unwrap();
            if !existing.policies.contains(&disabled_policy.id) {
                return Err(Error::Migration(
                    "invalid id state: disabled role".to_string(),
                ));
            }

            return Ok(existing.clone());
        }

        let disabled_role = Role {
            id: Uuid::new_v4().to_string(),
            name: "disabled".to_string(),
            policies: vec![disabled_policy.id.clone()],
        };

        store.insert::<Role>(&disabled_role).await?;
        Ok(disabled_role)
    }

    async fn ensure_disabled_group(store: &Store, disabled_role: &Role) -> Result<(), Error> {
        let filter = HashMap::from([("name", "disabled")]);
        let existing = store.query::<Group>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration(
                    "invalid db state: disabled group".to_string(),
                ));
            }
            let existing = existing.first().unwrap();
            if !existing.roles.contains(&disabled_role.id) {
                return Err(Error::Migration(
                    "invalid id state: disabled group".to_string(),
                ));
            }

            return Ok(());
        }

        let disabled_group = Group {
            id: Uuid::new_v4().to_string(),
            name: "disabled".to_string(),
            users: vec![],
            roles: vec![disabled_role.id.clone()],
        };

        store.insert::<Group>(&disabled_group).await?;

        Ok(())
    }
}
