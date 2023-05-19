/// Harbor custom CVSS data model.
pub mod cvss;

/// Persistence and domain logic relevant to managing [Vulnerability] instances associated with a
/// [Package].
pub mod vulnerabilities;

use platform::mongo_doc;
use platform::mongodb::MongoDocument;
pub use vulnerabilities::*;

mongo_doc!(Vulnerability);
