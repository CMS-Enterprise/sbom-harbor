use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use axum::http::{Method, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderName};
use tracing::{info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use harbcore::config::db_connection;

use platform::mongodb::Store;
use harbor_api::controllers;
// use harbcore::config::sdk_config_from_env;

const X_API_KEY: &str = "x-api-key";
const X_AMZ_DATE: &str = "x-amz-date";

#[tokio::main]
async fn main() {
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
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            api_key,
            amz_date,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH
        ]);

    let tracer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, span: &Span| {
            trace!("request:\n\tspan: {:?}\n\turi: {}",
                span.id(),
                request.uri())
        })
        .on_response(|response: &Response, latency: Duration, span: &Span| {
            trace!("response:\n\tspan: {:?}\n\tstatus: {}\n\tlatency: {}ms",
                span.id(),
                response.status(),
                latency.as_millis())
        })
        .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
            trace!("failure:\n\tspan: {:?}\n\terror: {}\n\tlatency: {}ms",
                span.id(),
                error,
                latency.as_millis())
            },
        );

    // Load injectable types.
    // let config = sdk_config_from_env().await.expect("failed to load config from environment");
    // let authorizer = Authorizer::new(&config).unwrap().expect("failed to load authorizer");
    let cx = db_connection();

    if cx.is_err() {
        trace!("unable to retrieve connection config: {}", cx.err().unwrap());
        return;
    }

    let store = Store::new(&cx.unwrap()).await.unwrap();

    let team_service = controllers::team::new_service(Arc::new(store));

    let harbor = Router::new()
        .fallback(handler_404)
        .route("/teams", get(controllers::team::list))
        .route("/team/:id", get(controllers::team::get))
        .route("/team", post(controllers::team::post))
        .route("/team/:id", put(controllers::team::put))
        .route("/team/:id", delete(controllers::team::delete))
        .with_state(team_service)
        .layer(cors)
        .layer(tracer);

    let addr = SocketAddr::from(([127,0,0,1],6000));
    info!("harbor listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(harbor.into_make_service())
        .await
        .expect("failed to start harbor");
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT FOUND")
}
