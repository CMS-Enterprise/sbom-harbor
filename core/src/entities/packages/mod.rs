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
pub use unsupported::*;
pub use xrefs::*;

use crate::entities::xrefs::{Xref, XrefKind, Xrefs};
use crate::xref;
use platform::mongodb::{mongo_doc, MongoDocument};
use std::collections::HashMap;

mongo_doc!(Dependency);
mongo_doc!(Package);
mongo_doc!(Purl);
mongo_doc!(Unsupported);

xref!(Dependency);
xref!(Package);
xref!(Purl);
xref!(Unsupported);
xref!(Finding);
