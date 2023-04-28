mod scan;
pub use scan::*;

use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Scan);
