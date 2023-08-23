use harbcore::config::dev_context;
use harbcore::testing::sbom_fixture_path;
use harbcore::Error;
use platform::persistence::mongodb::{MongoDocument, Store};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime;
use uuid::Uuid;

/// Scenario builder used to setup and teardown test scenarios.
pub struct Scenario {
    pub(crate) store: Arc<Store>,
}

impl Scenario {
    /// Factory method to create new instance of type.
    pub async fn new(store: Option<Arc<Store>>) -> Result<Scenario, Error> {
        let store = match store {
            None => {
                let cx = dev_context(None)?;
                Arc::new(Store::new(&cx).await?)
            }
            Some(s) => s,
        };

        Ok(Self { store })
    }

    /// Store an arbitrary entity to the datastore.
    pub fn with_entity<E>(&self, entity: &mut E) -> Result<E, Error>
    where
        E: MongoDocument,
    {
        let rt = runtime::Runtime::new().map_err(|e| Error::Runtime(e.to_string()))?;
        entity.set_id(Uuid::new_v4().to_string());

        rt.block_on(async {
            match self
                .store
                .insert::<E>(entity)
                .await
                .map_err(|e| Error::Entity(e.to_string()))
            {
                Ok(_) => Ok(entity.clone()),
                Err(e) => Err(Error::Entity(e.to_string())),
            }
        })
    }

    /// Update an arbitrary entity in the datastore.
    pub fn update<E>(&self, entity: &E) -> Result<(), Error>
    where
        E: MongoDocument,
    {
        let rt = runtime::Runtime::new().map_err(|e| Error::Runtime(e.to_string()))?;

        rt.block_on(async {
            match self
                .store
                .update::<E>(entity)
                .await
                .map_err(|e| Error::Entity(e.to_string()))
            {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::Entity(e.to_string())),
            }
        })
    }

    /// Remove an arbitrarily added entity from the datastore.
    pub async fn cleanup<E>(&self, entity: E) -> Result<(), Error>
    where
        E: MongoDocument,
    {
        let id = entity.id();
        self.store
            .delete::<E>(id.as_str())
            .await
            .map_err(|e| Error::Entity(e.to_string()))
    }

    /// Remove any entities matching the query.
    pub async fn clean_by_query<E>(&self, map: HashMap<&str, &str>) -> Result<(), Error>
    where
        E: MongoDocument,
    {
        let existing = self.store.query::<E>(map).await?;

        for entity in existing.iter() {
            let id = entity.id();
            self.store
                .delete::<E>(id.as_str())
                .await
                .map_err(|e| Error::Entity(e.to_string()))?;
        }

        Ok(())
    }
}

/// Load the default SBOM test fixture to a string for processing in test cases.
#[allow(dead_code)]
pub fn raw_sbom() -> Result<String, Error> {
    std::fs::read_to_string(sbom_fixture_path()?).map_err(|e| Error::Entity(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use harbcore::entities::vendors::Vendor;

    #[async_std::test]
    async fn can_setup_and_teardown() -> Result<(), Error> {
        let scenario = Scenario::new(None).await?;
        let vendor =
            scenario.with_entity(&mut Vendor::new("can_setup_and_teardown".to_string())?)?;

        assert!(!vendor.id.is_empty());

        let vendor_id = vendor.id.clone();

        scenario.cleanup(vendor).await?;

        match scenario.store.find::<Vendor>(vendor_id.as_str()).await {
            Ok(vendor) => match vendor {
                None => {}
                Some(_) => {
                    return Err(Error::Entity("failed to cleanup vendor".to_string()));
                }
            },
            Err(e) => {
                return Err(Error::Entity(e.to_string()));
            }
        }

        Ok(())
    }
}
