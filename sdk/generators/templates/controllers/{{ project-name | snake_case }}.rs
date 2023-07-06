use std::sync::Arc;
use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use tracing::instrument;

{% if request_type != "()" %}
use harbcore::entities::{{ request_type }};
use harbcore::services::{{ request_type }}Service;
use platform::persistence::mongodb::{Service, Store};
{% endif %}
{% if response_type != "()" %}
use harbcore::entities::{{ response_type }};
{% endif %}

use crate::auth::Claims;
use crate::Error;

{% if response_type != "()" %}
pub type Dyn{{ response_type }}Service = Arc<{{ response_type }}Service>;

pub fn new(store: Arc<Store>) -> Arc<{{ response_type }}Service> {
    Arc::new({{ response_type }}Service::new(store.clone()))
}
{% endif %}

{% case operation_type %}
    {% when "get" %}
#[instrument]
#[debug_handler]
pub async fn get(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<Dyn{{ response_type }}Service>) -> Result<Json<{{ response_type }}>, Error> {

    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    let result = service
        .find(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    match result {
        None => Err(Error::DoesNotExist(format!("entity not found: {}", id))),
        Some(t) => Ok(Json(t)),
    }
}
    {% when "post" %}
#[instrument]
#[debug_handler]
pub async fn post(
    _claims: Claims,
    State(service): State<Dyn{{ response_type }}Service>,
    Json({{ response_type | downcase }})Insert: Json<{{ response_type }}>) -> Result<Json<{{
response_type }}>, Error> {

    if !{{ response_type | downcase }}.id.is_empty() {
        return Err(Error::InvalidParameters("client generated id invalid".to_string()));
    }

    let mut {{ response_type | downcase}} = {{ response_type | downcase }};

    service
        .insert(&mut {{ response_type | downcase}})
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json({{ response_type | downcase }}))
}
    {% when "put" %}
#[instrument]
#[debug_handler]
pub async fn put(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<Dyn{{ response_type }}Service>,
    Json({{ response_type | downcase }}): Json<{{ response_type }}>) -> Result<Json<{{ response_type }}>, Error> {

    if id != {{ response_type | downcase }}.id {
        return Err(Error::InvalidParameters("id mismatch".to_string()));
    }

    let mut {{ response_type | downcase }} = {{ response_type | downcase }};

    service
        .update(&mut {{ response_type | downcase }})
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json({{ response_type | downcase }}))
}
    {% when "delete" %}
#[instrument]
#[debug_handler]
pub async fn delete(
    _claims: Claims,
    Path(id): Path<String>,
    State(service): State<Dyn{{ response_type }}Service>) -> Result<Json<()>, Error> {

    if id.is_empty() {
        return Err(Error::InvalidParameters("id invalid".to_string()));
    }

    service
        .delete(id.as_str())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(()))
}
    {% when "list" %}
#[instrument]
#[debug_handler]
pub async fn list(
    _claims: Claims,
    State(service): State<Dyn{{ response_type }}Service>) -> Result<Json<Vec<{{ response_type }}>>, Error> {

    let result = service
        .list()
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Json(result))
}
    {% when "custom" %}
#[instrument]
#[debug_handler]
pub async fn custom(
    _claims: Claims) -> Result<Json<Vec<{{ response_type }}>>, Error> {

    todo!()
}
{% endcase %}


