## Entities

**Entities** are types that either define the data elements being observed in the Harbor problem 
domain, or support interaction across services, both internal and external. Most entities are 
persisted as entries in a data store. However, ome entities are materialized at runtime only for 
the purpose of executing domain logic, and are never serialized or persisted. Entities are typically 
managed by one or more **services**.

Entities may also contain validation or helper functions that can operate on an instance of an entity.
Consider the following. In this example, the `Token` entity defines a function (`expired`) that 
provides a consistent, functional mechanism for determining if a `Token` instance has expired.

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use harbcore::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub id: String,
    pub name: String,
    pub token: String,
    pub enabled: bool,
    pub expires: String,
}

impl Token {
    pub fn expired(&self) -> Result<bool, Error> {
        if self.expires.is_empty() {
            return Ok(false);
        }

        match DateTime::parse_from_rfc3339(&self.expires) {
            Ok(expiry) => Ok(Utc::now() >= expiry),
            Err(err) => Err(Error::Runtime(format!("error parsing token expires: {}", err.to_string()))),
        }
    }
}
```
