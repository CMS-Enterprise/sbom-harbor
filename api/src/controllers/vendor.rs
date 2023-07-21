use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use harbcore::entities::vendors::Vendor;
use harbcore::services::vendors::VendorService;
use platform::persistence::mongodb::{Service, Store};
use tracing::instrument;

use crate::auth::Claims;
use crate::Error;

/// Arc reference type to register with Axum.
pub type DynVendorService = Arc<VendorService>;

/// Factory method for new instance of type.
pub fn new(store: Arc<Store>) -> Arc<VendorService> {
    Arc::new(VendorService::new(store))
}

// WATCH: Trying to get by without a custom extractor.
/// Get a [Vendor] by id.
#[instrument]
#[debug_handler]
pub async fn get(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynVendorService>,
) -> Result<Json<Vendor>, Error> {
    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    let vendor = service
        .find(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    match vendor {
        None => Err(Error::DoesNotExist(format!("vendor not found: {}", id))),
        Some(t) => Ok(Json(t)),
    }
}

/// List all [Vendors].
#[instrument]
#[debug_handler]
pub async fn list(
    _claims: Claims,
    State(service): State<DynVendorService>,
) -> Result<Json<Vec<Vendor>>, Error> {
    let vendors = service
        .list()
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(vendors))
}

/// Post a new [Vendor].
#[instrument]
#[debug_handler]
pub async fn post(
    _claims: Claims,
    State(service): State<DynVendorService>,
    Json(vendor): Json<Vendor>,
) -> Result<Json<Vendor>, Error> {
    if !vendor.id.is_empty() {
        return Err(Error::InvalidParameters(
            "client generated id invalid".to_string(),
        ));
    }

    let mut vendor = vendor;

    service
        .insert(&mut vendor)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(vendor))
}

/// Put an updated [Vendor].
#[instrument]
#[debug_handler]
pub async fn put(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynVendorService>,
    Json(vendor): Json<Vendor>,
) -> Result<Json<Vendor>, Error> {
    if id != vendor.id {
        return Err(Error::InvalidParameters("id mismatch".to_string()));
    }

    let vendor = vendor;

    service
        .update(&vendor)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(vendor))
}

/// Delete and existing [Vendor].
#[instrument]
#[debug_handler]
pub async fn delete(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<DynVendorService>,
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
