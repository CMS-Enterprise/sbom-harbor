mod task;
pub use task::*;

use platform::persistence::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Task);
