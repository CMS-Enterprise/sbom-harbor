use harbcore::config::dev_context;
use harbor_api::app::{app, Config};
use std::net::SocketAddr;
use std::env;
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

    // Retrieve the port number from the environment
    // variable, defaulting to 5000 if it isn't set.
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("5000"))
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("harbor listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(harbor.into_make_service())
        .await
        .expect("failed to start harbor");
}
