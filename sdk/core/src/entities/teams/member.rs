use platform::auth::User;
use serde::{Deserialize, Serialize};

/// A Member is an entity representing a Harbor User who can manage projects, codebases, tokens, and SBOMs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    /// The unique identifier for the Member.
    pub id: String,
    /// The email address for the Member.
    pub email: String,
}

impl From<User> for Member {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
        }
    }
}
