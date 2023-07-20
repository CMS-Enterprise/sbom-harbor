use std::default::Default;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use axum_extra::json_lines::JsonLines;
use futures_util::stream::StreamExt;
use harbcore::entities::sboms::Sbom;
use harbcore::services::products::ProductService;
use harbcore::services::sboms::SbomService;
use harbcore::services::vendors::VendorService;
use platform::persistence::mongodb::Store;
use serde_json::Value;

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
        println!("{}", body);
        //body.push_str(value.unwrap().to_string());
    }

    let sbom = service
        .ingest(id.as_str(), body.as_str().unwrap())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(sbom))
}
