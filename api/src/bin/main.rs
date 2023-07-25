use harbcore::config::dev_context;
use harbor_api::app::{app, Config};
use std::net::SocketAddr;
use tracing::{info, trace};

#[tokio::main]
async fn main() {
    // TODO: Dynamically load config
    let cx = match dev_context(None) {
        Ok(cx) => cx,
        Err(e) => {
            trace!("unable to retrieve connection config: {}", e);
            return;
        }
    };

    let harbor = app(Config::new(cx)).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    info!("harbor listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(harbor.into_make_service())
        .await
        .expect("failed to start harbor");
}
