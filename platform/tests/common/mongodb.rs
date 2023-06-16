use mongodb::Client as MongoClient;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use platform::auth::*;
use platform::mongodb::auth::DefaultAuthorizer;
use platform::mongodb::{Context, MongoDocument, Store};
use platform::Error;

pub const DB_IDENTIFIER: &str = "platform";
pub const KEY_NAME: &str = "id";
pub const COLLECTION: &str = "Group";

pub async fn local_context() -> Result<Context, Error> {
    let cx: Context = Context {
        host: "mongo".to_string(),
        username: "root".to_string(),
        password: "harbor".to_string(),
        port: 27017,
        db_name: DB_IDENTIFIER.to_string(),
        key_name: KEY_NAME.to_string(),
        connection_uri: None,
    };

    let client = MongoClient::with_uri_str(cx.connection_uri()?).await?;
    let dbs = client.list_database_names(None, None).await?;

    if !dbs.contains(&DB_IDENTIFIER.to_string()) {
        return Err(Error::Mongo(format!("{} db does not exist", DB_IDENTIFIER)));
    }

    let db = client.database(DB_IDENTIFIER);
    let collections = db.list_collection_names(None).await?;

    if !collections.contains(&COLLECTION.to_string()) {
        return Err(Error::Mongo(format!(
            "{} collection does not exist",
            COLLECTION
        )));
    }

    Ok(cx)
}

pub struct AuthScenario {
    pub name: String,
    resource: Resource,
    role: Role,
    user: User,
    store: Arc<Store>,
}

impl AuthScenario {
    pub async fn new(name: String, ctx: &Context) -> Result<AuthScenario, Error> {
        let store = Arc::new(Store::new(ctx).await?);
        let mut scenario = AuthScenario {
            name: name.clone(),
            resource: Resource {
                id: Uuid::new_v4().to_string(),
                name: name.clone(),
                kind: "".to_string(),
            },
            role: Role {
                id: Uuid::new_v4().to_string(),
                name: name.clone(),
                policies: vec![],
            },
            user: User {
                id: Uuid::new_v4().to_string(),
                email: "".to_string(),
            },
            store,
        };

        // TODO: Analyze whether its possible this to the convention based default init auth migration.
        // Drift between these would be bad, but tests need to be uncoupled. May be as simple
        // as creating a new DB per test.
        let any_resource = Resource {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            kind: ANY_RESOURCE_KIND.to_string(),
        };
        scenario.with::<Resource>(&any_resource).await?;

        let allow_any_policy = Policy {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            resource_id: any_resource.id.clone(),
            action: Action::Any,
            effect: Effect::Allow,
        };
        scenario.with::<Policy>(&allow_any_policy).await?;

        let disabled_policy = Policy {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            resource_id: any_resource.id.clone(),
            action: Action::Any,
            effect: Effect::Deny,
        };
        scenario.with::<Policy>(&disabled_policy).await?;

        let admin_role = Role {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            policies: vec![allow_any_policy.id.clone()],
        };
        scenario.with::<Role>(&admin_role).await?;

        let admin_group = Group {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            users: vec![],
            roles: vec![admin_role.id.clone()],
        };
        scenario.with::<Group>(&admin_group).await?;

        let resource = Resource {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            kind: ANY_RESOURCE_KIND.to_string(),
        };
        scenario.with::<Resource>(&resource).await?;

        let role = Role {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            policies: vec![],
        };
        scenario.with::<Role>(&role).await?;

        let user = User {
            id: Uuid::new_v4().to_string(),
            email: name.clone(),
        };
        scenario.with::<User>(&user).await?;

        let group = Group {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            users: vec![user.id.clone()],
            roles: vec![role.id.clone()],
        };
        scenario.with::<Group>(&group).await?;

        scenario.resource = resource;
        scenario.role = role;
        scenario.user = user;

        Ok(scenario)
    }

    pub async fn with<T>(&self, d: &T) -> Result<(), Error>
    where
        T: MongoDocument,
    {
        self.store.insert::<T>(d).await?;

        Ok(())
    }

    pub async fn with_policy(&mut self, action: Action, effect: Effect) -> Result<(), Error> {
        let policy = Policy {
            id: Uuid::new_v4().to_string(),
            name: self.name.clone(),
            resource_id: self.resource.id.clone(),
            action,
            effect,
        };

        self.store.insert::<Policy>(&policy).await?;
        self.role.policies.push(policy.id);
        self.store.update::<Role>(&self.role).await
    }

    pub async fn assert(&self, action: Action) -> Result<Effect, Error> {
        let authorizer = DefaultAuthorizer::new(self.store.clone());
        authorizer
            .assert(self.user.clone(), self.resource.clone(), action)
            .await
    }

    pub async fn teardown(&self) -> Result<(), Error> {
        let filter = HashMap::from([("name", self.name.as_str())]);

        let groups = self.store.query::<Group>(filter.clone()).await?;
        for group in groups {
            self.store.delete::<Group>(group.id.as_str()).await?;
        }

        let policies = self.store.query::<Policy>(filter.clone()).await?;
        for policy in policies {
            self.store.delete::<Policy>(policy.id.as_str()).await?;
        }

        let resources = self.store.query::<Resource>(filter.clone()).await?;
        for resource in resources {
            self.store.delete::<Resource>(resource.id.as_str()).await?;
        }

        let roles = self.store.query::<Role>(filter.clone()).await?;
        for role in roles {
            self.store.delete::<Role>(role.id.as_str()).await?
        }

        // Users have emails not names.
        let filter = HashMap::from([("email", self.name.as_str())]);
        let users = self.store.query::<User>(filter).await?;
        for user in users {
            self.store.delete::<User>(user.id.as_str()).await?
        }

        Ok(())
    }

    pub async fn remove_group_policy_for_user(&self) -> Result<(), Error> {
        let filter = HashMap::from([("name", self.name.as_str())]);

        let groups = self.store.query::<Group>(filter.clone()).await?;
        for mut group in groups {
            group.users.retain(|user_id| self.user.id.eq(user_id));
            self.store.update::<Group>(&group).await?;
        }

        Ok(())
    }
}
