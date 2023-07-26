use crate::common::{as_body, Scenario};
use axum::http::{self, StatusCode};
use harbcore::entities::vendors::Vendor;
use harbor_api::Error;
use http::Method;

mod common;

#[async_std::test]
async fn can_crud_vendor() -> Result<(), Error> {
    let scenario = Scenario::new().await?;

    // Create a vendor.
    let vendor = Vendor::new("api-vendor-test".to_string())?;

    let body = as_body(&vendor)?;

    // POST the Vendor.
    let response = scenario
        .response(Method::POST, "/v1/vendor", Some(body))
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let vendor: Vendor = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert!(!vendor.id.is_empty());

    let vendor_id = vendor.id.clone();

    // GET the Vendor.
    let vendor_url_with_id = format!("/v1/vendor/{}", vendor.id);
    let response = scenario
        .response(Method::GET, vendor_url_with_id.as_str(), None)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let mut vendor: Vendor = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert_eq!(vendor.id, vendor_id);

    // LIST the vendors
    let response = scenario.response(Method::GET, "/v1/vendors", None).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let vendors: Vec<Vendor> = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert!(vendors.iter().any(|p| p.id == vendor_id));

    // PUT the Vendor.
    let new_name = format!("{}-{}", vendor.name, vendor.id);
    vendor.name = new_name.clone();
    let body = as_body(&vendor)?;
    let response = scenario
        .response(Method::PUT, vendor_url_with_id.as_str(), Some(body))
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let vendor: Vendor = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert_eq!(vendor.name, new_name);

    // DELETE the Vendor
    let response = scenario
        .response(Method::DELETE, vendor_url_with_id.as_str(), None)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    // LIST the vendors again and make sure the id does not exist.
    let response = scenario.response(Method::GET, "/v1/vendors", None).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let vendors: Vec<Vendor> = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert!(!vendors.iter().any(|p| p.id == vendor_id));

    Ok(())
}
