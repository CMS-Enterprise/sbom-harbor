use crate::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a timestamp in seconds since `UNIX_EPOCH`.
pub fn timestamp() -> Result<u64, Error> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Time(e.to_string()))?;

    Ok(duration.as_secs())
}
