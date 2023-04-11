use async_trait::async_trait;

//mod github;
mod snyk;

#[async_trait]
pub trait SbomProvider {
    async fn provide_sboms(&self);
}