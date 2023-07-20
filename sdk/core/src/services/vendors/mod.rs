use platform::persistence::mongodb::{Service, Store};
use std::sync::Arc;

use crate::entities::vendors::Vendor;

/// Coordinates persistence logic for [Vendor] entities.
#[derive(Debug)]
pub struct VendorService {
    store: Arc<Store>,
}

impl VendorService {
    /// Factory method for new instance of type.
    pub fn new(store: Arc<Store>) -> VendorService {
        VendorService { store }
    }
}

impl Service<Vendor> for VendorService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}
