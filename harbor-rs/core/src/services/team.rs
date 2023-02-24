use aqum::mongodb::{Service, Store};
use std::sync::Arc;

use crate::models::Team;

#[derive(Debug)]
pub struct TeamService {
    store: Arc<Store>,
}

impl TeamService {
    pub fn new(store: Arc<Store>) -> TeamService {
        TeamService { store }
    }
}

impl Service<Team> for TeamService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

// #[derive(Clone, Debug)]
// pub struct ListTeamsContext {
//     pub children: bool,
// }
//
// impl ListTeamsContext {
//     pub fn new(children: bool) -> Self {
//         Self { children}
//     }
// }
//
// /// TeamContext
// #[derive(Clone, Debug)]
// pub struct TeamContext {
//     /// Id query constraint.
//     pub id: String,
//     /// Flag indicating whether to include children in the result.
//     pub children: bool,
// }
//
// /// CreateTeamContext
// #[derive(Debug)]
// pub struct CreateTeamContext {
//     /// Team model
//     pub team: Team,
//     /// Flag indicating whether to handle children in the operation.
//     pub children: bool,
// }
//
// /// UpdateTeamContext
// #[derive(Debug)]
// pub struct UpdateTeamContext {
//     /// Id query constraint.
//     pub id: String,
//     /// Team model
//     pub team: Team,
//     /// Flag indicating whether to handle children in the operation.
//     pub children: bool,
// }
