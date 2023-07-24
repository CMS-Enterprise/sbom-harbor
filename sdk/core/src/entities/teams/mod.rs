mod build_target;
pub use build_target::*;

mod repository;
pub use repository::*;

mod team;
pub use team::*;

mod token;
pub use token::*;

use platform::persistence::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Team);
mongo_doc!(Repository);
mongo_doc!(BuildTarget);
mongo_doc!(Token);
