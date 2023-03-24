use lambda_http::{Body, Error, Request, Response};
use lambda_http::http::StatusCode;
use tracing::{instrument, trace};

use aquia::config::Config;
use aquia::dynamo::Store;
use aquia::lambdahttp::from_entity;
use crate::api;

use crate::entities::Team;

/// Implements the ServiceFn<T> interface expected by the Lambda runtime.
/// Provides protocol level validation, and then delegates to request handler.
#[instrument]
pub async fn get_teams_handler(req: Request) -> Result<Response<Body>, Error> {
    let response = handle_request().await?;

    trace!("complete with response: {:?}", response);

    let response: Vec<api::Team> = response.iter()
        .map(|t| {
            return from_entity(t).unwrap();
        })
        .collect();

    let json = serde_json::to_vec(&response)?;
    let body = Body::from(json);

    let response: Response<Body> = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new)?;

    Ok(response)
}

// TODO: Document what the request handler does.

pub async fn handle_request() -> Result<Vec<Team>, Error> {
    let config = Config::new().await?;
    let store = Store::new(config);

    // Entity type required to access schema info.
    // TODO: See if decoupling is the right move.
    let mut query = Team::new("all".to_string());

    let result = store.list::<Team>(&mut query).await?;

    Ok(result)
}
