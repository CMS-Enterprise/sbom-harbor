use crate::entities::xrefs::{Xref, Xrefs};
use crate::Error;
use async_trait::async_trait;
use platform::mongodb::{Context, MongoDocument, Service};
use std::collections::HashMap;

#[async_trait]
pub trait XrefService<X>: Service<X>
where
    X: MongoDocument + Xrefs,
{
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
