mod github;

pub trait SbomProvider {
    async fn provide_sboms(&self);
}