use crate::Error;

pub fn url_encode(value: &str) -> String {
    urlencoding::encode(value).to_string()
}

pub fn url_decode(value: &str) -> Result<String, Error> {
    Ok(urlencoding::decode(value)
        .map_err(|e| Error::Encoding(e.to_string()))?
        .to_string())
}
