use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use serde::{
    Deserialize,
    Serialize
};

use platform::mongodb::{
    Service as MongoService,
    MongoDocument,
    Store,
    mongo_doc,
};

#[derive(Clone)]
pub struct GitHubProviderMongoService {
    pub(crate) store: Arc<Store>
}

impl GitHubProviderMongoService {
    pub(crate) fn new(store: Store) -> Self {
        GitHubProviderMongoService {
            store: Arc::new(store)
        }
    }
}

impl MongoService<GitHubSbomProviderEntry>
    for GitHubProviderMongoService {

    fn store(&self) -> Arc<Store> {
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

    svc.insert(
        &mut GitHubSbomProviderEntry {
            id,
            last_hash,
            team_id,
            project_id,
            codebase_id,
        }
    );

    match svc.find(id.as_str()) {
        Ok(opt) => match opt {
            Some(doc) => {
                assert_eq!(id, doc.id);
                assert_eq!(last_hash, doc.last_hash);
                assert_eq!(team_id, doc.team_id);
                assert_eq!(project_id, doc.project_id);
                assert_eq!(codebase_id, doc.codebase_id);
            },
            None => panic!("No value in Option: Missing GitHubSbomProviderEntry")
        },
        Err(err) => panic!("Error getting GitHubSbomProviderEntry: {}", err)
    }
}
