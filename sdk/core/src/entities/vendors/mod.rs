use platform::mongo_doc;
use platform::persistence::mongodb::MongoDocument;

mod product;
pub use product::*;
mod vendor;
pub use vendor::*;

mongo_doc!(vendor::Vendor);
