use platform::persistence::mongodb::{Service, Store};
use std::sync::Arc;

use crate::entities::{{ service_type }};

#[derive(Debug)]
pub struct {{ service_type }}Service {
    store: Arc<Store>,
}

impl {{ service_type }}Service {
    pub fn new(store: Arc<Store>) -> {{ service_type }}Service {
        {{ service_type }}Service { store: store.clone() }
    }
}

impl Service<{{ service_type }}> for {{ service_type }}Service {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}
