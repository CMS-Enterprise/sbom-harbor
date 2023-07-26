use crate::entities::products::Product;
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

    /// Upsert a product to the products Vector.
    pub fn products(&mut self, product: Product) -> Result<&Self, Error> {
        if self.id != product.vendor_id {
            return Err(Error::Entity("invalid product for vendor".to_string()));
        }

        let mut products = match &self.products {
            None => HashMap::new(),
            Some(products) => products.clone(),
        };

        if products.contains_key(product.id.as_str()) {
            products.insert(product.id.clone(), product);
            self.products = Some(products);
            return Ok(self);
        }

        for (_, existing) in products.iter() {
            if existing.name == product.name && existing.version == product.version {
                return Err(Error::Entity("product exists for version".to_string()));
            }
        }

        products.insert(product.id.clone(), product);
        self.products = Some(products);

        Ok(self)
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
