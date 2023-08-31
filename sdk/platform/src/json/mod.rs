use axum_extra::json_lines::JsonLines;
use futures_util::stream::{Stream, StreamExt};
use serde_json::{json, Deserializer, Value};

use crate::Error;

pub fn sanitize_ndjson(raw: &str) -> Result<String, Error> {
    // let value = serde_json::to_value(json!(raw))?;
    let mut stream = Deserializer::from_str(raw).into_iter::<Value>();
    // let lines = JsonLines::new(value);
    let mut buf: Value = Value::default();

    while let Some(value) = stream.next() {
        // buf = value.map_err(|e| Error::Json(e.to_string()))?;
        buf = value.unwrap();
    }

    let result = match buf.as_str() {
        None => return Err(Error::Json("json_empty".to_string())),
        Some(b) => b,
    };

    // Recursively call until no more occurrences.
    match result.contains("\n") {
        true => sanitize_ndjson(result),
        false => Ok(result.to_string()),
    }
}
