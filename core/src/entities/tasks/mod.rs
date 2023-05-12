mod task;
pub use task::*;

use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Task);
