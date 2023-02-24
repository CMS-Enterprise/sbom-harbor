use serde::{Deserialize, Serialize};
use aqum::auth::User;

/// A Member is an entity representing a Harbor User who can manage projects, codebases, tokens, and SBOMs.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Member {
    /// The unique identifier for the Member.
    #[serde(rename = "_id")]
    pub id: String,
    /// The email address for the Member.
    pub email: String,
}

impl Member {
    pub fn new(email: String) -> Self {
        Self {
            id: "".to_string(),
            email,
        }
    }
}

impl From<User> for Member {
    fn from(value: User) -> Self {
        Self{
            id: value.id,
            email: value.email,
        }
    }
}
