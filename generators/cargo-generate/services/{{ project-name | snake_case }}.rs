use platform::mongodb::{Service, Store};
use std::sync::Arc;

use crate::models::{{ response_type }};

#[derive(Debug)]
pub struct {{ response_type }}Service {
    store: Arc<Store>,
}

impl {{ response_type }}Service {
    pub fn new(store: Arc<Store>) -> {{ response_type }}Service {
        {{ response_type }}Service { store }
    }
}

impl Service<{{ response_type }}> for {{ response_type }}Service {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }

    // TODO: Override default implementations as needed.
}