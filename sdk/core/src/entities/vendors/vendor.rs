use super::product::Product;
use serde::{Deserialize, Serialize};

///  A Vendor is an entity that sells software or software services. Vendors are required to
/// supply SBOMs for the products they sell.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vendor {
    /// The unique identifier for the Vendor.
    pub id: String,
    /// The name of the Vendor.
    pub name: String,
    /// Products that are owned by the Vendor.
    #[serde(default = "Vec::new")]
    pub products: Vec<Product>,
}

impl Vendor {
    /// Factory method to create new instance of type.
    pub fn new(name: String) -> Self {
        Self {
            id: "".to_string(),
            name,
            products: Default::default(),
        }
    }

    /// Add a product to the products Vector.
    pub fn products(&mut self, product: Product) -> &Self {
        self.products.push(product);
        self
    }

    /// Determines if the specified product id is owned by an instance of a Vendor.
    pub fn owns_product(&self, product_id: String) -> bool {
        self.products.iter().any(|p| p.id == product_id)
    }
}
