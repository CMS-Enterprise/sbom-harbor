use crate::entities::sboms::{Finding, Sbom};
use crate::services::findings::StorageProvider;
use crate::Error;
use platform::mongodb::{Context, Service};
use std::fmt::{Debug, Formatter};

/// Provides Finding related data management capabilities.
#[derive(Debug)]
pub struct FindingService {
    cx: Context,
    storage: Box<dyn StorageProvider>,
}

impl Service<Sbom> for FindingService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

pub fn new(cx: Context, storage: Box<dyn StorageProvider>) -> Self {
    Self { cx, storage }
}
