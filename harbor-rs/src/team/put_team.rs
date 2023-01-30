use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use tracing::{debug, instrument, trace};

use aquia::config::Config;
use aquia::dynamo::Store;
use aquia::http::Error as HttpError;
use aquia::lambdahttp::from_entity;
use crate::api;

use crate::entities::Team;

/// Implements the ServiceFn<T> interface expected by the Lambda runtime.
/// Provides protocol level validation, and then delegates to request handler.
#[instrument]
pub async fn put_team_handler(req: Request) -> Result<Response<Body>, Error> {
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
            .body(Body::Text("BAD_REQUEST".to_string()))
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

// TODO: Document what the request handler does.

pub async fn handle_request(request: Team) -> Result<Option<Team>, Error> {
    let config = Config::new().await?;
    let store = Store::new(config);

    store.update::<Team>(&mut request.clone()).await?;
    Ok(Some(request))
}

pub(crate) fn validate(request: &Team) -> Result<(), Error> {
    let mut errs = String::from("");

    if request.id.is_empty() {
        errs.push_str("missing required attribute 'id'");
    }

    if request.name.is_empty() {
        errs.push_str("missing required attribute 'name'");
    }

    if !errs.is_empty() {
        return Err(Error::try_from(HttpError::PostError(errs)).unwrap());
    }

    Ok(())
}
