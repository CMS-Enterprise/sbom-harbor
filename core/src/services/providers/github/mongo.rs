use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use serde::{
    Deserialize,
    Serialize
};

use platform::mongodb::{Service as MongoService, MongoDocument, Store as MongoStore, mongo_doc, Context};

#[derive(Clone)]
pub struct GitHubProviderMongoService {
    pub(crate) store: Arc<MongoStore>
}

impl GitHubProviderMongoService {
    pub(crate) fn new(store: MongoStore) -> Self {
        GitHubProviderMongoService {
            store: Arc::new(store)
        }
    }
}

impl MongoService<GitHubSbomProviderEntry>
    for GitHubProviderMongoService {

    fn store(&self) -> Arc<MongoStore> {
        (&self.store).clone()
    }
}

impl Debug for GitHubProviderMongoService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitHubProvider")
    }
}

/// Struct to define a GitHub Provider document in Mongo
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitHubSbomProviderEntry {

    /// Url of the GitHub Repository
    pub id: String,

    /// Last commit hash of the repository
    pub last_hash: String,

    /// Harbor v1 team id
    pub team_id: String,

    /// Harbor v1 project id
    pub project_id: String,

    /// Harbor v1 codebase id
    pub codebase_id: String,
}
mongo_doc!(GitHubSbomProviderEntry);

#[tokio::test]
async fn test_add_document() {

    let ctx = Context {
        host: "localhost".to_string(),
        port: 27017,
        db_name: "harbor".to_string(),
        key_name: "id".to_string(),
        username: "root".to_string(),
        password: "harbor".to_string(),
    };

    let svc = GitHubProviderMongoService {
        store: Arc::new(
            match MongoStore::new(&ctx).await {
                Ok(store) => store,
                Err(err) => panic!("Error getting store: {}", err)
            }
        )
    };

    let id = String::from("test-url-id");
    let last_hash = String::from("test-last-hash");
    let team_id = String::from("test-team-id");
    let project_id = String::from("test-project-id");
    let codebase_id = String::from("test-codebase-id");

    let entry = &mut GitHubSbomProviderEntry {
        id: id.clone(),
        last_hash: last_hash.clone(),
        team_id: team_id.clone(),
        project_id: project_id.clone(),
        codebase_id: codebase_id.clone(),
    };

    match svc.insert(entry).await {
        Ok(_result) => {
            match svc.find(id.as_str()).await {
                Ok(opt) => match opt {
                    Some(doc) => {
                        assert_eq!(id, doc.id.clone());
                        assert_eq!(last_hash, doc.last_hash);
                        assert_eq!(team_id, doc.team_id);
                        assert_eq!(project_id, doc.project_id);
                        assert_eq!(codebase_id, doc.codebase_id);
                    },
                    None => panic!("No value in Option: Missing GitHubSbomProviderEntry")
                },
                Err(err) => panic!("Error getting GitHubSbomProviderEntry: {}", err)
            }
        },
        Err(err) => panic!("Unable to insert document into Mongo: {}", err)
    }
}
