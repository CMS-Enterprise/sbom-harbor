use serde::de::{Deserialize, Deserializer, IntoDeserializer};
use serde::ser::Serializer;
use serde::{ser, Serialize};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// An [Xref] is a dynamic way to track cross-references to internal and external systems.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Xref {
    /// Discriminator indicating which internal or external system being referenced.
    pub(crate) kind: XrefKind,
    /// The HashMap of key-value pairs that are used to cross-reference the entity in the other
    /// system.
    pub(crate) map: HashMap<String, String>,
}

impl PartialEq<Self> for Xref {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind {
            return false;
        }

        for (key, other_value) in other.map.iter() {
            match self.map.get(key) {
                None => {
                    return false;
                }
                Some(self_value) => {
                    if !self_value.eq(other_value) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

/// Discriminator indicating which entity type or external system the Xref references.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "XrefKind")]
pub enum XrefKind {
    /// [Xref] references a [Codebase] entity.
    Codebase,
    /// [Xref] references a [Product] entity.
    Product,
    /// [Xref] references values from an external system.
    External(String),
}

impl Display for XrefKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XrefKind::Codebase => write!(f, "codebase"),
            XrefKind::Product => write!(f, "product"),
            XrefKind::External(name) => write!(f, "external::{}", name.to_lowercase()),
        }
    }
}

impl ser::Serialize for XrefKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for XrefKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.contains("external::") {
            let name = s.replace("external::", "");
            Ok(XrefKind::External(name))
        } else {
            XrefKind::deserialize(s.into_deserializer())
        }
    }
}

/// Converts and Xref into a HashMap.
pub fn flatten(xref: &Xref) -> HashMap<String, String> {
    let mut results = HashMap::new();

    for x in xref.map.iter() {
        results.insert(format!("{}::{}", xref.kind, x.0), x.1.to_string());
    }

    results
}

/// Allows a consistent means of appending Xrefs to a type that has Xrefs.
pub trait Xrefs {
    /// Append an Xref. Returns false if Xref already existed or the operation was a noop.
    fn xrefs(&mut self, xref: &Xref) -> bool;
}

/// Macro to expand a struct so that it can be use to track cross-references.
#[macro_export]
macro_rules! xref {
    ($t:ty) => {
        impl Xrefs for $t {
            fn xrefs(&mut self, xref: &Xref) -> bool {
                if self.xrefs.iter().any(|x| x.eq(xref)) {
                    return false;
                } else {
                    self.xrefs.push(xref.clone());
                    return true;
                }
            }
        }
    };
}

pub use xref;
