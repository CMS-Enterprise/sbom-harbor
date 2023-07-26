use platform::mongo_doc;
use platform::persistence::mongodb::MongoDocument;

mod product;
pub use product::*;

mongo_doc!(product::Product);
