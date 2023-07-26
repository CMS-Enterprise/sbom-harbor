use crate::Error;
use hyper::body::HttpBody;

/// Converts a Hyper `HttpBody` to an owned String.
/// # Errors
///
/// Will return an `Err` if the body cannot be converted to `Bytes` or from `Bytes` to a UTF-8
/// string.
pub async fn to_string<T>(body: T) -> Result<String, Error>
where
    T: HttpBody,
{
    let body = hyper::body::to_bytes(body)
        .await
        .map_err(|_| Error::Http("error parsing body".to_string()))?;

    let body = body.to_vec();

    let result = match std::str::from_utf8(&body) {
        Ok(body) => body.to_string(),
        Err(e) => {
            return Err(Error::Http(e.to_string()));
        }
    };

    Ok(result)
}
