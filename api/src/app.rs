use axum::http::header::{HeaderName, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{Method, Request};
use axum::response::{IntoResponse, Response};
use axum::Router;
use harbcore::services::packages::PackageService;
use harbcore::services::products::ProductService;
use harbcore::services::sboms::{FileSystemStorageProvider, SbomService};
use harbcore::services::teams::TeamService;
use harbcore::services::vendors::VendorService;
use platform::hyper::StatusCode;
use platform::persistence::mongodb::{Context, Store};
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::trace;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::controllers;

const X_API_KEY: &str = "x-api-key";
const X_AMZ_DATE: &str = "x-amz-date";

/// Dynamic application configuration.
pub struct Config {
    cx: Context,
}

impl Config {
    /// Factory method for new instance of type.
    pub fn new(cx: Context) -> Config {
        Config { cx }
    }
}

/// Factory method for new instance of application route handler.
pub async fn app(config: Config) -> Router {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "harbor=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_key = HeaderName::from_static(X_API_KEY);
    let amz_date = HeaderName::from_static(X_AMZ_DATE);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, api_key, amz_date])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
        ]);

    let tracer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, span: &Span| {
            trace!(
                "request:\n\tspan: {:?}\n\turi: {}",
                span.id(),
                request.uri()
            )
        })
        .on_response(|response: &Response, latency: Duration, span: &Span| {
            trace!(
                "response:\n\tspan: {:?}\n\tstatus: {}\n\tlatency: {}ms",
                span.id(),
                response.status(),
                latency.as_millis()
            )
        })
        .on_failure(
            |error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
                trace!(
                    "failure:\n\tspan: {:?}\n\terror: {}\n\tlatency: {}ms",
                    span.id(),
                    error,
                    latency.as_millis()
                )
            },
        );

    // Load injectable types.
    // let authorizer = Authorizer::new(&config).unwrap().expect("failed to load authorizer");
    let store = Arc::new(Store::new(&config.cx).await.unwrap());

    let packages = PackageService::new(store.clone());
    // TODO: Inject StorageProvider from config.
    let sboms = Arc::new(SbomService::new(
        store.clone(),
        Some(Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/sboms".to_string(),
        ))),
        Some(packages),
    ));

    let teams = Arc::new(TeamService::new(store.clone()));
    let vendors = Arc::new(VendorService::new(store.clone()));
    let products = Arc::new(ProductService::new(
        store,
        Some(vendors.clone()),
        Some(sboms),
    ));

    Router::new()
        .fallback(handler_404)
        .route("/health", axum::routing::get(controllers::health::get))
        .route("/v1/teams", axum::routing::get(controllers::team::list))
        .route("/v1/team/:id", axum::routing::get(controllers::team::get))
        .route("/v1/team", axum::routing::post(controllers::team::post))
        .route("/v1/team/:id", axum::routing::put(controllers::team::put))
        .route(
            "/v1/team/:id",
            axum::routing::delete(controllers::team::delete),
        )
        .with_state(teams)
        .route("/v1/vendors", axum::routing::get(controllers::vendor::list))
        .route(
            "/v1/vendor/:id",
            axum::routing::get(controllers::vendor::get),
        )
        .route("/v1/vendor", axum::routing::post(controllers::vendor::post))
        .route(
            "/v1/vendor/:id",
            axum::routing::put(controllers::vendor::put),
        )
        .route(
            "/v1/vendor/:id",
            axum::routing::delete(controllers::vendor::delete),
        )
        .with_state(vendors)
        .route(
            "/v1/products",
            axum::routing::get(controllers::product::list),
        )
        .route(
            "/v1/product/:id",
            axum::routing::get(controllers::product::get),
        )
        .route(
            "/v1/product",
            axum::routing::post(controllers::product::post),
        )
        .route(
            "/v1/product/:id",
            axum::routing::put(controllers::product::put),
        )
        .route(
            "/v1/product/:id",
            axum::routing::delete(controllers::product::delete),
        )
        .route(
            "/v1/product/:id/sbom",
            axum::routing::post(controllers::product::sbom),
        )
        .with_state(products)
        .layer(cors)
        .layer(tracer)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT FOUND")
}
