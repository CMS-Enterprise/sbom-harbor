use crate::common::{sbom_as_body, Scenario};
use axum::http::{self, StatusCode};
use harbcore::entities::products::Product;
use harbcore::entities::sboms::Sbom;
use harbcore::entities::vendors::Vendor;
use harbor_api::Error;
use http::Method;

mod common;

#[async_std::test]
async fn can_ingest_sbom() -> Result<(), Error> {
    let body = sbom_as_body()?;
    let scenario = Scenario::new(Method::POST).await?;

    // Create a vendor.
    let mut vendor = Vendor::new("harbor-ingest-test".to_string())?;
    scenario.with_entity(&mut vendor).await?;
    assert!(!vendor.id.is_empty());

    // Create a product
    let mut product = Product::new(
        "harbor-ingest-test".to_string(),
        "1.0.0".to_string(),
        vendor.id.clone(),
    )?;
    scenario.with_entity(&mut product).await?;
    assert!(!product.id.is_empty());

    // Run the route handler.
    let response = scenario
        .response(format!("/v1/product/{}/sbom", product.id), Some(body))
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let sbom: Sbom = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert!(!sbom.id.is_empty());

    // TODO: Create common function to clean up all Packages etc associated with an Sbom on delete.
    scenario.cleanup(sbom).await?;
    scenario.cleanup(product).await?;
    scenario.cleanup(vendor).await?;

    Ok(())
}
