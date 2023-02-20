use crate::Error;
use std::collections::HashMap;

pub mod team;

/// Retrieves the children flag from the querystring.
pub fn resolve_children(query: HashMap<String, String>) -> bool {
    match query.get("children") {
        None => false,
        Some(c) => {
            match c.is_empty() {
                // if the parameter exists, but is not
                // explicitly set, assume the client is using it as a flag.
                true => true,
                // parse set value.
                false => c.to_lowercase().eq("true")
            }
        }
    }
}

/// Retrieves an id from the querystring by key name.
pub fn resolve_id(id_key: String, query: HashMap<String, String>) -> Result<String, Error> {
    match query.get(id_key.as_str()) {
        None => Err(Error::Entity(format!("missing required parameter {}", id_key))),
        Some(id) => Ok(id.to_string())
    }
}
