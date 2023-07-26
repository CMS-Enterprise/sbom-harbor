use std::default::Default;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use axum_extra::json_lines::JsonLines;
use futures_util::stream::StreamExt;
use harbcore::entities::products::Product;
use harbcore::entities::sboms::Sbom;
use harbcore::services::products::ProductService;
use harbcore::services::sboms::SbomService;
use harbcore::services::vendors::VendorService;
use platform::persistence::mongodb::{Service, Store};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use tracing::instrument;

use crate::auth::Claims;
use crate::Error;

/// Arc reference type to register with Axum.
pub type DynProductService = Arc<ProductService>;

/// Factory method for new instance of type.
pub fn new(
    store: Arc<Store>,
    vendors: Option<Arc<VendorService>>,
    sboms: Option<Arc<SbomService>>,
) -> Arc<ProductService> {
    Arc::new(ProductService::new(store, vendors, sboms))
}

// WATCH: Trying to get by without a custom extractor.
/// Get a [Product] by id.
#[instrument]
#[debug_handler]
pub async fn get(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynProductService>,
) -> Result<Json<Product>, Error> {
    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    let product = service
        .find(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    match product {
        None => Err(Error::DoesNotExist(format!("product not found: {}", id))),
        Some(t) => Ok(Json(t)),
    }
}

/// List all [Products].
#[instrument]
#[debug_handler]
pub async fn list(
    _claims: Claims,
    State(service): State<DynProductService>,
) -> Result<Json<Vec<Product>>, Error> {
    let products = service
        .list()
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(products))
}

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct ProductInsert {
    /// The name of the product.
    pub name: Option<String>,

    /// The version of the product.
    pub version: Option<String>,

    /// The unique identifier for the vendor of the [Product].
    pub vendor_id: Option<String>,
}

impl ProductInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<Product, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::InvalidParameters("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        let version = match &self.version {
            None => {
                return Err(Error::InvalidParameters("version required".to_string()));
            }
            Some(version) => version.clone(),
        };

        let vendor_id = match &self.vendor_id {
            None => {
                return Err(Error::InvalidParameters("vendor id required".to_string()));
            }
            Some(vendor_id) => vendor_id.clone(),
        };

        Product::new(name, version, vendor_id).map_err(|e| Error::InvalidParameters(e.to_string()))
    }
}

/// Post a new [Product].
#[instrument]
#[debug_handler]
pub async fn post(
    _claims: Claims,
    State(service): State<DynProductService>,
    Json(product): Json<ProductInsert>,
) -> Result<Json<Product>, Error> {
    let mut product = product.to_entity()?;

    service
        .insert(&mut product)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(product))
}

/// Put an updated [Product].
#[instrument]
#[debug_handler]
pub async fn put(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynProductService>,
    Json(product): Json<Product>,
) -> Result<Json<Product>, Error> {
    if id != product.id {
        return Err(Error::InvalidParameters("id mismatch".to_string()));
    }

    let product = product;

    service
        .update(&product)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(product))
}

/// Delete and existing [Product].
#[instrument]
#[debug_handler]
pub async fn delete(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynProductService>,
) -> Result<Json<()>, Error> {
    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    service
        .delete(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(()))
}

/// Ingests an SBOM and associates it with a [Product].
pub async fn sbom(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynProductService>,
    mut sbom_stream: JsonLines<Value>,
) -> Result<Json<Sbom>, Error> {
    let mut body: Value = Value::default();

    while let Some(value) = sbom_stream.next().await {
        body = value.map_err(|e| Error::InternalServerError(e.to_string()))?;
    }

    let sbom = service
        .ingest(id.as_str(), body.as_str().unwrap())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(sbom))
}
