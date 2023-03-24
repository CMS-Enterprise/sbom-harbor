use platform::config;
use crate::Error;

pub fn db_connection() -> Result<String, Error> {
    std::env::var("DB_CONNECTION")
        .or_else(|_| Ok("mongodb://localhost:27017".to_string()))
}
