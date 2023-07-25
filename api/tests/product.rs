use crate::common::{as_body, Scenario};
use axum::http::{self, StatusCode};
use harbcore::entities::products::Product;
use harbcore::entities::vendors::Vendor;
use harbor_api::Error;
use http::Method;

mod common;

#[async_std::test]
async fn can_crud_product() -> Result<(), Error> {
    let scenario = Scenario::new().await?;

    // Create a vendor.
    let mut vendor = Vendor::new("api-product-test".to_string())?;
    scenario.with_entity(&mut vendor).await?;
    assert!(!vendor.id.is_empty());

    // Create a product
    let product = Product::new(
        format!("api-product-test-{}", vendor.id),
        "1.0.0".to_string(),
        vendor.id.clone(),
    )?;

    let body = as_body(&product)?;

    // POST the Product.
    let response = scenario
        .response(Method::POST, "/v1/product", Some(body))
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let product: Product = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert!(!product.id.is_empty());

    let product_id = product.id.clone();

    // GET the Product.
    let product_url_with_id = format!("/v1/product/{}", product.id);
    let response = scenario
        .response(Method::GET, product_url_with_id.as_str(), None)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let mut product: Product = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert_eq!(product.id, product_id);

    // LIST the products
    let response = scenario.response(Method::GET, "/v1/products", None).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let products: Vec<Product> = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert!(products.iter().any(|p| p.id == product_id));

    // PUT the Product.
    product.version = "2.0.0".to_string();
    let body = as_body(&product)?;
    let response = scenario
        .response(Method::PUT, product_url_with_id.as_str(), Some(body))
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let product: Product = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert_eq!(product.version, "2.0.0".to_string());

    // DELETE THE PRODUCT
    let response = scenario
        .response(Method::DELETE, product_url_with_id.as_str(), None)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    // LIST the products again and make sure the id does not exist.
    let response = scenario.response(Method::GET, "/v1/products", None).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = platform::hyper::body::to_string(response.into_body()).await?;
    let products: Vec<Product> = serde_json::from_str(body.as_str())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    assert!(!products.iter().any(|p| p.id == product_id));

    // Cleanup the vendor.
    scenario.cleanup(vendor).await?;

    Ok(())
}
