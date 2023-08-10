use std::fmt::Debug;
use std::sync::Arc;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use platform::{hyper, mongo_doc};
use platform::hyper::ContentType;
use platform::persistence::mongodb::{MongoDocument, Service as MongoService, Store as MongoStore};
use crate::services::snyk::API_VERSION;
use crate::Error;
use crate::services::snyk::client::{orgs_url, projects_url};


/// A purpose build Snyk HTTP Client.
#[derive(Debug)]
pub struct Client {
    token: String,
    inner: hyper::Client,
}

impl Client {
    /// Factory method for creating new instances of a Client.
    pub fn new(token: String) -> Self {
        let inner = hyper::Client::new();
        Self { token, inner }
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

    pub async fn orgs(&self) -> Result<Value, Error> {
        let response: Option<Value> = self
            .inner
            .get(
                &orgs_url(),
                ContentType::Json,
                &self.token(),
                None::<Value>,
            )
            .await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list orgs".to_string())),
            Some(value) => Ok(value),
        }
    }

    pub async fn projects(
        &self,
        org_id: &str,
    ) -> Result<Value, Error> {
        let response: Option<Value> = self
            .inner
            .get(
                &projects_url(org_id),
                ContentType::Json,
                &self.token(),
                None::<Value>,
            )
            .await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list projects".to_string())),
            Some(value) => Ok(value),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EnrichedSnykOrg {
    id: String
}
mongo_doc!(EnrichedSnykOrg);

#[derive(Clone, Debug)]
pub struct DbSvc {
    pub(crate) store: Arc<MongoStore>,
}

// #[async_trait]
impl MongoService<EnrichedSnykOrg> for DbSvc {
    fn store(&self) -> Arc<MongoStore> {
        self.store.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use serde_json::{Map, Value};
    use platform::persistence::mongodb::{Service, Store};
    use crate::config::dev_context;
    use crate::Error;
    use crate::services::snyk::client::rawclient::{Client, DbSvc, EnrichedSnykOrg};

    fn combine(org_map: Map<String, Value>, projects_arr: Vec<Value>) -> Map<String, Value> {
        let mut map: Map<String, Value> = Map::new();

        for (key, value) in org_map {
            map.insert(key, value);
        }

        map.insert(String::from("projects"), Value::Array(projects_arr));

        map
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn extracts_raw_snyk_data() -> Result<(), Error> {
        let token = std::env::var("SNYK_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let client = Client::new(token);
        let orgs_obj = client.orgs().await?;
        let orgs_arr = orgs_obj.get("orgs").unwrap();
        let org_vec: &Vec<Value> = orgs_arr.as_array().unwrap();

        let cx = dev_context(None)?;
        let store = Arc::new(Store::new(&cx).await?);

        let db_svc: DbSvc = DbSvc {
            store,
        };

        for org in org_vec {
            let map: &Map<String, Value> = org.as_object().unwrap();
            let id_value = map.get("id").unwrap().as_str().unwrap();
            let slug = map.get("slug").unwrap().as_str().unwrap();
            let projects_value = client.projects(id_value).await.unwrap();
            let projects_arr = projects_value.as_object()
                .unwrap().get("data").unwrap().as_array().unwrap();
            let combined_org_and_projects = combine((*map).clone(), (*projects_arr).clone());
            let mut org_value = Value::Object(combined_org_and_projects);

            println!("Inserting: {}", slug);
            db_svc.store().insert_serde("EnrichedSnykOrgs", &mut org_value)
                .await.expect("Error inserting EnrichedSnykOrg...");
        }

        Ok(())
    }
}
