use crate::Error;
use serde::{Deserialize, Serialize};

/// The Product entity represents a specific product and version combination of a software or
/// service a Vendor sells and is required to submit SBOMs for.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    /// The unique identifier for the Product.
    pub id: String,
    /// The name of the product.
    pub name: String,
    /// The version of the product.
    pub version: String,
}

impl Product {
    /// Factory method to create new instance of type.
    pub fn new(name: String, version: String) -> Result<Product, Error> {
        if name.is_empty() {
            return Err(Error::Entity("product name cannot be empty".to_string()));
        }

        if version.is_empty() {
            return Err(Error::Entity("product version cannot be empty".to_string()));
        }

        Ok(Product {
            id: "".to_string(),
            name,
            version,
        })
    }
}
