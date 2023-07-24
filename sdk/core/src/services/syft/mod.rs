use crate::entities::cyclonedx::Bom;

use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Errors specific to Syft operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Raised when there is an error while attempting to syft a Repository
    #[error("syft error: {0}")]
    SyftError(String),
}

const CYCLONEDX_JSON_FORMAT: &str = "cyclonedx-json";
lazy_static! {
    /// Mop of Syft catalogers to package manager purl prefixes based on Syft
    /// [supported ecosystems](https://github.com/anchore/syft/blob/376c42893b38a68e9703470d9e625bf98612a1d4/README.md?plain=1#L488).
    static ref CATALOGERS: HashMap<&'static str, &'static str> = {
            HashMap::from([
            ("alpmdb-cataloger", "alpm"),
            ("apkdb-cataloger", "apk"),
            ("binary-cataloger", "binary"),
            ("cargo-auditable-binary-cataloger", "cargo"),
            ("cocoapods-cataloger", "cocoapods"),
            ("conan-cataloger", "conan"),
            ("dartlang-lock-cataloger", "pub"),
            ("dotnet-deps-cataloger", "nuget"),
            ("dpkgdb-cataloger", "deb"),
            ("elixir-mix-lock-cataloger", "hex"),
            ("erlang-rebar-lock-cataloger", "hex"),
            ("go-mod-file-cataloger", "golang"),
            ("go-module-binary-cataloger", "golang"),
            // ("graalvm-native-image-cataloger", ""),
            ("haskell-cataloger", "hackage"),
            ("java-cataloger", "maven"),
            ("java-gradle-lockfile-cataloger", "maven"),
            ("java-pom-cataloger", "maven"),
            ("javascript-lock-cataloger", "npm"),
            ("javascript-package-cataloger", "npm"),
            ("linux-kernel-cataloger", "generic"),
            ("nix-store-cataloger", "nix"),
            ("php-composer-installed-cataloger", "composer"),
            ("php-composer-lock-cataloger", "composer"),
            ("portage-cataloger", "ebuild"),
            ("python-index-cataloger", "pypi"),
            ("python-package-cataloger", "pypi"),
            ("rpm-db-cataloger", "rpm"),
            ("rpm-file-cataloger", "rpm"),
            ("ruby-gemfile-cataloger", "gem"),
            ("ruby-gemspec-cataloger", "gem"),
            ("rust-cargo-lock-cataloger", "cargo"),
            // ("sbom-cataloger", ""),
        ])
    };
}

// TODO This method is still stopgap because it is using the actual cataloger as the type rather than
//  the correct type that the cataloger *maps* to.
/*
   =============================================================
   ==> scheme:type/namespace/name@version?qualifiers#subpath <==
   =============================================================

   The definition for each components is:

   => scheme:     this is the URL scheme with the constant value of "pkg". One of the primary
                  reason for this single scheme is to facilitate the future official
                  registration of the "pkg" scheme for package URLs. Required.

   => type:       the package "type" or package "protocol" such as maven, npm, nuget, gem,
                  pypi, etc. Required.

   => namespace:  some name prefix such as a Maven groupid, a Docker image owner, a GitHub user or
                  organization. Optional and type-specific.

   => name:       the name of the package. Required.

   => version:    the version of the package. Optional.

   => qualifiers: extra qualifying data for a package such as an OS, architecture, a distro, etc.
                  Optional and type-specific.

   => subpath:    extra subpath within a package, relative to the package root. Optional.
*/
fn generate_stopgap_purl(
    full_name: String,
    commit_hash: String,
    cataloger: Option<String>,
    sub_path: Option<String>,
) -> String {
    let cataloger = match cataloger {
        Some(cataloger) => cataloger,
        None => String::from("no-cataloger"),
    };

    let purl = format!("pkg:{}:{}@{}", cataloger, full_name, commit_hash);

    match sub_path {
        None => purl,
        Some(value) => {
            if value.is_empty() || value == "/" {
                purl
            } else {
                let mut owned_purl = purl;
                owned_purl.push_str(format!("#{}", value).as_str());
                owned_purl
            }
        }
    }
}

