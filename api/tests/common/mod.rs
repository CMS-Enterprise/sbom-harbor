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
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

pub struct Scenario {
    router: Router,
    store: Arc<Store>,
    method: Method,
}

impl Scenario {
    pub async fn new(method: Method) -> Result<Scenario, Error> {
        let cx = dev_context(None)?;
        let router = router().await?;
        let store = Arc::new(Store::new(&cx).await?);

        Ok(Self {
            router,
            store,
            method,
        })
    }

    pub async fn response(
        &self,
        route: String,
        body: Option<Body>,
    ) -> Result<axum::response::Response, Error> {
        let router = self.router.clone();
        let request = request(self.method.clone(), route, body);

        let response = router
            .oneshot(request)
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(response)
    }

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

pub fn request(method: Method, route: String, body: Option<Body>) -> Request<Body> {
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

pub fn raw_sbom() -> Result<String, Error> {
    std::fs::read_to_string(sbom_fixture_path()?)
        .map_err(|e| Error::InternalServerError(e.to_string()))
}

pub fn sbom_as_body() -> Result<Body, Error> {
    let raw = raw_sbom()?;

    Ok(Body::from(
        serde_json::to_vec(&json!(raw)).map_err(|e| Error::InternalServerError(e.to_string()))?,
    ))
}
