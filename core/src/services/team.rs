use platform::mongodb::{Service, Store};
use std::sync::Arc;

use crate::models::Team;

#[derive(Debug)]
pub struct TeamService {
    store: Arc<Store>,
}

impl TeamService {
    pub fn new(store: Arc<Store>) -> TeamService {
        TeamService { store }
    }
}

impl Service<Team> for TeamService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}
