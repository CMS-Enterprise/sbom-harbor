use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Result};
use hyper::{Body, Client, Method, Request, StatusCode, Uri};
use hyper_rustls::HttpsConnectorBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;

const CONTENT_TYPE: &str = "content-type";

/// ContentType is used to configure the content type request header.
pub enum ContentType {
    /// Content is application/x-www-form-urlencoded
    FormUrlEncoded,
    /// Content is application/json
    Json,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::FormUrlEncoded => write!(f, "application/x-www-form-urlencoded"),
            ContentType::Json => write!(f, "application/json")
        }
    }
}

/// Performs a GET request to the specified URL.
///
/// This function is a convenience wrapper around [request<T, U>].
pub async fn get<T: Serialize, U: DeserializeOwned>(
    url: &str,
    token: &str,
    payload: Option<T>,
    content_type: Option<ContentType>,
) -> Result<Option<U>> {
    request(Method::GET, url, String::from(token), payload, content_type).await
}

/// Performs a POST request to the specified URL.
///
/// This function is a convenience wrapper around [request<T, U>].
pub async fn post<T: Serialize, U: DeserializeOwned>(
    url: &str,
    token: &str,
    payload: Option<T>,
    content_type: Option<ContentType>,
) -> Result<Option<U>> {
    request(Method::POST, url, String::from(token), payload, content_type).await
}

/// Performs a DELETE request to the specified URL.
///
/// This function is a convenience wrapper around [request<T, U>].
pub async fn delete<T: Serialize, U: DeserializeOwned>(
    url: &str,
    token: &str,
    payload: Option<T>,
    content_type: Option<ContentType>,
) -> Result<Option<U>> {
    request(Method::DELETE, url, String::from(token), payload, content_type).await
}

/// Performs an HTTP request with the specified HTTP Method.
///
/// Token is optional. Due to type constraints callers must specify
/// a type that implements [serde::Serialize] even when passing [None]
/// as the payload.
pub async fn request<T: Serialize, U: DeserializeOwned>(
    method: Method,
    url: &str,
    token: String,
    payload: Option<T>,
    content_type: Option<ContentType>,
) -> Result<Option<U>> {
    let uri: Uri = Uri::try_from(url)?;

    let content_type = match content_type {
        None => ContentType::Json,
        Some(c) => c,
    };

    let req_body: Body = match payload {
        Some(p) => {
            let body = match content_type {
                ContentType::FormUrlEncoded => serde_urlencoded::to_string(p)?,
                ContentType::Json => serde_json::to_string(&p)?,
            };

            Body::from(body)
        }
        None => Body::empty(),
    };

    let mut req: Request<Body> = Request::builder()
        .method(method)
        .uri(uri)
        .header(CONTENT_TYPE, content_type.to_string())
        .body(req_body)?;

    if !token.is_empty() {
        req.headers_mut().append("Authorization", token.parse()?);
    }

    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http2()
        .build();

    let client: Client<_, hyper::Body> = Client::builder().build(https);

    let resp = match client.request(req).await {
        Ok(r) => r,
        Err(err) => {
            let msg = format!("error making request: {}", err);
            return Err(anyhow!(msg));
        }
    };

    let resp_status = resp.status();
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let resp_body = match String::from_utf8(resp_body.to_vec()) {
        Ok(body) => body,
        Err(err) => {
            return Err(anyhow!(format!("error parsing error response: {}", err)));
        }
    };

    if resp_status != StatusCode::OK {
        let msg = format!("error processing request: {}", resp_body);
        return Err(anyhow!(format!("{}", msg)));
    }

    // TODO: This a hack around how the API returns empty JSON.
    if resp_body.eq("{}") {
        return Ok(None);
    }

    let result = match serde_json::from_slice(resp_body.as_ref()) {
        Ok(r) => r,
        Err(err) => {
            let msg = format!("error parsing response: {} - {}", err, resp_body);
            return Err(anyhow!(msg));
        }
    };

    Ok(Some(result))
}
