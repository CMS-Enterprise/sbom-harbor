use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use tracing::{debug, instrument, trace};

use aquia::config::Config;
use aquia::dynamo::{Entity, Store};
use crate::entities::Team;

/// Implements the ServiceFn<T> interface expected by the Lambda runtime.
/// Provides protocol level validation, and then delegates to request handler.
#[instrument]
pub async fn delete_team_handler(req: Request) -> Result<Response<Body>, Error> {
    let params = req.path_parameters();
    let id = params.first("teamId");

    if let Err(err) = validate(id.clone()) {
        debug!("{}", err);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text(err.to_string()))
            .map_err(Box::new)?);
    };

    let response = handle_request(id.unwrap().to_string()).await?;

    trace!("complete with response: {:?}", response);

    let json = serde_json::to_vec(&response)?;
    let body = Body::from(json);

    let response: Response<Body> = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new)?;

    Ok(response)
}

/// Delete team by id.
pub async fn handle_request(id: String) -> Result<(), Error> {
    let config = Config::new().await?;
    let store = Store::new(config);

    let mut entity = Team::new("".to_string());
    entity.partition_key = id;

    let result = store.delete::<Team>(entity.key_context()).await?;
    Ok(result)
}

pub(crate) fn validate(id: Option<&str>) -> Result<(), Error> {
    if id.is_none() || id.unwrap().is_empty() {
        return Err(Box::new(aquia::http::Error::ParameterError("teamId is required".to_string())));
    }

    Ok(())
}
