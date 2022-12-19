use serde::Deserialize;

/// An event requesting the synchronization of a Harbor instance with
/// a source control provider.
#[derive(Debug, Deserialize)]
pub struct ImportEvent {}
