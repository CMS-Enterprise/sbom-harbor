use crate::entities::xrefs::{Xref, XrefKind};
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// The Product entity represents a specific product and version combination of a software or
/// service a Vendor sells and is required to submit SBOMs for.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Product {
    /// The unique identifier for the Product.
    pub id: String,

    /// The name of the product.
    pub name: String,

    /// The version of the product.
    pub version: String,

    /// The unique identifier for the vendor of the [Product].
    pub vendor_id: String,
}

impl Product {
    /// Factory method to create new instance of type.
    pub fn new(name: String, version: String, vendor_id: String) -> Result<Product, Error> {
        if name.is_empty() {
            return Err(Error::Entity("name cannot be empty".to_string()));
        }

        if version.is_empty() {
            return Err(Error::Entity("version cannot be empty".to_string()));
        }

        if vendor_id.is_empty() {
            return Err(Error::Entity("vendor id cannot be empty".to_string()));
        }

        Ok(Product {
            id: "".to_string(),
            name,
            version,
            vendor_id,
        })
    }

    /// Returns an Xref to the [Product] instance.
    pub fn as_xref(&self, vendor_name: &str) -> Xref {
        Xref {
            kind: XrefKind::Product,
            map: HashMap::from([
                ("vendorId".to_string(), self.vendor_id.clone()),
                ("vendorName".to_string(), vendor_name.to_string()),
                ("productId".to_string(), self.id.clone()),
                ("productName".to_string(), self.name.clone()),
                ("productVersion".to_string(), self.version.clone()),
            ]),
        }
    }
}

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct ProductInsert {
    /// The name of the product.
    pub name: Option<String>,

    /// The version of the product.
    pub version: Option<String>,

    /// The unique identifier for the vendor of the [Product].
    pub vendor_id: Option<String>,
}

impl ProductInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<Product, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::Entity("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        let version = match &self.version {
            None => {
                return Err(Error::Entity("version required".to_string()));
            }
            Some(version) => version.clone(),
        };

        let vendor_id = match &self.vendor_id {
            None => {
                return Err(Error::Entity("vendor id required".to_string()));
            }
            Some(vendor_id) => vendor_id.clone(),
        };

        Product::new(name, version, vendor_id)
    }
}
