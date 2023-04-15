mod cdx;
mod dependency;
mod finding;
mod package;
mod purl;
mod unsupported;
mod xrefs;

pub use cdx::*;
pub use dependency::*;
pub use finding::*;
pub use package::*;
pub use purl::*;
pub use registry::*;

use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Dependency);
mongo_doc!(Finding);
mongo_doc!(Package);
mongo_doc!(Purl);
mongo_doc!(Unsupported);
mongo_doc!(SnykXRef);
