use crate::common::Scenario;
use harbcore::config::dev_context;
use harbcore::entities::products::Product;
use harbcore::entities::vendors::Vendor;
use harbcore::services::packages::PackageService;
use harbcore::services::products::ProductService;
use harbcore::services::sboms::{FileSystemStorageProvider, SbomService};
use harbcore::services::vendors::VendorService;
use harbcore::testing::sbom_raw;
use harbcore::Error;
use platform::persistence::mongodb::{Service, Store};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

mod common;

#[async_std::test]
async fn can_ingest_sbom() -> Result<(), Error> {
    let raw = sbom_raw()?;
    let cx = dev_context(None)?;
    let store = Arc::new(Store::new(&cx).await?);
    let scenario = Scenario::new(Some(store.clone())).await?;

    let service = ProductService::new(
        store.clone(),
        Some(Arc::new(VendorService::new(store.clone()))),
        Some(Arc::new(SbomService::new(
            store.clone(),
            Some(Box::new(FileSystemStorageProvider::new(
                "/tmp/harbor/sboms".to_string(),
            ))),
            Some(PackageService::new(store.clone())),
        ))),
    );

    // Create a vendor.
    let mut vendor = Vendor::new("core-ingest-test".to_string())?;
    scenario.with_entity(&mut vendor)?;
    assert!(!vendor.id.is_empty());

    // Create a product
    let mut product = Product::new(
        "core-ingest-test".to_string(),
        "1.0.0".to_string(),
        vendor.id.clone(),
    )?;
    scenario.with_entity(&mut product)?;
    assert!(!product.id.is_empty());

    let sbom = service.ingest(product.id.as_str(), raw.as_str()).await?;

    assert!(!sbom.id.is_empty());

    scenario.cleanup(sbom).await?;
    scenario.cleanup(product).await?;
    scenario.cleanup(vendor).await?;

    Ok(())
}

#[async_std::test]
async fn can_validate_insert_product() -> Result<(), Error> {
    let store = Arc::new(Store::new(&dev_context(None)?).await?);
    let mut errors = vec![];

    match can_validate_ok(store.clone()).await {
        Ok(_) => {}
        Err(e) => errors.push(e.to_string()),
    }

    match can_validate_product_empty_vendor_id(store.clone()).await {
        Ok(_) => {}
        Err(e) => errors.push(e.to_string()),
    }

    match can_validate_product_no_vendor(store.clone()).await {
        Ok(_) => {}
        Err(e) => errors.push(e.to_string()),
    }

    match can_validate_product_db_duplicate(store.clone()).await {
        Ok(_) => {}
        Err(e) => errors.push(e.to_string()),
    }

    match can_validate_vendor_products_version_duplicate(store.clone()).await {
        Ok(_) => {}
        Err(e) => errors.push(e.to_string()),
    }

    assert!(errors.is_empty(), "{}", errors.join("\n").to_string());

    Ok(())
}

fn product_service(store: Arc<Store>) -> ProductService {
    ProductService::new(
        store.clone(),
        Some(Arc::new(VendorService::new(store.clone()))),
        Some(Arc::new(SbomService::new(
            store.clone(),
            Some(Box::new(FileSystemStorageProvider::new(
                "/tmp/harbor/sboms".to_string(),
            ))),
            Some(PackageService::new(store.clone())),
        ))),
    )
}

async fn can_validate_ok(store: Arc<Store>) -> Result<(), Error> {
    let scenario = Scenario::new(Some(store.clone())).await?;
    let service = product_service(store.clone());

    // Create a vendor.
    let mut vendor = Vendor::new("can_validate_ok".to_string())?;
    scenario.with_entity(&mut vendor)?;
    if vendor.id.is_empty() {
        return Err(Error::Entity("vendor id empty".to_string()));
    }

    // Create a product
    let mut product = Product::new(
        "can_validate_ok".to_string(),
        "1.0.0".to_string(),
        vendor.id.clone(),
    )?;

    let result = match service.insert(&mut product).await {
        Ok(_) => match product.id.is_empty() {
            true => Err(Error::Entity("product id empty".to_string())),
            false => Ok(()),
        },
        Err(e) => Err(Error::Entity(e.to_string())),
    };

    scenario.cleanup(product).await?;
    scenario.cleanup(vendor).await?;

    result
}

async fn can_validate_product_empty_vendor_id(store: Arc<Store>) -> Result<(), Error> {
    let expected = "vendor id cannot be empty";
    let scenario = Scenario::new(Some(store.clone())).await?;

    let mut product = Product {
        id: "".to_string(),
        name: "can_validate_insert_product_empty_vendor_id".to_string(),
        version: "0.0.0".to_string(),
        vendor_id: "".to_string(),
    };

    let service = product_service(store.clone());

    match service.insert(&mut product).await {
        Ok(_) => {
            let err_msg = format!("validation failure: expected {}", expected);
            return Err(Error::Entity(err_msg));
        }
        Err(e) => {
            let err_msg = e.to_string();
            if !err_msg.contains(expected) {
                let err_msg = format!(
                    "validation failure: \n\texpected: {}\n\tgot: {}\n",
                    expected, err_msg
                );
                return Err(Error::Entity(err_msg));
            }
        }
    }

    scenario
        .clean_by_query::<Product>(HashMap::from([("name", product.name.as_str())]))
        .await?;

    Ok(())
}

