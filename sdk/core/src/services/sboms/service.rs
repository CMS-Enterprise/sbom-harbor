use std::collections::HashMap;
use std::sync::Arc;

use platform::mongodb::{Service, Store};

use crate::entities::sboms::Sbom;
use crate::entities::xrefs::Xref;
use crate::services::sboms::StorageProvider;
use crate::services::xrefs::XrefService;
use crate::Error;

/// Invoke [sbom-scorecard](https://github.com/eBay/sbom-scorecard) and return the results.
pub fn score(_path: &str) -> Result<String, Error> {
    Ok("not implemented".to_string())
}

/// Compare 2 SBOM scores.
pub fn compare(first_path: &str, second_path: &str) -> Result<String, Error> {
    let first_score = score(first_path)?;
    let second_score = score(second_path)?;

    let mut result = format!("----------------{} score-------------------", first_path);
    result.push_str(first_score.as_str());
    result.push_str(format!("----------------{} score-------------------", second_path).as_str());
    result.push_str(second_score.as_str());

    Ok(result)
}

// Implement Xref Service so that xrefs can be managed for Sboms.
impl XrefService<Sbom> for SbomService {}

/// Provides SBOM related capabilities.
#[derive(Debug)]
pub struct SbomService {
    store: Arc<Store>,
    storage: Box<dyn StorageProvider>,
}

impl Service<Sbom> for SbomService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl SbomService {
    /// Factory method for creating new instance of type.
    pub fn new(store: Arc<Store>, storage: Box<dyn StorageProvider>) -> Self {
        Self { store, storage }
    }

    /// Stores the SBOM to the configured persistence provider using the Purl as the unique
    /// identifier.
    pub async fn write_to_storage(
        &self,
        raw: Vec<u8>,
        sbom: &mut Sbom,
        xref: Option<Xref>,
    ) -> Result<(), Error> {
        // Persist to some sort of permanent storage.
        self.storage.write(raw, sbom, &xref).await?;

        Ok(())
    }

    /// Sets the forward only instance counter using the Purl as the unique identifier.
    pub async fn set_instance_by_purl(&self, sbom: &mut Sbom) -> Result<(), Error> {
        // TODO: As more enrichment sources are added this may need to constrain by Xref too.
        let existing = self.find_by_purl(&sbom.purl).await?;

        sbom.instance = match existing.iter().max_by_key(|s| s.instance) {
            None => 1,
            Some(most_recent) => most_recent.instance + 1,
        };

        Ok(())
    }

    /// Find an [Sbom] by its Package URL.
    pub async fn find_by_purl(&self, purl: &Option<String>) -> Result<Vec<Sbom>, Error> {
        match purl {
            None => Err(Error::Entity("sbom_purl_none".to_string())),
            Some(purl) => self
                .query(HashMap::from([("purl", purl.as_str())]))
                .await
                .map_err(|e| Error::Entity(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[async_std::test]
    async fn can_compare_sboms() -> Result<(), Error> {
        Ok(())
    }
}
