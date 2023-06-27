use crate::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Health check handler.
pub async fn get() -> Result<Response, Error> {
    let response = healthy();
    Ok(response.into_response())
}

fn healthy() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("content-type", "text/plain")],
        String::from("OK"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_get_healthy_response() -> Result<(), Error> {
        let response = healthy();
        let response = response.into_response();

        assert_eq!(StatusCode::OK, response.status());
        assert!(response.headers().contains_key("content-type"));

        let header_val = response.headers().get("content-type");

        match header_val {
            None => {
                return Err(Error::InternalServerError(
                    "content-type not set".to_string(),
                ));
            }
            Some(content_type) => {
                assert_eq!(
                    "text/plain",
                    content_type
                        .to_str()
                        .map_err(|e| Error::InternalServerError(e.to_string()))?
                );
            }
        }

        let body = platform::hyper::body::to_string(response.into_body()).await?;

        assert_eq!(body.as_str(), "OK");

        Ok(())
    }
}
