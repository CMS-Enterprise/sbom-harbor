use crate::hyper::{ContentType, Error, Method, StatusCode, CONTENT_TYPE};
use hyper::client::HttpConnector;
use hyper::{Body, Client as NativeClient, Request, Uri};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Wrapper type over a native Hyper Client. Allows for consistent, concise instance construction
/// and a conventional set of abstractions over low level methods.
#[derive(Debug)]
pub struct Client {
    inner: NativeClient<HttpsConnector<HttpConnector>, hyper::Body>,
}

impl Default for Client {
    fn default() -> Self {
        Client::new()
    }
}

impl Client {
    /// Factory method to create new instances of type.
    pub fn new() -> Self {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http2()
            .build();

        let inner: NativeClient<_, Body> = NativeClient::builder().build(https);

        Self { inner }
    }

    /// Performs a GET request to the specified URL.
    ///
    /// This function is a convenience wrapper around [request<T, U>].
    pub async fn get<T: Serialize, U: DeserializeOwned>(
        &self,
        url: &str,
        content_type: ContentType,
        token: &str,
        payload: Option<T>,
    ) -> Result<Option<U>, Error> {
        self.request(Method::GET, url, content_type, String::from(token), payload)
            .await
    }

    /// Performs a POST request to the specified URL.
    ///
    /// This function is a convenience wrapper around [request<T, U>].
    pub async fn post<T: Serialize, U: DeserializeOwned>(
        &self,
        url: &str,
        content_type: ContentType,
        token: &str,
        payload: Option<T>,
    ) -> Result<Option<U>, Error> {
        self.request(
            Method::POST,
            url,
            content_type,
            String::from(token),
            payload,
        )
        .await
    }

    /// Performs a DELETE request to the specified URL.
    ///
    /// This function is a convenience wrapper around [request<T, U>].
    pub async fn delete<T: Serialize, U: DeserializeOwned>(
        &self,
        url: &str,
        content_type: ContentType,
        token: &str,
        payload: Option<T>,
    ) -> Result<Option<U>, Error> {
        self.request(
            Method::DELETE,
            url,
            content_type,
            String::from(token),
            payload,
        )
        .await
    }

    /// Performs an HTTP request with the specified HTTP Method.
    ///
    /// Token is optional. Due to type constraints callers must specify
    /// a type that implements [serde::Serialize] even when passing [None]
    /// as the payload.
    pub async fn request<T: Serialize, U: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        content_type: ContentType,
        token: String,
        payload: Option<T>,
    ) -> Result<Option<U>, Error> {
        let result = self.raw(method, url, content_type, token, payload).await?;
        let resp_status = result.0;
        let resp_body = result.1;

        if resp_status != StatusCode::OK {
            return Err(Error::Remote(resp_body));
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

    /// Allows making raw HTTP requests without the opinionated JSON behaviors.
    pub async fn raw<T: Serialize>(
        &self,
        method: Method,
        url: &str,
        content_type: ContentType,
        token: String,
        payload: Option<T>,
    ) -> Result<(StatusCode, String), Error> {
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
            .body(req_body)?;

        if !token.is_empty() {
            req.headers_mut().append("Authorization", token.parse()?);
        }

        let resp = match self.inner.request(req).await {
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

        Ok((resp_status, resp_body))
    }
}

#[cfg(test)]
mod tests {
    use crate::hyper::client::Client;
    use crate::hyper::{ContentType, Error, Method, StatusCode};

    #[async_std::test]
    async fn can_make_get_request() -> Result<(), Error> {
        let client = Client::new();

        let result: (StatusCode, String) = client
            .raw(
                Method::GET,
                "https://api.first.org/data/v1/epss?cve=CVE-2022-27225",
                ContentType::Json,
                "".to_string(),
                None::<String>,
            )
            .await?;

        assert_eq!(result.0, StatusCode::OK);
        assert!(!result.1.is_empty());

        Ok(())
    }
}
