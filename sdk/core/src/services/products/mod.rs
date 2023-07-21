use async_trait::async_trait;
use platform::persistence::mongodb::{Service, Store};
use platform::Error;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

use crate::entities::products::Product;
use crate::entities::sboms::{Sbom, SbomProviderKind};
use crate::services::sboms::SbomService;
use crate::services::vendors::VendorService;

/// Coordinates persistence logic for [Product] entities.
#[derive(Debug)]
pub struct ProductService {
    store: Arc<Store>,
    vendors: Option<Arc<VendorService>>,
    sboms: Option<Arc<SbomService>>,
}

impl ProductService {
    /// Factory method for new instance of type.
    pub fn new(
        store: Arc<Store>,
        vendors: Option<Arc<VendorService>>,
        sboms: Option<Arc<SbomService>>,
    ) -> ProductService {
        ProductService {
            store,
            vendors,
            sboms,
        }
    }

    /// Ingests an SBOM for the given product.
    pub async fn ingest(&self, id: &str, raw: &str) -> Result<Sbom, Error> {
        let sboms = match &self.sboms {
            None => {
                return Err(Error::Config("sbom service required".to_string()));
            }
            Some(s) => s.clone(),
        };

        let vendors = match &self.vendors {
            None => {
                return Err(Error::Config("vendor service required".to_string()));
            }
            Some(v) => v,
        };

        let product = match self.find(id).await {
            Ok(product) => match product {
                None => {
                    return Err(Error::Query("invalid product id".to_string()));
                }
                Some(p) => p,
            },
            Err(e) => {
                return Err(Error::Query(e.to_string()));
            }
        };

        let vendor = match vendors.find(product.vendor_id.as_str()).await {
            Ok(vendor) => match vendor {
                None => {
                    return Err(Error::Entity("invalid vendor id".to_string()));
                }
                Some(v) => v,
            },
            Err(e) => {
                return Err(Error::Query(e.to_string()));
            }
        };

        sboms
            .ingest(
                raw,
                None,
                SbomProviderKind::Vendor(vendor.name.clone()),
                product.as_xref(vendor.name.as_str()),
                None,
            )
            .await
            .map_err(|e| Error::Entity(e.to_string()))
    }
}

// TODO: Add transaction support to the Store/Service abstractions.
#[async_trait]
impl Service<Product> for ProductService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }

    /// Insert a document into a [Collection].
    #[instrument]
    async fn insert<'a>(&self, product: &mut Product) -> Result<(), Error> {
        if product.vendor_id.is_empty() {
            return Err(Error::Entity("vendor id cannot be empty".to_string()));
        }

        // Validate service configuration.
        let vendors = match &self.vendors {
            None => {
                return Err(Error::Config("vendor service required".to_string()));
            }
            Some(vendors) => vendors.clone(),
        };

        // Validate vendor exists.
        let mut vendor = match vendors.find(product.vendor_id.as_str()).await {
            Ok(vendor) => match vendor {
                None => {
                    return Err(Error::Entity("invalid vendor id".to_string()));
                }
                Some(vendor) => vendor,
            },
            Err(e) => {
                return Err(Error::Entity(e.to_string()));
            }
        };

        // WATCH: Name and version should be inherently unique, but it seems likely that there
        // would be
        // Validate product does not exist for vendor.
        match self
            .is_duplicate(HashMap::from([
                ("name", product.name.as_str()),
                ("version", product.version.as_str()),
            ]))
            .await
        {
            Ok(is_duplicate) => {
                if is_duplicate {
                    return Err(Error::Entity("product exists".to_string()));
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        match self.insert_inner(product).await {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Mongo(e.to_string()));
            }
        };

        // Add product reference to vendor.
        let vendor = match vendor.products(product.clone()) {
            Ok(vendor) => vendor,
            Err(mut e) => {
                // Rollback insert if vendor reference cannot be set.
                e = match self.delete(product.id.as_str()).await {
                    Ok(_) => e,
                    Err(inner) => {
                        Error::Entity(format!("rollback_failed::{}::{}", inner, e)).into()
                    }
                };
                return Err(Error::Entity(e.to_string()));
            }
        };

        vendors.update(vendor).await?;

        Ok(())
    }
}
