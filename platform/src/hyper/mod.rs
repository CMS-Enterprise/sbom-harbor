use std::fmt::{Display, Formatter};
use hyper::{Body, Client, Method, Request, StatusCode, Uri};
use hyper::header::InvalidHeaderValue;
use hyper::http::uri::InvalidUri;
use hyper_rustls::HttpsConnectorBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

const CONTENT_TYPE: &str = "content-type";
const USER_AGENT: &str = "User-Agent";

/// HTTP Content Types.
pub enum ContentType {
    /// Form data is sent in a single block in the HTTP message body.
    FormUrlEncoded,
    /// Content sent in JSON format encoded in the UTF-8 character encoding.
    Json,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::FormUrlEncoded => write!(f, "application/x-www-form-urlencoded"),
            ContentType::Json => write!(f, "application/json"),
        }
    }
}

/// Performs a GET request to the specified URL.
///
/// This function is a convenience wrapper around [request<T, U>].
pub async fn get<T: Serialize, U: DeserializeOwned>(
    url: &str,
    content_type: ContentType,
    token: &str,
    payload: Option<T>,
) -> Result<Option<U>, Error> {
    request(Method::GET, url, content_type, String::from(token), payload).await
}

/// Performs a POST request to the specified URL.
///
/// This function is a convenience wrapper around [request<T, U>].
pub async fn post<T: Serialize, U: DeserializeOwned>(
    url: &str,
    content_type: ContentType,
    token: &str,
    payload: Option<T>,
) -> Result<Option<U>, Error> {
    request(Method::POST, url, content_type, String::from(token), payload).await
}

/// Performs a DELETE request to the specified URL.
///
/// This function is a convenience wrapper around [request<T, U>].
pub async fn delete<T: Serialize, U: DeserializeOwned>(
    url: &str,
    content_type: ContentType,
    token: &str,
    payload: Option<T>,
) -> Result<Option<U>, Error> {
    request(Method::DELETE, url, content_type, String::from(token), payload).await
}

/// Performs an HTTP request with the specified HTTP Method.
///
/// Token is optional. Due to type constraints callers must specify
/// a type that implements [serde::Serialize] even when passing [None]
/// as the payload.
pub async fn request<T: Serialize, U: DeserializeOwned>(
    method: Method,
    url: &str,
    content_type: ContentType,
    token: String,
    payload: Option<T>,
) -> Result<Option<U>, Error> {
    let uri: Uri = Uri::try_from(url)?;

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
        .header(USER_AGENT, String::from("SBOM Harbor CLI"))
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
            return Err(Error::Remote(err.to_string()));
        }
    };

    let resp_status = resp.status();
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let resp_body = match String::from_utf8(resp_body.to_vec()) {
        Ok(body) => body,
        Err(err) => {
            return Err(Error::Body(err.to_string()));
        }
    };

    if resp_status != StatusCode::OK {
        return Err(Error::Remote(resp_status, resp_body));
    }

    // TODO: This a hack around empty JSON.
    if resp_body.eq("{}") {
        return Ok(None);
    }

    let result = match serde_json::from_slice(resp_body.as_ref()) {
        Ok(r) => r,
        Err(err) => {
            return Err(Error::Serde(err.to_string()));
        }
    };

    Ok(Some(result))
}

/// Represents all handled Errors for this module.
#[derive(Error, Debug)]
pub enum Error {
    /// Error parsing [Body].
    #[error("error parsing body: {0}")]
    Body(String),
    /// Invalid [Header].
    #[error("invalid header: {0}")]
    InvalidHeader(String),
    /// Invalid [URI].
    #[error("invalid uri: {0}")]
    InvalidUri(String),
    /// Error in [Hyper] runtime.
    #[error("error in hyper runtime: {0}")]
    Hyper(String),
    /// Error calling remote resource.
    #[error("error from remote resource: {0}")]
    Remote(StatusCode, String),
    #[error("error serializing types: {0}")]
    Serde(String),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper(err.to_string())
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Error::Hyper(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        Error::Serde(err.to_string())
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Self {
        Error::InvalidHeader(err.to_string())
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Self {
        Error::InvalidUri(err.to_string())
    }
}
