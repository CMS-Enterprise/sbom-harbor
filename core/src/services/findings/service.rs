use crate::entities::packages::Finding;
use crate::Error;
use platform::mongodb::{Context, Service};
use std::fmt::{Debug, Formatter};

/// Provides Finding related data management capabilities.
#[derive(Debug)]
pub struct FindingService {
    cx: Context,
}

impl Service<Finding> for FindingService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}
