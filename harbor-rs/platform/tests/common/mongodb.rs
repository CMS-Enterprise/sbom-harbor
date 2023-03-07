use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use aws_sdk_docdb::Client;
use mongodb::Client as MongoClient;
use uuid::Uuid;

use platform::Error;
use platform::mongodb::{Context, MongoDocument, Store};
use platform::auth::*;
use platform::mongodb::auth::DefaultAuthorizer;

use crate::common::config_from_env;

const USERNAME: &str = "harbor-test-admin";
const PASSWORD: &str = "harbor-test-password";
pub const DB_IDENTIFIER: &str = "harbor";
pub const KEY_NAME: &str = "id";
pub const COLLECTION: &str = "Group";

pub struct ClusterContext {
    endpoint: String,
}

impl ClusterContext {
    #[allow(dead_code)]
    pub fn connection_string(&self) -> String {
        format!("mongodb://{}:{}@{}", USERNAME, PASSWORD, self.endpoint)
    }
}

pub struct LocalContext;

impl LocalContext {
    #[allow(dead_code)]
    pub fn connection_string() -> String {
        "mongodb://localhost:27017".to_string()
    }
}

#[allow(dead_code)]
pub async fn local_context() -> Result<Context, Error> {
    let ctx = Context{
        connection_uri: LocalContext::connection_string(),
        db_name: DB_IDENTIFIER.to_string(),
        key_name: KEY_NAME.to_string(),
    };

    let client = MongoClient::with_uri_str(ctx.connection_uri.clone()).await?;
    let dbs = client.list_database_names(None, None).await?;

    if !dbs.contains(&DB_IDENTIFIER.to_string()) {
        return Err(Error::Mongo(format!("{} db does not exist", DB_IDENTIFIER.to_string())))
    }

    let db = client.database(DB_IDENTIFIER);
    let collections = db.list_collection_names(None).await?;

    if !collections.contains(&COLLECTION.to_string()) {
        return Err(Error::Mongo(format!("{} collection does not exist", COLLECTION.to_string())));
    }

    Ok(ctx)
}

#[allow(dead_code)]
pub async fn cluster_context() -> Result<ClusterContext, Error> {
    let config = config_from_env().await?;
    let client = Client::new(&config);

    let output = client
        .describe_db_clusters()
        .db_cluster_identifier(DB_IDENTIFIER)
        .send()
        .await;

    if output.is_ok() {
        let db_clusters = output.unwrap();
        let db_clusters = db_clusters.db_clusters().unwrap();

        let endpoint = db_clusters
            .first()
            .unwrap()
            .endpoint()
            .unwrap();

        return Ok(ClusterContext { endpoint: endpoint.to_string() });
    }

    create_db_cluster(&client).await?;
    let ctx = wait_for_ready_cluster(&client).await?;

    Ok(ctx)
}

#[allow(dead_code)]
async fn create_db_cluster(client: &Client) -> Result<(), Error> {
    let output = client.create_db_cluster()
        .db_cluster_identifier(DB_IDENTIFIER)
        .engine("docdb")
        .engine_version("4.0.0")
        .master_username(USERNAME)
        .master_user_password(PASSWORD)
        .send()
        .await?;

    let cluster = output.db_cluster().unwrap();
    let endpoint = cluster.endpoint().unwrap();

    println!("{}", endpoint);
    println!("{}", cluster.status().unwrap());

    Ok(())
}

#[allow(dead_code)]
async fn wait_for_ready_cluster(client: &Client) -> Result<ClusterContext, Error> {
    let mut ctx = ClusterContext {
        endpoint: "".to_string(),
    };

    loop {
        if let Some(db_clusters) = client
            .describe_db_clusters()
            .db_cluster_identifier(DB_IDENTIFIER)
            .send()
            .await
            .ok() {
            if db_clusters.db_clusters().is_some() {
                let endpoint = db_clusters
                    .db_clusters()
                    .unwrap()
                    .first()
                    .unwrap()
                    .endpoint()
                    .unwrap();

                ctx.endpoint = endpoint.to_string();
                break;
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(ctx)
}

#[allow(dead_code)]
pub async fn teardown_cluster() -> Result<String, Error> {
    let config = config_from_env().await?;
    let client = Client::new(&config);

    client.delete_db_cluster()
        .db_cluster_identifier(DB_IDENTIFIER)
        .send()
        .await.map(|o| Ok(o.db_cluster()
            .unwrap()
            .status()
            .unwrap()
            .to_string()))?
}

pub struct AuthScenario {
    pub name: String,
    resource: Resource,
    role: Role,
    user: User,
    store: Arc<Store>,
}

impl AuthScenario {
    #[allow(dead_code)]
    pub async fn new(name: String, ctx: &Context) -> Result<AuthScenario, Error> {
        let store = Arc::new(Store::new(&ctx).await?);
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
            user: User { id: Uuid::new_v4().to_string(), email: "".to_string() },
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

        let resource = Resource{
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            kind: ANY_RESOURCE_KIND.to_string(),
        };
        scenario.with::<Resource>(&resource).await?;

        let role = Role{
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            policies: vec![],
        };
        scenario.with::<Role>(&role).await?;

        let user = User{
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

    #[allow(dead_code)]
    pub async fn with<T>(&self, d: &T) -> Result<(), Error>
    where T: MongoDocument {
        self.store.insert::<T>(d).await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn with_policy(&mut self, action: Action, effect: Effect) -> Result<(), Error> {
        let policy = Policy{
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

    #[allow(dead_code)]
    pub async fn assert(&self, action: Action) -> Result<Effect, Error> {
        let authorizer = DefaultAuthorizer::new(self.store.clone());
        authorizer.assert(self.user.clone(), self.resource.clone(), action).await
    }

    #[allow(dead_code)]
    pub async fn teardown(&self) -> Result<(), Error> {
        let filter = HashMap::from([("name", self.name.as_str())]);

        let groups = self.store.query::<Group>(filter.clone()).await?;
        for group in groups {
            self.store.delete::<Group>(group.id.as_str()).await?;
        }

        let policies = self.store.query::<Policy>(filter.clone()).await?;
        for policy in policies {
            self.store.delete::<Policy>(policy.id.as_str()).await?;
        };

        let resources = self.store.query::<Resource>(filter.clone()).await?;
        for resource in resources {
            self.store.delete::<Resource>(resource.id.as_str()).await?;
        };

        let roles = self.store.query::<Role>(filter.clone()).await?;
        for role in roles {
            self.store.delete::<Role>(role.id.as_str()).await?
        };

        // Users have emails not names.
        let filter = HashMap::from([("email", self.name.as_str())]);
        let users = self.store.query::<User>(filter).await?;
        for user in users {
            self.store.delete::<User>(user.id.as_str()).await?
        };

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn remove_group_policy_for_user(&self) -> Result<(), Error> {
        let filter = HashMap::from([("name", self.name.as_str())]);

        let groups = self.store.query::<Group>(filter.clone()).await?;
        for mut group in groups {
            group.users = group.users
                .into_iter()
                .filter(|user_id: &String| self.user.id.eq(user_id))
                .collect();
            self.store.update::<Group>(&group).await?;
        }

        Ok(())
    }
}
