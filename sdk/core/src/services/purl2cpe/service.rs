use std::ops::Deref;
use std::path::Path;
use std::string::FromUtf8Error;
use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;
use urlencoding::decode;

use platform::git::Service as Git;
use platform::persistence::mongodb::service::Service as MongoService;
use platform::persistence::mongodb::store::Store;
use platform::Error as PlatformError;

use crate::config::github_pat;
use crate::entities::datasets::{Cpe, Purl2Cpes};
use crate::entities::packages::Package;

const PURL_2_CPE_URL: &str = "https://github.com/scanoss/purl2cpe.git";
const CLONE_PATH: &str = "/tmp/harbor/purl2cpe";

/// Service to build the data set for purl2cpe
#[derive(Debug)]
pub struct Purl2CpeService {
    pub(crate) store: Arc<Store>,
}

#[async_trait]
impl MongoService<Purl2Cpes> for Purl2CpeService {
    /// Getter for the store
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }

    /// Insert a document into a [Collection].
    async fn insert<'a>(&self, doc: &mut Purl2Cpes) -> Result<(), PlatformError> {
        if doc.id.is_empty() {
            return Err(PlatformError::Mongo(String::from(
                "==> purl2cpe::id(purl)::empty",
            )));
        }

        if doc.cpes.is_empty() {
            return Err(PlatformError::Mongo(String::from(
                "==> purl2cpe::cpes::empty",
            )));
        }

        self.store.insert(doc).await?;
        Ok(())
    }
}

fn find_cpe(version: String, cpes: Vec<String>) -> Option<String> {
    let mut final_cpe = None::<String>;
    let all_versions = String::from("*");
    let no_version = String::from("-");
    for cpe_str in cpes {
        let cpe = Cpe::new(cpe_str);

        // If this conditional evaluates to true, we have exactly
        // what we want and we can short circuit the loop
        if cpe.version.eq(&version) {
            final_cpe = Some(cpe.get_string());
            break;
        }

        // If the default is found, we load that up but keep looping
        if cpe.version.eq(&all_versions) || cpe.version.eq(&no_version) {
            final_cpe = Some(cpe.get_string());
        }
    }

    final_cpe
}

impl Purl2CpeService {
    /// Creates a new Purl2CpeService
    pub fn new(store: Arc<Store>) -> Self {
        Purl2CpeService { store }
    }

    /// Method to update a Package document in the store with a cpe
    pub async fn update_package_with_cpe(&self, id: String, cpe: String) -> Result<(), Error> {
        let mut package_doc = match self
            .store
            .find::<Package>(id.as_str())
            .await
            .map_err(Error::Mongo)?
        {
            None => {
                return Err(Error::Purl2Cpe(format!(
                    "Unable to find Package document in the data store with id: {}",
                    id
                )))
            }
            Some(package_doc) => package_doc,
        };

        package_doc.cpe = Some(cpe);
        self.store
            .update::<Package>(&package_doc)
            .await
            .map_err(Error::Mongo)?;

        Ok(())
    }

    /// Method to get a cpe for a purl
    pub async fn get_cpe(&self, full_purl: String) -> Result<Option<String>, Error> {
        fn error_result(msg: &str) -> Result<Option<String>, Error> {
            Err(Error::Purl2Cpe(String::from(msg)))
        }

        let at_sign = '@';

        if full_purl.is_empty() {
            return error_result("==> unable to process, purl is empty");
        }

        return if full_purl.contains(at_sign) {
            let full_purl: Vec<&str> = full_purl.split(at_sign).collect();
            let purl = full_purl.first();
            let version = String::from(*full_purl.get(1).unwrap());

            let purl_str = match purl {
                Some(purl_str) => {
                    println!(
                        "==> purl({}) contains version({})",
                        purl_str,
                        version.clone()
                    );
                    *purl_str
                }
                None => {
                    return error_result("==> unable to get cpe, purl is None while unwrapping")
                }
            };

            // If there are percent encoded chars in the purl, like
            // the at-sign for npm packages, we need to decode the string
            // so that it matches what is in the Purl2Cpe Collection
            let decoded_purl_str = decode(purl_str).map_err(Error::PercentEncoding)?;

            match self.store.find::<Purl2Cpes>(decoded_purl_str.deref()).await {
                // If the database is erroring out, then we can't go on.
                Err(err) => {
                    return error_result(
                        format!("==> unable to get cpe, database error: {}", err).as_str(),
                    )
                }

                // The Database gave us a response
                Ok(option) => match option {
                    // The response was empty
                    None => {
                        println!(
                            "==> unable to get cpe for purl({}), database response is empty",
                            purl_str
                        );

                        Ok(None)
                    }

                    // The response had CPEs!
                    Some(purl_2_cpes) => {
                        let cpes: Vec<String> = purl_2_cpes.cpes;

                        // If there is only one cpe, then return it.
                        if cpes.len() == 1 {
                            let cpe = cpes.get(0).unwrap().to_owned();
                            return Ok(Some(cpe));
                        }

                        Ok(find_cpe(version, cpes))
                    }
                },
            }
        } else {
            // If there are percent encoded chars in the purl, like
            // the at-sign for npm packages, we need to decode the string
            // so that it matches what is in the Purl2Cpe Collection
            let decoded_purl_str = decode(full_purl.as_str()).map_err(Error::PercentEncoding)?;

            match self.store.find::<Purl2Cpes>(decoded_purl_str.deref()).await {
                // If the database is erroring out, then we can't go on.
                Err(err) => {
                    return error_result(
                        format!("==> unable to get cpe, database error: {}", err).as_str(),
                    )
                }

                // The Database gave us a response
                Ok(option) => match option {
                    // The response was empty
                    None => {
                        println!(
                            "==> unable to get cpe for purl({}), database response is empty",
                            full_purl
                        );

                        Ok(None)
                    }

                    // the response had CPEs!
                    Some(purl_2_cpes) => {
                        let cpes: Vec<String> = purl_2_cpes.cpes;

                        // If there is only one cpe, then return it.
                        if cpes.len() == 1 {
                            let cpe = cpes.get(0).unwrap().to_owned();
                            return Ok(Some(cpe));
                        }

                        Ok(find_cpe(String::from("*"), cpes))
                    }
                },
            }
        };
    }

