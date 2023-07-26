use platform::mongo_doc;
use platform::persistence::mongodb::MongoDocument;

mod vendor;
pub use vendor::*;

mongo_doc!(vendor::Vendor);
