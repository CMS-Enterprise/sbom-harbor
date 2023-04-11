use async_trait::async_trait;

mod snyk;
pub mod github;

#[async_trait]
pub trait SbomProvider {
    async fn provide_sboms(&self);
}