    /// Clones the purl2cpe repository at:
    ///     https://github.com/scanoss/purl2cpe
    pub fn clone_purl2cpe_repo(&self) -> Result<String, Error> {
        if Path::new(CLONE_PATH).is_dir() {
            self.remove_purl2cpe_clone()?;
        }

        let token =
            github_pat().map_err(|err| Error::Purl2Cpe(format!("Unable to get pat: {}", err)))?;
        Git::new(String::from(PURL_2_CPE_URL))
            .clone_repo(CLONE_PATH, Some(token))
            .map_err(Error::Git)?;

        Ok(String::from(CLONE_PATH))
    }

    /// Method to find yaml files
    pub fn find_purl_yaml_files(&self) -> Result<Vec<String>, Error> {
        let files = Git::new(String::from(PURL_2_CPE_URL))
            .find(String::from("purls.yml"), String::from(CLONE_PATH))
            .map_err(Error::Git)?;

        Ok(files)
    }

    /// Removes a cloned repository from the filesystem.
    pub fn remove_purl2cpe_clone(&self) -> Result<(), Error> {
        Git::remove_clone(CLONE_PATH).map_err(Error::Git)
    }
}

/// Represents all exposed Errors for this task.
#[derive(Error, Debug)]
pub enum Error {
    /// Error to handle percent encoding errors
    #[error(transparent)]
    PercentEncoding(#[from] FromUtf8Error),
    /// Error to map from platform Git Error
    #[error(transparent)]
    Git(#[from] platform::git::error::Error),
    /// Error to map from platform, specifically Mongo
    #[error(transparent)]
    Mongo(#[from] platform::Error),
    /// Error for Perl2Cpe general error
    #[error("purl2cpe error: {0}")]
    Purl2Cpe(String),
}

/*
 * Why are so many of these tests "debug manual only"?
 *
 * The reason is that each of them need specific data loaded into the
 * database and that should be done by having a set_up() function for
 * the test suite, but Rust does not support that.  Upon researching,
 * I found that a custom test framework feature has been developed that
 * would give us more control over the way the tests are executed and
 * allow us to solve the problem.  Here is a link from the unstable Rust
 * Book with the information:
 *
 * https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html#custom_test_frameworks
 *
 * Unfortunately, developing a framework for us is too far outside the
 * scope of the work I'm doing, so I'll just drop a:
 * TODO Add custom testing framework
 * Right here and we can talk about it later
 */
#[cfg(test)]
mod test {
    use std::sync::Arc;

    use platform::persistence::mongodb::Store;

    use crate::config::dev_context;
    use crate::entities::datasets::Purl2Cpes;
    use crate::services::purl2cpe::service::{find_cpe, Purl2CpeService};

    async fn add_test_data() {
        let store = get_store().await;
        store
            .drop_collection::<Purl2Cpes>()
            .await
            .expect("Unable to drop collection");

        let cpes = vec!["cpe:2.3:a:01org:tpm2.0-tools:1.1.0:*:*:*:*:*:*:*"]
            .into_iter()
            .map(String::from)
            .collect();
        let doc = Purl2Cpes::new(String::from("pkg:deb/debian/tpm2-tools"), cpes);
        store
            .insert::<Purl2Cpes>(&doc)
            .await
            .expect("Data insertion failure");

        let cpes = vec!["cpe:2.3:a:01org:tpm2.0-tools:1.1.0:*:*:*:*:*:*:*"]
            .into_iter()
            .map(String::from)
            .collect();
        let doc = Purl2Cpes::new(String::from("pkg:github/tpm2-software/tpm2-tools"), cpes);
        store
            .insert::<Purl2Cpes>(&doc)
            .await
            .expect("Data insertion failure");

        let cpes = vec![
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.2:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:*:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:-:*:*:*:*:wordpress:*:*",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let doc = Purl2Cpes::new(String::from("pkg:github/wpplugins/0mk-shortener"), cpes);
        store
            .insert::<Purl2Cpes>(&doc)
            .await
            .expect("Data insertion failure")
    }

    async fn get_store() -> Arc<Store> {
        let ctx = dev_context(None).unwrap();
        let store = Store::new(&ctx).await.unwrap();
        Arc::new(store)
    }

    async fn get_test_svc() -> Purl2CpeService {
        Purl2CpeService::new(get_store().await)
    }

    #[test]
    fn test_find_cpe_perfect_match_exists() {
        let version = String::from("0.9");

        let cpes = vec![
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.2:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:*:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.9:*:*:*:*:wordpress:*:*",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let found_cpe =
            String::from("cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.9:*:*:*:*:wordpress:*:*");
        assert_eq!(found_cpe, find_cpe(version, cpes).unwrap());
    }

    #[test]
    fn test_find_cpe_no_perfect_match_but_all_versions_exists() {
        let version = String::from("0.9");

        let cpes = vec![
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.2:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:*:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:1.2:*:*:*:*:wordpress:*:*",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let found_cpe =
            String::from("cpe:2.3:a:0mk_shortener_project:0mk_shortener:*:*:*:*:*:wordpress:*:*");
        assert_eq!(found_cpe, find_cpe(version, cpes).unwrap());
    }

    #[test]
    fn test_find_cpe_no_perfect_match_but_no_version_exists() {
        let version = String::from("0.9");

        let cpes = vec![
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.2:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:-:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:1.2:*:*:*:*:wordpress:*:*",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let found_cpe =
            String::from("cpe:2.3:a:0mk_shortener_project:0mk_shortener:-:*:*:*:*:wordpress:*:*");
        assert_eq!(found_cpe, find_cpe(version, cpes).unwrap());
    }

    #[test]
    fn test_find_cpe_no_perfect_match_or_special_versions_exists() {
        let version = String::from("0.9");

        let cpes = vec![
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.2:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:3.2:*:*:*:*:wordpress:*:*",
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:1.2:*:*:*:*:wordpress:*:*",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(None, find_cpe(version, cpes));
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_cpe_with_purl_version() {
        let test_svc = get_test_svc().await;
        let test_purl = String::from("pkg:deb/debian/tpm2-tools@1.1.0");
        let cpe: Option<String> = test_svc
            .get_cpe(test_purl)
            .await
            .expect("Should not be Error Result");
        assert_ne!(None, cpe);
        assert_eq!(
            "cpe:2.3:a:01org:tpm2.0-tools:1.1.0:*:*:*:*:*:*:*",
            cpe.unwrap()
        );
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_cpe_empty_purl() {
        add_test_data().await;
        let test_svc = get_test_svc().await;
        let test_purl = String::from("");
        let cpe = test_svc.get_cpe(test_purl).await;
        assert!(cpe.is_err());
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_cpe_with_purl_no_version_single_cpe_entry() {
        add_test_data().await;
        let test_svc = get_test_svc().await;
        let test_no_version_purl = String::from("pkg:github/tpm2-software/tpm2-tools");
        let cpe = test_svc
            .get_cpe(test_no_version_purl)
            .await
            .expect("Should not be Error Result");
        assert_ne!(None, cpe);
        assert_eq!(
            "cpe:2.3:a:01org:tpm2.0-tools:1.1.0:*:*:*:*:*:*:*",
            cpe.unwrap()
        );
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_cpe_with_purl_no_version_multiple_cpe_entry() {
        add_test_data().await;

        /*
         * Versions of CPE for "pkg:github/wpplugins/0mk-shortener":
         * 0. cpe:2.3:a:0mk_shortener_project:0mk_shortener:0.2:*:*:*:*:wordpress:*:*
         * 1. cpe:2.3:a:0mk_shortener_project:0mk_shortener:*:*:*:*:*:wordpress:*:*
         * 2. cpe:2.3:a:0mk_shortener_project:0mk_shortener:-:*:*:*:*:wordpress:*:*
         */

        let test_svc = get_test_svc().await;
        let test_no_version_purl = String::from("pkg:github/wpplugins/0mk-shortener");
        let cpe = test_svc
            .get_cpe(test_no_version_purl)
            .await
            .expect("Should not be Error Result");
        let tco = Some(String::from(
            "cpe:2.3:a:0mk_shortener_project:0mk_shortener:*:*:*:*:*:wordpress:*:*",
        ));
        assert_eq!(tco, cpe);
    }
}
