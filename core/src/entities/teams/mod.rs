mod codebase;
pub use codebase::*;

mod member;
pub use member::*;

mod project;
pub use project::*;

mod team;
pub use team::*;

mod token;
pub use token::*;

use platform::mongodb::{mongo_doc, MongoDocument};

mongo_doc!(Codebase);
mongo_doc!(Member);
mongo_doc!(Project);
mongo_doc!(Team);
mongo_doc!(Token);
