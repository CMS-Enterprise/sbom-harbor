use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use tracing::{debug, instrument, trace};

use aquia::config::Config;
use aquia::dynamo::Store;
use aquia::http::Error as HttpError;
use aquia::lambdahttp::from_entity;
use uuid::Uuid;
use crate::api;

use crate::entities::Team;


/// Implements the ServiceFn<T> interface expected by the Lambda runtime.
/// Provides protocol level validation, and then delegates to request handler.
#[instrument]
pub async fn post_team_handler(req: Request) -> Result<Response<Body>, Error> {
    let request: Option<Team> = req.payload()?;

    let request = match request {
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Text("missing payload".to_string()))
                .map_err(Box::new)?);
        }
        Some(r) => r,
    };

    if let Err(err) = validate(&request) {
        debug!("{}", err);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text(err.to_string()))
            .map_err(Box::new)?);
    };

    let response = handle_request(request).await?;

    trace!("complete with response: {:?}", response);

    let response: api::Team = match response {
        None => {
            return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("BAD REQUEST".to_string()))
            .map_err(Box::new)?);
        }
        Some(r) => from_entity(&r).unwrap(),
    };

    let json = serde_json::to_vec(&response)?;
    let body = Body::from(json);

    let response: Response<Body> = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new)?;

    Ok(response)
}

/// Create a new team.
pub async fn handle_request(request: Team) -> Result<Option<Team>, Error> {
    let config = Config::new().await?;
    let store = Store::new(config);

    let mut request = request.clone();
    request.id = Uuid::new_v4().to_string();

    store.insert::<Team>(&mut request.clone()).await?;
    Ok(Some(request))
}

pub(crate) fn validate(request: &Team) -> Result<(), Error> {
    if !request.id.is_empty() {
        return Err(Error::try_from(HttpError::PostError("attempt to create existing entity".to_string())).unwrap());
    }

    if request.name.is_empty() {
        return Err(Error::try_from(HttpError::PostError("missing required attribute 'name'".to_string())).unwrap());
    }

    Ok(())
}