/// Syft Service
pub(crate) struct Service {
    source_path: String,
}

impl Service {
    fn run_syft(&self) -> Result<Output, Error> {
        match Command::new("syft")
            .arg("--output")
            .arg(CYCLONEDX_JSON_FORMAT)
            .arg(self.source_path.as_str())
            .output()
        {
            Ok(output) => Ok(output),
            Err(err) => Err(Error::SyftError(format!("Error executing Syft: {}", err))),
        }
    }

    fn run_syft_with_cataloger(&self, cataloger: String) -> Result<Output, Error> {
        match Command::new("syft")
            .arg("--output")
            .arg(CYCLONEDX_JSON_FORMAT)
            .arg("--catalogers")
            .arg(cataloger)
            .arg(self.source_path.as_str())
            .output()
        {
            Ok(output) => Ok(output),
            Err(err) => Err(Error::SyftError(format!("Error executing Syft: {}", err))),
        }
    }

    /// Conventional Constructor
    pub fn new(source_path: String) -> Self {
        Service { source_path }
    }

    /// Invokes the syft CLI against the cloned repository to generate an SBOM.
    pub(crate) fn execute(
        &self,
        full_name: String,
        commit_hash: String,
        cataloger: Option<String>,
        sub_path: Option<String>,
    ) -> Result<String, Error> {
        let mut orig_dir = None::<PathBuf>;

        if let Some(path) = sub_path.clone() {
            let absolute_path = format!("{}/{}", self.source_path, path.as_str());
            let file_path = Path::new(&absolute_path);

            orig_dir = match env::current_dir() {
                Ok(path_buf) => Some(path_buf),
                Err(err) => {
                    return Err(Error::SyftError(format!(
                        "Unable to get current directory: {}",
                        err
                    )))
                }
            };

            println!("==> about to run Syft in: {:#?}", file_path);
            env::set_current_dir(file_path).map_err(|err| {
                Error::SyftError(format!(
                    "Unable to change directories to build target location: {}",
                    err
                ))
            })?
        }

        let output = match cataloger.clone() {
            Some(cataloger) => self.run_syft_with_cataloger(cataloger),
            None => self.run_syft(),
        }?;

        // Handle error generated by syft.
        if !&output.status.success() {
            return match String::from_utf8(output.stderr) {
                Ok(stderr) => Err(Error::SyftError(format!(
                    "Error executing Syft: {}",
                    stderr
                ))),
                Err(err) => Err(Error::SyftError(format!(
                    "Error executing Syft and reading stderr: {}",
                    err
                ))),
            };
        }

        if output.stdout.is_empty() {
            return Err(Error::SyftError("syft generated empty SBOM".to_string()));
        };

        let output: String = String::from_utf8_lossy(&output.stdout).to_string();
        let mut json: Value =
            serde_json::from_str(&output).map_err(|e| Error::SyftError(e.to_string()))?;

        // Access the nested "component" object
        if let Some(metadata) = json.get_mut("metadata").and_then(Value::as_object_mut) {
            if let Some(component) = metadata.get_mut("component").and_then(Value::as_object_mut) {
                component.insert(
                    String::from("purl"),
                    json!(generate_stopgap_purl(
                        full_name,
                        commit_hash,
                        cataloger,
                        sub_path
                    )),
                );
            }
        }

        if let Some(path) = orig_dir {
            println!(
                "==> about to change directories back to: {:#?}\n",
                path.to_str().unwrap()
            );
            env::set_current_dir(path).map_err(|err| {
                Error::SyftError(format!(
                    "Unable to change directories back to original location: {}",
                    err
                ))
            })?
        }

        Ok(serde_json::to_string(&json).unwrap())
    }
}

