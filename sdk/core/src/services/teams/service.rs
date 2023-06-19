use platform::persistence::mongodb::{Service, Store};
use std::sync::Arc;

use crate::entities::teams::Team;

/// Contains all domain and transaction logic related to [Teams] and their subordinate entities.
#[derive(Debug)]
pub struct TeamService {
    store: Arc<Store>,
}

impl TeamService {
    /// Factory method to create new instance of type.
    pub fn new(store: Arc<Store>) -> TeamService {
        TeamService { store }
    }
}

impl Service<Team> for TeamService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}
