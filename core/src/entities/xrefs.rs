use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// An [Xref] is a dynamic way to track cross-references to internal and external systems.
/// Fundamentally, it is just a [HashMap] of one or more id names to id value. By aliasing the
/// type we are able to more clearly communicate the domain and add extension convenience functions.
pub type Xref = HashMap<String, String>;

#[allow(missing_docs)]
#[serde(rename_all = "camelCase")]
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

pub fn flatten(xrefs: HashMap<XrefKind, Xref>) -> HashMap<String, String> {
    let mut results = HashMap::new();

    for (kind, xref) in xrefs.into_iter() {
        for (key, value) in xref.into_iter() {
            results.insert(format!("{}::{}", kind, key), value);
        }
    }

    results
}

pub trait Xrefs {
    fn inner(&self) -> &Option<HashMap<XrefKind, Xref>>;
    fn xrefs(&mut self, xrefs: Option<HashMap<XrefKind, Xref>>);
}

/// Macro to expand a struct so that it can be use to track cross-references.
#[macro_export]
macro_rules! xref {
    ($t:ty) => {
        impl Xrefs for $t {
            fn inner(&self) -> &Option<HashMap<XrefKind, Xref>> {
                &self.xrefs
            }

            fn xrefs(&mut self, xrefs: Option<HashMap<XrefKind, Xref>>) {
                let xrefs = match xrefs {
                    None => {
                        return;
                    }
                    Some(xrefs) => xrefs,
                };

                let mut inner = match self.inner().clone() {
                    None => {
                        self.xrefs = Some(xrefs);
                        return;
                    }
                    Some(refs) => refs,
                };

                for (kind, xref) in xrefs.into_iter() {
                    match inner.get_mut(&kind) {
                        None => {
                            inner.insert(kind, xref);
                        }
                        Some(existing) => {
                            existing.extend(xref.into_iter().map(|(k, v)| (k.clone(), v.clone())))
                        }
                    }
                }
            }
        }
    };
}

pub use xref;