/// Best-effort algorithm to determine the package manager for an Sbom generated by Syft.
pub fn try_extract_package_manager(bom: &Bom) -> String {
    // define a default so that we always succeed, but can easily identify records where we were
    // unable to resolve a best guess.
    let pkg_harbor = "pkg:harbor".to_string();

    // Deref the components so that we can inspect their catalogers.
    let components = match &bom.components {
        None => {
            return pkg_harbor;
        }
        Some(c) => {
            if c.is_empty() {
                return pkg_harbor;
            }
            c
        }
    };

    // Keep a count of each found cataloger.
    let mut catalogers: HashMap<&str, u16> = HashMap::new();

    // Inspect each component's properties.
    for component in components {
        // Move to next component if no properties.
        let properties = match &component.properties {
            None => continue,
            Some(p) => {
                if p.is_empty() {
                    continue;
                }
                p
            }
        };

        // Look for the cataloger property.
        for property in properties {
            // Skip to next property if not the cataloger property.
            let cataloger = match &property.name {
                None => continue,
                Some(n) => {
                    if n != "syft:package:foundBy" {
                        continue;
                    }
                    match &property.value {
                        None => "",
                        Some(cataloger) => cataloger.as_str(),
                    }
                }
            };

            // If we can resolve the cataloger, increment its count.
            if CATALOGERS.contains_key(cataloger) {
                match catalogers.contains_key(cataloger) {
                    true => {
                        *catalogers.get_mut(cataloger).unwrap() += 1;
                    }
                    false => {
                        catalogers.insert(cataloger, 1);
                    }
                }
            }

            // Stop iterating properties and move on to next component.
            break;
        }
    }

    // Find the cataloger with the highest count.
    let max_cataloger = catalogers
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| *k);

    let key = match max_cataloger {
        Some(key) => key,
        None => {
            return pkg_harbor;
        }
    };

    // Return the purl package manager abbreviation for the cataloger.
    let abbr = match CATALOGERS.get(key) {
        None => {
            return pkg_harbor;
        }
        Some(a) => a,
    };

    format!("pkg:{}", abbr)
}

/// Generates an SBOM by shelling out to the Syft CLI.
pub fn execute(source_path: &str, name: &str, version: &str) -> Result<String, Error> {
    let mut cmd = Command::new("syft");
    let cmd = cmd
        .arg("--output")
        .arg("cyclonedx-json")
        .arg(source_path)
        .arg("--source-name")
        .arg(name)
        .arg("--source-version")
        .arg(version);

    platform::process::execute(cmd, "syft").map_err(|e| Error::Sbom(format!("syft_error::{}", e)))
}

/// Module for testing
#[cfg(test)]
mod test {
    use crate::entities::cyclonedx::Bom;
    use crate::entities::sboms::{Author, CdxFormat, Sbom};
    use crate::entities::xrefs::{Xref, XrefKind};
    use crate::services::syft::{generate_stopgap_purl, Error, Service as Syft};
    use crate::testing::sbom_raw;
    use platform::filesystem::get_tmp_location;
    use platform::git::Service as Git;
    use std::collections::HashMap;

