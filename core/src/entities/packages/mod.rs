mod dependency;
mod package;
mod purl;
mod registry;
mod vulnerability;
mod xrefs;

pub use dependency::*;
pub use package::*;
pub use purl::*;
pub use registry::*;

use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Dependency);
mongo_doc!(Package);
mongo_doc!(Purl);
mongo_doc!(Unsupported);
mongo_doc!(Vulnerability);
mongo_doc!(SnykXRef);