async fn can_validate_product_no_vendor(store: Arc<Store>) -> Result<(), Error> {
    let expected = "invalid vendor id";
    let scenario = Scenario::new(Some(store.clone())).await?;

    let mut product = Product {
        id: "".to_string(),
        name: "can_validate_insert_product_no_vendor".to_string(),
        version: "0.0.0".to_string(),
        vendor_id: "can_validate_insert_product_no_vendor".to_string(),
    };

    let service = product_service(store.clone());

    match service.insert(&mut product).await {
        Ok(_) => {
            let err_msg = format!("validation failure: expected {}", expected);
            return Err(Error::Entity(err_msg));
        }
        Err(e) => {
            let err_msg = e.to_string();
            if !err_msg.contains(expected) {
                let err_msg = format!(
                    "validation failure: \n\texpected: {}\n\tgot: {}\n",
                    expected, err_msg
                );
                return Err(Error::Entity(err_msg));
            }
        }
    }

    scenario
        .clean_by_query::<Product>(HashMap::from([("name", product.name.as_str())]))
        .await?;

    Ok(())
}

async fn can_validate_product_db_duplicate(store: Arc<Store>) -> Result<(), Error> {
    let expected = "product exists";
    let scenario = Scenario::new(Some(store.clone())).await?;

    let mut vendor = Vendor::new("can_validate_insert_product_db_duplicate".to_string())?;
    let vendor = scenario.with_entity(&mut vendor)?;

    let mut product = Product {
        id: "".to_string(),
        name: "can_validate_insert_product_db_duplicate".to_string(),
        version: "0.0.0".to_string(),
        vendor_id: vendor.id.clone(),
    };

    // get a dupe prior to the id getting set on save.
    let mut insert_dupe = product.clone();
    // now insert existing.
    scenario.with_entity(&mut product)?;

    // Try to insert
    let service = product_service(store.clone());
    let result = match service.insert(&mut insert_dupe).await {
        Ok(_) => {
            let err_msg = format!("validation failure: expected {}", expected);
            Err(Error::Entity(err_msg))
        }
        Err(e) => {
            let err_msg = e.to_string();
            if !err_msg.contains(expected) {
                let err_msg = format!(
                    "validation failure: \n\texpected: {}\n\tgot: {}\n",
                    expected, err_msg
                );
                return Err(Error::Entity(err_msg));
            }
            Ok(())
        }
    };

    scenario
        .clean_by_query::<Product>(HashMap::from([("name", product.name.as_str())]))
        .await?;
    scenario
        .clean_by_query::<Vendor>(HashMap::from([("name", vendor.name.as_str())]))
        .await?;

    result
}

async fn can_validate_vendor_products_version_duplicate(store: Arc<Store>) -> Result<(), Error> {
    let expected = "product exists for version";
    let scenario = Scenario::new(Some(store.clone())).await?;

    // Create vendor.
    let mut vendor = Vendor::new("can_validate_vendor_products_version_duplicate".to_string())?;
    vendor = scenario.with_entity(&mut vendor)?;

    // Create product for vendor.
    let mut product = Product {
        id: Uuid::new_v4().to_string(),
        name: "can_validate_vendor_products_version_duplicate".to_string(),
        version: "0.0.0".to_string(),
        vendor_id: vendor.id.clone(),
    };
    // Needs an id so we'll save it to the db.
    scenario.with_entity(&mut product)?;

    // Add reference to vendor.
    vendor.products(product.clone())?;
    // update the vendor with existing product entry that has same name and version.
    scenario.update(&vendor)?;

    // get a dupe.
    let mut insert_dupe = product.clone();
    // blank out id.
    insert_dupe.id = "".to_string();
    scenario.with_entity(&mut insert_dupe)?;

    let result = match vendor.products(insert_dupe) {
        Ok(_) => {
            let err_msg = format!("validation failure: expected {}", expected);
            Err(Error::Entity(err_msg))
        }
        Err(e) => {
            let err_msg = e.to_string();
            if !err_msg.contains(expected) {
                let err_msg = format!(
                    "validation failure: \n\texpected: {}\n\tgot: {}\n",
                    expected, err_msg
                );
                return Err(Error::Entity(err_msg));
            }
            Ok(())
        }
    };

    scenario
        .clean_by_query::<Product>(HashMap::from([("name", product.name.as_str())]))
        .await?;
    scenario
        .clean_by_query::<Vendor>(HashMap::from([("name", vendor.name.as_str())]))
        .await?;

    result
}
