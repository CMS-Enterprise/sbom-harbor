use crate::Error;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a timestamp in seconds since `UNIX_EPOCH`.
pub fn timestamp() -> Result<u64, Error> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Time(e.to_string()))?;

    Ok(duration.as_secs())
}

/// Generates an rfc3339 time stamp which is an implementation of
/// a iso8601 timestamp.
pub fn iso8601_timestamp() -> Result<Option<String>, Error> {
    let unix_timestamp: u64 = timestamp()?;

    // if the u64 is greater than the maximum value an i64 can hold
    // (which is 9223372036854775807), it will wrap around and
    // become a negative number. A unix timestamp will never be
    // this large however, so a simple test should make sure that
    // we get a reasonable value for the timestamp.
    let i64_unix_ts = unix_timestamp as i64;
    if i64_unix_ts <= 0 {
        return Err(Error::Time(format!(
            "Unix timestamp is too large: {}",
            unix_timestamp
        )));
    }

    let naive_datetime = NaiveDateTime::from_timestamp_opt(i64_unix_ts, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    // ISO 8601 allows some freedom over the syntax and RFC 3339
    // exercises that freedom to rigidly define a fixed format.
    Ok(Some(datetime.to_rfc3339()))
}
