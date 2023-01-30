use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::{Discriminator, Team};

/// A Member is an entity representing a Harbor User who can submit SBOMs.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Member {
    #[serde(rename = "TeamId")]
    pub partition_key: String,
    #[serde(rename = "EntityKey")]
    pub sort_key: String,
    /// The id of the Team of which this user is a member.
    #[serde(rename = "parentId")]
    pub parent_id: String,

    /// The unique identifier for the Member.
    pub id: String,
    /// The email address for the Member.
    pub email: String,
    // TODO: Consider roles
    /// Flag indicating whether the member is a team lead.
    #[serde(rename = "isTeamLead")]
    pub is_team_lead: bool,
}

impl Member {
    pub fn new(parent: &Team, email: String, is_team_lead: bool) -> Self {
        let id = Uuid::new_v4().to_string();

        Self {
            partition_key: parent.partition_key.clone(),
            sort_key: Discriminator::Member.to_sort_key(&id).unwrap(),
            parent_id: parent.id.clone(),
            id,
            email,
            is_team_lead,
        }
    }
}
