use axum::http::header::AUTHORIZATION;
use axum::{
    body::Body,
    http::{self, Request},
    Router,
};
use harbcore::config::dev_context;
use harbcore::testing::sbom_fixture_path;
use harbor_api::app::{app, Config};
use harbor_api::Error;
use http::Method;
use mime;
use platform::persistence::mongodb::{MongoDocument, Store};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

pub struct Scenario {
    router: Router,
    store: Arc<Store>,
}

impl Scenario {
    pub async fn new() -> Result<Scenario, Error> {
        let cx = dev_context(None)?;
        let router = router().await?;
        let store = Arc::new(Store::new(&cx).await?);

        Ok(Self { router, store })
    }

    pub async fn response(
        &self,
        method: Method,
        route: &str,
        body: Option<Body>,
    ) -> Result<axum::response::Response, Error> {
        let router = self.router.clone();
        let request = request(method, route, body);

        let response = router
            .oneshot(request)
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(response)
    }

    #[allow(dead_code)]
    pub async fn with_entity<E>(&self, entity: &mut E) -> Result<(), Error>
    where
        E: MongoDocument,
    {
        entity.set_id(Uuid::new_v4().to_string());

        self.store
            .insert::<E>(entity)
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))
    }

    #[allow(dead_code)]
    pub async fn cleanup<E>(&self, entity: E) -> Result<(), Error>
    where
        E: MongoDocument,
    {
        let id = entity.id();
        self.store
            .delete::<E>(id.as_str())
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))
    }
}
#[allow(dead_code)]
pub async fn router() -> Result<Router, Error> {
    let cx = match dev_context(None) {
        Ok(cx) => cx,
        Err(e) => {
            return Err(Error::InternalServerError(e.to_string()));
        }
    };

    let harbor = app(Config::new(cx)).await;
    Ok(harbor)
}

#[allow(dead_code)]
pub fn request(method: Method, route: &str, body: Option<Body>) -> Request<Body> {
    let body = match body {
        None => Body::from(""),
        Some(b) => b,
    };

    Request::builder()
        .method(method)
        .uri(route)
        .header(AUTHORIZATION, "Bearer 123")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(body)
        .unwrap()
}

#[allow(dead_code)]
pub fn raw_sbom() -> Result<String, Error> {
    std::fs::read_to_string(sbom_fixture_path()?)
        .map_err(|e| Error::InternalServerError(e.to_string()))
}

#[allow(dead_code)]
pub fn sbom_as_body() -> Result<Body, Error> {
    let raw = raw_sbom()?;

    Ok(Body::from(
        serde_json::to_vec(&json!(raw)).map_err(|e| Error::InternalServerError(e.to_string()))?,
    ))
}

#[allow(dead_code)]
pub fn as_body<T>(instance: &T) -> Result<Body, Error>
where
    T: Serialize,
{
    Ok(Body::from(
        serde_json::to_vec(instance).map_err(|e| Error::InternalServerError(e.to_string()))?,
    ))
}
