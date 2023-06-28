use super::product::Product;
use crate::entities::teams::Token;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

///  A Vendor is an entity that sells software or software services. Vendors are required to
/// supply SBOMs for the products they sell.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Vendor {
    /// The unique identifier for the Vendor.
    pub id: String,

    /// The name of the Vendor.
    pub name: String,

    /// Products that are owned by the Vendor.
    pub products: Option<HashMap<String, Product>>,

    /// Tokens that are associated with the Vendor.
    pub tokens: Option<HashMap<String, Token>>,
}

impl Vendor {
    /// Factory method to create new instance of type.
    pub fn new(name: String) -> Result<Vendor, Error> {
        if name.is_empty() {
            return Err(Error::Entity("name required".to_string()));
        }

        if name.len() < 2 {
            return Err(Error::Entity(
                "name must be at least 2 characters in length".to_string(),
            ));
        }

        Ok(Vendor {
            id: "".to_string(),
            name,
            products: None,
            tokens: None,
        })
    }

    /// Add a product to the products Vector.
    pub fn products(&mut self, _product: Product) -> &Self {
        // TODO:
        self
    }

    /// Determines if the specified product id is owned by an instance of a Vendor.
    pub fn owns_product(&self, product_id: String) -> bool {
        match &self.products {
            None => false,
            Some(products) => products.contains_key(product_id.as_str()),
        }
    }

    /// Add a product to the products Vector.
    pub fn tokens(&mut self, _token: Token) -> &Self {
        // TODO:
        self
    }
}

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct VendorInsert {
    /// The name of the Vendor.
    pub name: Option<String>,
}

impl VendorInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<Vendor, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::Entity("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        Vendor::new(name)
    }
}
