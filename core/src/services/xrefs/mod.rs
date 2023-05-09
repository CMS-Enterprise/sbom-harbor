use crate::entities::xrefs::{Xref, Xrefs};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{MongoDocument, Service};
use std::collections::HashMap;

#[async_trait]
/// Trait that indicates a [Service] supports writing new Xrefs to existing documents that
/// implement the Xrefs trait.
pub trait XrefService<X>: Service<X>
where
    X: MongoDocument + Xrefs,
{
    /// Saves an Xref to each doc that matches the query filter.
    async fn save_xref(&self, filter: HashMap<&str, &str>, xref: &Xref) -> Result<(), Error> {
        let matches = self.query(filter).await?;

        for mut doc in matches {
            match doc.xrefs(xref) {
                true => {
                    self.update(&doc)
                        .await
                        .map_err(|e| Error::Entity(format!("save_xref::{}", e)))?;
                }
                false => {
                    continue;
                }
            }
        }

        Ok(())
    }
}
