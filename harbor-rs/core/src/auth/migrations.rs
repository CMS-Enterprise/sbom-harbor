use std::collections::HashMap;
use async_trait::async_trait;

use aqum::mongodb::auth::*;
use aqum::mongodb::{Context, ObjectId, Store};
use aqum::mongodb::migrations::*;

use crate::auth::ResourceKind;

use aqum::Error;

pub async fn sync(ctx: Context) -> Result<Vec<LogEntry>, Error> {
    let service = MigrationService::new(&ctx).await?;
    let mut log_entries = vec![];

    let entry = init_auth(&service).await?;
    log_entries.push(entry);

    Ok(log_entries)
}


pub async fn init_auth(service: &MigrationService) -> Result<LogEntry, Error> {
    let migration = InitAuth{};
    service.apply(&migration, Direction::Up)
        .await
        .map_err(|e| Error::Migration(e.to_string()))
}

struct InitAuth{}

#[async_trait]
impl Migration for InitAuth {
    fn name(&self) -> String {
        "init_auth".to_string()
    }

    async fn up(&self, service: &MigrationService) -> Result<LogEntry, Error> {
        let store = Store::new(&service.ctx).await?;

        // Build the minimal require authentication data.
        let any_resource = Self::ensure_any_resource(&store).await?;
        let admin_policy = Self::ensure_allow_any_policy(&store, &any_resource).await?;
        Self::ensure_disabled_policy(&store, &any_resource).await?;
        let admin_role = Self::ensure_admin_role(&store, &admin_policy).await?;
        Self::ensure_admin_group(&store, &admin_role).await?;

        let log_entry = LogEntry::new(self.name(), Direction::Up);
        store.insert(log_entry.clone()).await?;

        Ok(log_entry)
    }

    async fn down(&self, _service: &MigrationService) -> Result<LogEntry, Error> {
        todo!()
    }
}

impl InitAuth {
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
            id: ObjectId::new().to_string(),
            name: "*".to_string(),
            kind: ResourceKind::Any.to_string(),
        };

        store.insert::<Resource>(any_resource.clone()).await?;
        Ok(any_resource)
    }

    // The allow any policy allows any action on any resource
    async fn ensure_allow_any_policy(store: &Store, any_resource: &Resource) -> Result<Policy, Error> {
        let filter = HashMap::from([("name", "allow any")]);
        let existing = store.query::<Policy>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration("invalid db state: allow any policy".to_string()));
            }
            let existing = existing.first().unwrap();
            if !existing.resource_id.eq(&any_resource.id) {
                return Err(Error::Migration("invalid id state: allow any policy".to_string()));
            }
            return Ok(existing.clone());
        }

        let allow_any = Policy {
            id: ObjectId::new().to_string(),
            name: "allow any".to_string(),
            resource_id: any_resource.id.clone(),
            action: Action::Any,
            effect: Effect::Allow,
        };

        store.insert::<Policy>(allow_any.clone()).await?;
        Ok(allow_any)
    }

    // The disabled policy allows for disabling users without deleting them.
    // This can even be applied to admins. Useful when an account should be
    // denied access, but retained for audit purposes.
    async fn ensure_disabled_policy(store: &Store, any_resource: &Resource) -> Result<(), Error> {
        let filter = HashMap::from([("name", "disabled")]);
        let existing = store.query::<Policy>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration("invalid db state: disabled policy".to_string()));
            }
            let existing = existing.first().unwrap();
            if !existing.resource_id.eq(&any_resource.id) {
                return Err(Error::Migration("invalid id state: disabled policy".to_string()));
            }

            return Ok(());
        }

        let disabled_policy = Policy {
            id: ObjectId::new().to_string(),
            name: "disabled".to_string(),
            resource_id: any_resource.id.clone(),
            action: Action::Any,
            effect: Effect::Deny,
        };

        store.insert::<Policy>(disabled_policy.clone()).await?;
        Ok(())
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
            id: ObjectId::new().to_string(),
            name: "administrator".to_string(),
            policies: vec![admin_policy.id.clone()],
        };

        store.insert::<Role>(admin_role.clone()).await?;
        Ok(admin_role)
    }

    async fn ensure_admin_group(store: &Store, admin_role: &Role) -> Result<(), Error>{
        let filter = HashMap::from([("name", "Administrators")]);
        let existing = store.query::<Group>(filter).await?;
        if !existing.is_empty() {
            if existing.len() > 1 {
                return Err(Error::Migration("invalid db state: admin group".to_string()));
            }
            let existing = existing.first().unwrap();
            if !existing.roles.contains(&admin_role.id) {
                return Err(Error::Migration("invalid id state: admin group".to_string()));
            }

            return Ok(());
        }

        let admin_group = Group {
            id: ObjectId::new().to_string(),
            name: "Administrators".to_string(),
            users: vec![],
            roles: vec![admin_role.id.clone()],
        };

        store.insert::<Group>(admin_group.clone()).await?;

        Ok(())
    }
}
