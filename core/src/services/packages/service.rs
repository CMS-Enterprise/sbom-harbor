use crate::entities::packages::{Dependency, Package, Purl, Unsupported};
use platform::mongodb::{Context, Service};
use std::fmt::{Debug, Formatter};

/// Provides Finding related data management capabilities.
#[derive(Debug)]
pub struct PackageService {
    cx: Context,
}

impl Service<Dependency> for PackageService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Package> for PackageService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Purl> for PackageService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl Service<Unsupported> for PackageService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl PackageService {}
