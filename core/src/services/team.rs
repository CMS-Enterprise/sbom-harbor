use platform::mongodb::{Context, Service};
use std::sync::Arc;

use crate::models::teams::Team;

/// Contains all domain and transaction logic related to [Teams] and their subordinate entities.
#[derive(Debug)]
pub struct TeamService {
    cx: Context,
}

impl TeamService {
    /// Factory method to create new instance of type.
    pub fn new(cx: Context) -> TeamService {
        TeamService { cx }
    }
}

impl Service<Team> for TeamService {
    fn cx(&self) -> &Context { &self.cx }
}
