mod cdx;
mod package;
mod unsupported;
mod xrefs;

pub use cdx::*;
pub use package::*;
pub use unsupported::*;

use super::xrefs::xref;
use super::xrefs::{Xref, Xrefs};
use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Package);
mongo_doc!(Unsupported);

xref!(Package);
xref!(Unsupported);
