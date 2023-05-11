mod cdx;
mod dependency;
mod package;
mod purl;
mod unsupported;
mod xrefs;

pub use cdx::*;
pub use dependency::*;
pub use package::*;
pub use purl::*;
pub use unsupported::*;
pub use xrefs::*;

use super::xrefs::xref;
use super::xrefs::{Xref, Xrefs};
use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Dependency);
mongo_doc!(Package);
mongo_doc!(Purl);
mongo_doc!(Unsupported);

xref!(Dependency);
xref!(Package);
xref!(Purl);
xref!(Unsupported);
