use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::entities::sboms::SbomProviderKind;
use crate::entities::xrefs::Xref;

/// An [Unsupported] is a item for which an SBOM cannot be generated using the current SBOM
/// provider. These items likely need to be assessed to see if an SBOM can be generated using an
/// alternate tool, or if they are simply invalid targets.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Unsupported {
    /// The unique identifier for the Unsupported.
    pub id: String,

    /// The unique identifier from the external system. Since a Purl cannot be determined, there
    /// needs to be a unique key present before insert for changeset tracking.
    pub external_id: String,

    /// The name of the item.
    pub name: String,

    /// The package manager of the unsupported package.
    pub package_manager: Option<String>,

    /// The provider that ran the sync that the [Unsupported] was identified in.
    pub provider: SbomProviderKind,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,
}