    #[test]
    fn test_generate_stopgap_purl_with_sub_path() -> Result<(), Error> {
        let cataloger = String::from("harbor");
        let full_name = String::from("My/test");
        let commit_hash = String::from("abc123");
        let sub_path = String::from("/path/to/build/target");

        let test_purl_with_sub_path = format!(
            "pkg:{}:{}@{}#{}",
            cataloger, full_name, commit_hash, sub_path
        );

        let purl_with_sub_path =
            generate_stopgap_purl(full_name, commit_hash, Some(cataloger), Some(sub_path));

        assert_eq!(purl_with_sub_path, test_purl_with_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_stopgap_purl_no_sub_path() -> Result<(), Error> {
        let cataloger = String::from("harbor");
        let full_name = String::from("My/test");
        let commit_hash = String::from("abc123");

        let test_purl_no_sub_path = format!("pkg:harbor:{}@{}", full_name, commit_hash);
        let purl_no_sub_path = generate_stopgap_purl(
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger),
            None,
        );

        assert_eq!(purl_no_sub_path, test_purl_no_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_stopgap_purl_root_sub_path() -> Result<(), Error> {
        let cataloger = String::from("harbor");
        let full_name = String::from("My/test");
        let commit_hash = String::from("abc123");
        let sub_path = String::from("/");

        let test_purl_root_sub_path = format!("pkg:harbor:{}@{}", full_name, commit_hash);
        let purl_root_sub_path = generate_stopgap_purl(
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger),
            Some(sub_path),
        );

        assert_eq!(purl_root_sub_path, test_purl_root_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_stopgap_purl_root_empty_path() -> Result<(), Error> {
        let full_name = String::from("My/test");
        let commit_hash = String::from("abc123");
        let cataloger = String::from("harbor");
        let sub_path = String::from("");

        let test_purl_empty_sub_path = format!("pkg:harbor:{}@{}", full_name, commit_hash);
        let purl_empty_sub_path =
            generate_stopgap_purl(full_name, commit_hash, Some(cataloger), Some(sub_path));

        assert_eq!(purl_empty_sub_path, test_purl_empty_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_single() -> Result<(), Error> {
        let repo_url = "https://github.com/harbor-test-org/java-repo.git".to_string();
        let git = Git::new(repo_url);

        let repo_loc = get_tmp_location();
        let full_name = "Test/Repo";
        let cataloger = Some(String::from("harbor"));
        let commit_hash = "abc123";

        let purl_under_test = generate_stopgap_purl(
            full_name.to_string(),
            commit_hash.to_string(),
            cataloger.clone(),
            None,
        );

        git.clone_repo(repo_loc.as_str())
            .map_err(|err| Error::SyftError(format!("{}", err)))?;

        let syft = Syft::new(repo_loc.clone());

        let syft_result = syft.execute(
            full_name.to_string(),
            commit_hash.to_string(),
            cataloger,
            None,
        );

        Git::remove_clone(repo_loc.as_str()).map_err(|err| Error::SyftError(format!("{}", err)))?;

        let sbom = Bom::parse(syft_result.unwrap().as_str(), CdxFormat::Json)
            .map_err(|err| Error::SyftError(format!("{}", err)))?;

        let purl = sbom.metadata.unwrap().component.unwrap().purl.unwrap();

        assert_eq!(purl, purl_under_test);

        Ok(())
    }

    #[test]
    fn test_generate_multi() -> Result<(), Error> {
        let repo_url = "https://github.com/harbor-test-org/java-multi-module.git".to_string();
        let git = Git::new(repo_url);

        let repo_loc = get_tmp_location();
        let full_name = String::from("Test/Repo");
        let cataloger = Some(String::from("java-pom"));
        let commit_hash = String::from("abc123");

        git.clone_repo(repo_loc.as_str())
            .map_err(|err| Error::SyftError(format!("{}", err)))?;

        let poms: Vec<String> = git
            .find("pom.xml".to_string(), repo_loc.clone())
            .map_err(|err| Error::SyftError(format!("{}", err)))?;

        let syft = Syft::new(repo_loc);

        for pom_path in poms.iter() {
            let trimmed_pom_path = pom_path.replace("/pom.xml", "");

            match syft.execute(
                full_name.clone(),
                commit_hash.clone(),
                cataloger.clone(),
                Some(trimmed_pom_path.clone()),
            ) {
                Ok(sbom) => {
                    let purl_under_test = generate_stopgap_purl(
                        full_name.clone(),
                        commit_hash.clone(),
                        cataloger.clone(),
                        Some(trimmed_pom_path.to_string()),
                    );

                    let sbom = Bom::parse(sbom.as_str(), CdxFormat::Json)
                        .map_err(|err| Error::SyftError(format!("{}", err)))?;

                    let purl = sbom.metadata.unwrap().component.unwrap().purl.unwrap();

                    assert_eq!(purl, purl_under_test);
                }
                Err(err) => return Err(Error::SyftError(format!("{}", err))),
            }
        }

        Ok(())
    }

    #[test]
    fn can_resolve_package_purl() -> Result<(), Error> {
        let raw = sbom_raw()?;

        let sbom = Sbom::from_raw_cdx(
            raw.as_str(),
            CdxFormat::Json,
            Author::Vendor("can_resolve_package_manager".to_string()),
            &None,
            Xref {
                kind: XrefKind::Product,
                map: HashMap::default(),
            },
            None,
        )?;

        match sbom.purl {
            None => {
                return Err(Error::Entity("could not resolve sbom purl".to_string()));
            }
            Some(purl) => {
                assert!(purl.starts_with("pkg:cargo"));
            }
        }

        Ok(())
    }
}
