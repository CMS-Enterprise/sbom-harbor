use crate::clients::snyk::models::OrgV1;

/// ## Overview
/// This module provides a lightweight partial OpenAPI client for the Snyk API. It is primarily used for
///
/// - Using Snyk as an SBOM Provider
/// - Using Snyk as Enrichment Provider.
///
/// ## SBOM Provider
///
/// Example
/// ```rust
///
/// ```
///
/// ## Enrichment Provider
///
/// Example
/// ```rust
/// 
/// ```

/// A lightweight Snyk OpenAPI client.
pub mod client;

/// Rust structs that represent the models/schemas relevant to the endpoints the client supports from
/// the Snyk OpenAPI specification.
pub mod models;

#[cfg(test)]
mod tests {
    use crate::clients::snyk::client::{Client, org_groups};
    use crate::Error;

    #[async_std::test]
    async fn can_list_orgs_and_groups() -> Result<(), Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        println!("token: {}", token);

        let client = Client::new(token);

        let orgs = client.orgs().await?;

        assert!(orgs.is_some());

        let orgs = orgs.unwrap();
        assert!(orgs.len() > 0);

        let groups = org_groups(orgs);
        assert!(groups.len() > 0);

        Ok(())
    }
}