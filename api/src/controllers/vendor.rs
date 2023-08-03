use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use harbcore::entities::vendors::Vendor;
use harbcore::services::vendors::VendorService;
use platform::persistence::mongodb::{Service, Store};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
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

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct VendorInsert {
    /// The name of the Vendor.
    pub name: Option<String>,
}

impl VendorInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<Vendor, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::InvalidParameters("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        Vendor::new(name).map_err(|e| Error::InvalidParameters(e.to_string()))
    }
}

/// Post a new [Vendor].
#[instrument]
#[debug_handler]
pub async fn post(
    _claims: Claims,
    State(service): State<DynVendorService>,
    Json(vendor): Json<VendorInsert>,
) -> Result<Json<Vendor>, Error> {
    let mut vendor = vendor.to_entity()?;

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
