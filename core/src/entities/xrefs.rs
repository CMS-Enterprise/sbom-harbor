use crate::Error;
use serde::de::{Deserialize, Deserializer, IntoDeserializer};
use serde::ser::Serializer;
use serde::{ser, Serialize};
use serde_derive::Deserialize;
use serde_json::ser::State;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// An [Xref] is a dynamic way to track cross-references to internal and external systems.
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

        return true;
    }
}

#[allow(missing_docs)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "XrefKind")]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum XrefKind {
    Codebase,
    Product,
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

pub fn flatten(xref: Xref) -> HashMap<String, String> {
    let mut results = HashMap::new();

    for x in xref.map.into_iter() {
        results.insert(format!("{}::{}", xref.kind, key), value);
    }

    results
}

pub trait Xrefs {
    fn xrefs(&mut self, xrefs: Option<Vec<Xref>>);
}

/// Macro to expand a struct so that it can be use to track cross-references.
#[macro_export]
macro_rules! xref {
    ($t:ty) => {
        impl Xrefs for $t {
            fn xrefs(&mut self, xrefs: &Option<Vec<Xref>>) {
                let xrefs = match xrefs {
                    None => {
                        return;
                    }
                    Some(xrefs) => xrefs,
                };

                let mut existing = match self.xrefs {
                    None => {
                        self.xrefs = xrefs.clone();
                        return;
                    }
                    Some(refs) => refs.clone(),
                };

                for xref in xrefs.into_iter() {
                    match existing.iter().any(|x| x.eq(xref)) {
                        true {},
                        false {
                            existing.append(xref);
                        }
                    }
                }
            }
        }
    };
}

pub use xref;
