pub mod get_team;
pub mod get_teams;
pub mod post_team;
pub mod put_team;
pub mod delete_team;

// Export handlers to simplify path imports.
pub use crate::team::get_team::get_team_handler;
pub use crate::team::get_teams::get_teams_handler;
pub use crate::team::post_team::post_team_handler;
pub use crate::team::put_team::put_team_handler;
pub use crate::team::delete_team::delete_team_handler;
