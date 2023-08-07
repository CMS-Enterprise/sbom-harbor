use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::string::FromUtf8Error;
use std::{env, io};

use thiserror::Error;

use crate::entities::cyclonedx::component::ComponentType;
use crate::entities::cyclonedx::{Bom, Component, Metadata};
use crate::entities::sboms::CdxFormat;

use crate::Error as CoreError;

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
fn generate_purl(
    full_name: String,
    version: String,
    cataloger: Option<String>,
    sub_path: Option<String>,
) -> String {
    let cataloger = match cataloger {
        Some(cataloger) => cataloger,
        None => String::from("no-cataloger"),
    };

    let purl_type_opt = CATALOGERS.get(cataloger.as_str());
    let purl_type = purl_type_opt.unwrap_or(&"no-type");
    let purl = format!("pkg:{}/{}@{}", purl_type, full_name, version);

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

fn ensure_purl_in_metadata(
    sbom: Bom,
    full_name: String,
    version: String,
    cataloger: Option<String>,
    sub_path: Option<String>,
) -> Metadata {
    match sbom.metadata {
        None => create_metadata(full_name, version, cataloger, sub_path),
        Some(metadata) => {
            let mut unboxed_metadata = *metadata;

            let component: Component = match unboxed_metadata.clone().component {
                None => create_component(full_name, version, cataloger, sub_path),
                Some(component) => {
                    let mut unboxed_component = *component;

                    if unboxed_component.purl.is_none() {
                        let purl = generate_purl(full_name, version, cataloger, sub_path);
                        unboxed_component.purl = Some(purl);
                    }

                    unboxed_component
                }
            };

            unboxed_metadata.component = Some(Box::new(component));

            unboxed_metadata
        }
    }
}

fn create_metadata(
    full_name: String,
    commit_hash: String,
    cataloger: Option<String>,
    sub_path: Option<String>,
) -> Metadata {
    let component: Component = create_component(full_name, commit_hash, cataloger, sub_path);

    let mut metadata: Metadata = Metadata::new();
    metadata.component = Some(Box::new(component));

    metadata
}

fn create_component(
    full_name: String,
    commit_hash: String,
    cataloger: Option<String>,
    sub_path: Option<String>,
) -> Component {
    let component_type: ComponentType = ComponentType::Application;
    let name = String::from(".");
    let purl = generate_purl(full_name, commit_hash, cataloger, sub_path);

    let mut component: Component = Component::new(component_type, name);
    component.purl = Some(purl);

    component
}

/// Syft Service
pub(crate) struct Service {
    source_path: String,
}

impl Service {
    fn run_syft(&self, cataloger_opt: Option<String>) -> Result<Output, Error> {
        let mut command = Command::new("syft");
        let command = command.args(["--output", CYCLONEDX_JSON_FORMAT]);

        if let Some(cataloger) = cataloger_opt {
            command.args(["--catalogers", cataloger.as_str()]);
        }

        let output = command
            .arg(self.source_path.as_str())
            .output()
            .map_err(Error::Io)?;

        Ok(output)
    }

    /// Conventional Constructor
    pub fn new(source_path: String) -> Self {
        Service { source_path }
    }

    /// Invokes the syft CLI against the cloned repository to generate an SBOM.
    pub(crate) fn execute(
        &self,
        full_name: String,
        version: String,
        cataloger: Option<String>,
        sub_path: Option<String>,
    ) -> Result<String, Error> {
        let mut orig_dir = None::<PathBuf>;

        if let Some(path) = sub_path.clone() {
            let absolute_path = format!("{}/{}", self.source_path, path.as_str());
            let file_path = Path::new(&absolute_path);
            orig_dir = Some(env::current_dir().map_err(Error::Io)?);

            println!("==> about to run Syft in: {:#?}", file_path);
            env::set_current_dir(absolute_path).map_err(Error::Io)?
        }

        let output = self.run_syft(cataloger.clone())?;

        // Handle error generated by syft.
        if !&output.status.success() {
            let error_msg = String::from_utf8(output.stderr).map_err(Error::Utf8Conversion)?;
            return Err(Error::Syft(error_msg));
        }

        if output.stdout.is_empty() {
            return Err(Error::Syft("syft generated empty SBOM".to_string()));
        };

        let output: String = String::from_utf8_lossy(&output.stdout).to_string();
        let mut sbom: Bom = Bom::parse(output.as_str(), CdxFormat::Json)
            .map_err(Error::Core)?;

        let metadata = ensure_purl_in_metadata(
            sbom.clone(),
            full_name,
            version,
            cataloger,
            sub_path
        );

        sbom.metadata = Some(Box::new(metadata));

        if let Some(path) = orig_dir {
            println!(
                "==> about to change directories back to: {:#?}\n",
                path.to_str().unwrap()
            );

            env::set_current_dir(path).map_err(Error::Io)?
        }

        let sbom_value = serde_json::to_value(sbom).map_err(Error::Serde)?;
        let string_value = serde_json::to_string(&sbom_value).map_err(Error::Serde)?;
        Ok(string_value)
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

/// Errors specific to Syft operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Error used when an error is caused within the module that
    /// did not originate from another library or module.
    #[error("syft: {0}")]
    Syft(String),

    /// Handle errors that come from the IO system
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Handle errors that come from UTF-8 conversion
    #[error(transparent)]
    Utf8Conversion(#[from] FromUtf8Error),

    /// Handle errors that come from JSON Processing
    #[error(transparent)]
    Serde(#[from] serde_json::error::Error),

    /// Handle errors from core functions
    #[error(transparent)]
    Core(#[from] CoreError)
}

/// Module for testing
#[cfg(test)]
mod test {

    use platform::filesystem::get_tmp_location;
    use platform::git::Service as Git;
    use std::collections::HashMap;

    use crate::entities::cyclonedx::bom::BomFormat;
    use crate::entities::cyclonedx::component::ComponentType;
    use crate::entities::cyclonedx::{Bom, Component, Metadata};
    use crate::entities::sboms::{Author, CdxFormat, Sbom};
    use crate::entities::xrefs::{Xref, XrefKind};
    use crate::services::syft::{
        create_component, create_metadata, ensure_purl_in_metadata, generate_purl, Error,
        Service as Syft, CATALOGERS,
    };
    use crate::testing::sbom_raw;

    fn test_created_component(
        test_component: Component,
        full_name: String,
        commit_hash: String,
        purl_type: Option<String>,
        sub_path: Option<String>,
    ) {
        let purl = test_component.purl.unwrap();
        assert!(purl.contains(full_name.as_str()));
        assert!(purl.contains(commit_hash.as_str()));
        assert!(purl.contains(purl_type.unwrap().as_str()));
        assert!(purl.contains(sub_path.unwrap().as_str()));
    }

    fn get_test_bom(metadata: Option<Box<Metadata>>) -> Bom {
        Bom {
            dollar_schema: None,
            bom_format: BomFormat::CycloneDx,
            spec_version: "1.4".to_string(),
            serial_number: None,
            version: 1,
            metadata,
            components: None,
            services: None,
            external_references: None,
            dependencies: None,
            compositions: None,
            vulnerabilities: None,
            signature: None,
        }
    }

    fn get_test_data(cataloger_opt: Option<&str>) -> (String, String, String, String, String) {
        let full_name = String::from("test/name");
        let commit_hash = String::from("abc123");

        let cataloger = match cataloger_opt {
            None => String::from("rust-cargo-lock-cataloger"),
            Some(cataloger) => String::from(cataloger),
        };

        let purl_type = CATALOGERS.get(cataloger.as_str()).unwrap().to_string();
        let sub_path = String::from("sub/path");
        (full_name, commit_hash, purl_type, cataloger, sub_path)
    }

    #[test]
    fn test_ensure_purl_no_metadata() {
        let sbom: Bom = get_test_bom(None);
        let (full_name, commit_hash, purl_type, cataloger, sub_path) = get_test_data(None);

        let created_metadata = ensure_purl_in_metadata(
            sbom,
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger.clone()),
            Some(sub_path.clone()),
        );

        let unboxed_component = *created_metadata.component.unwrap();

        test_created_component(
            unboxed_component,
            full_name.clone(),
            commit_hash.clone(),
            Some(purl_type.clone()),
            Some(sub_path.clone()),
        );
    }

    #[test]
    fn test_ensure_purl_no_component() {
        let sbom: Bom = get_test_bom(Some(Box::new(Metadata::new())));
        let (full_name, commit_hash, purl_type, cataloger, sub_path) = get_test_data(None);

        let created_metadata = ensure_purl_in_metadata(
            sbom,
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger.clone()),
            Some(sub_path.clone()),
        );

        let unboxed_component = *created_metadata.component.unwrap();

        test_created_component(
            unboxed_component,
            full_name.clone(),
            commit_hash.clone(),
            Some(purl_type.clone()),
            Some(sub_path.clone()),
        );
    }

    #[test]
    fn test_ensure_purl_no_purl() {
        let (full_name, commit_hash, purl_type, cataloger, sub_path) = get_test_data(None);

        let component_type: ComponentType = ComponentType::Application;
        let name = String::from(".");
        let component: Component = Component::new(component_type, name);

        let mut metadata = Metadata::new();
        metadata.component = Some(Box::new(component));

        let sbom: Bom = get_test_bom(Some(Box::new(metadata)));

        let created_metadata = ensure_purl_in_metadata(
            sbom,
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger.clone()),
            Some(sub_path.clone()),
        );

        let unboxed_component = *created_metadata.component.unwrap();

        test_created_component(
            unboxed_component,
            full_name.clone(),
            commit_hash.clone(),
            Some(purl_type.clone()),
            Some(sub_path.clone()),
        );
    }

    #[test]
    fn test_ensure_purl_unrecognized_cataloger() {
        let sbom: Bom = get_test_bom(Some(Box::new(Metadata::new())));
        let (full_name, commit_hash, _, _, sub_path) = get_test_data(None);

        let cataloger = String::from("unrecognized_cataloger");
        let purl_type = String::from("no-type");

        let created_metadata = ensure_purl_in_metadata(
            sbom.clone(),
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger.clone()),
            Some(sub_path.clone()),
        );

        let unboxed_component = *created_metadata.component.unwrap();

        test_created_component(
            unboxed_component,
            full_name.clone(),
            commit_hash.clone(),
            Some(purl_type.clone()),
            Some(sub_path.clone()),
        );
    }

    #[test]
    fn test_create_metadata() {
        let (full_name, commit_hash, purl_type, cataloger, sub_path) = get_test_data(None);

        let test_metadata: Metadata = create_metadata(
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger.clone()),
            Some(sub_path.clone()),
        );

        assert!(test_metadata.component.is_some());

        let test_component = *test_metadata.component.unwrap();

        test_created_component(
            test_component,
            full_name.clone(),
            commit_hash.clone(),
            Some(purl_type.clone()),
            Some(sub_path.clone()),
        );
    }

    #[test]
    fn test_create_component() {
        let (full_name, commit_hash, purl_type, cataloger, sub_path) = get_test_data(None);

        let test_component: Component = create_component(
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger.clone()),
            Some(sub_path.clone()),
        );

        test_created_component(
            test_component,
            full_name.clone(),
            commit_hash.clone(),
            Some(purl_type.clone()),
            Some(sub_path.clone()),
        );
    }

    #[test]
    fn test_create_component_no_subpath() {
        let (full_name, commit_hash, purl_type, cataloger, _) = get_test_data(None);

        let test_component: Component = create_component(
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger.clone()),
            None,
        );

        let purl = test_component.purl.unwrap();

        assert!(purl.contains(full_name.as_str()));
        assert!(purl.contains(commit_hash.as_str()));
        assert!(purl.contains(purl_type.as_str()));
    }

    #[test]
    fn test_generate_purl_with_sub_path() -> Result<(), Error> {
        let cataloger_str = "java-pom-cataloger";

        let (full_name, commit_hash, purl_type, cataloger, sub_path) =
            get_test_data(Some(cataloger_str));

        let test_purl_with_sub_path = format!(
            "pkg:{}/{}@{}#{}",
            purl_type, full_name, commit_hash, sub_path
        );

        let purl_with_sub_path =
            generate_purl(full_name, commit_hash, Some(cataloger), Some(sub_path));

        assert_eq!(purl_with_sub_path, test_purl_with_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_purl_no_sub_path() -> Result<(), Error> {
        let (full_name, commit_hash, purl_type, cataloger, _) = get_test_data(None);

        let test_purl_no_sub_path = format!("pkg:{}/{}@{}", purl_type, full_name, commit_hash);

        let purl_no_sub_path = generate_purl(
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger),
            None,
        );

        assert_eq!(purl_no_sub_path, test_purl_no_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_purl_root_sub_path() -> Result<(), Error> {
        let (full_name, commit_hash, purl_type, cataloger, _) = get_test_data(None);

        let sub_path = String::from("/");

        let test_purl_root_sub_path = format!("pkg:{}/{}@{}", purl_type, full_name, commit_hash);
        let purl_root_sub_path = generate_purl(
            full_name.clone(),
            commit_hash.clone(),
            Some(cataloger),
            Some(sub_path),
        );

        assert_eq!(purl_root_sub_path, test_purl_root_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_purl_root_empty_path() -> Result<(), Error> {
        let (full_name, commit_hash, purl_type, cataloger, _) = get_test_data(None);
        let sub_path = String::from("");

        let test_purl_empty_sub_path = format!("pkg:{}/{}@{}", purl_type, full_name, commit_hash);
        let purl_empty_sub_path =
            generate_purl(full_name, commit_hash, Some(cataloger), Some(sub_path));

        assert_eq!(purl_empty_sub_path, test_purl_empty_sub_path);

        Ok(())
    }

    #[test]
    fn test_generate_single() -> Result<(), Error> {
        let repo_url = "https://github.com/harbor-test-org/java-repo.git".to_string();
        let git = Git::new(repo_url);

        let repo_loc = get_tmp_location();

        let (full_name, commit_hash, _purl_type, cataloger, _) = get_test_data(None);

        let purl_under_test = generate_purl(
            full_name.to_string(),
            commit_hash.to_string(),
            Some(cataloger.clone()),
            None,
        );

        git.clone_repo(repo_loc.as_str(), None)
            .map_err(|err| Error::Syft(format!("{}", err)))?;

        let syft = Syft::new(repo_loc.clone());

        let syft_result = syft.execute(
            full_name,
            commit_hash,
            Some(cataloger),
            None,
        );

        Git::remove_clone(repo_loc.as_str()).map_err(|err| Error::Syft(format!("{}", err)))?;

        let sbom = Bom::parse(syft_result.unwrap().as_str(), CdxFormat::Json)
            .map_err(Error::Core)?;

        let metadata = sbom.metadata.unwrap();
        let component = metadata.component.unwrap();
        let purl_opt = component.purl;
        let purl = purl_opt.unwrap();

        assert_eq!(purl, purl_under_test);

        Ok(())
    }

    #[test]
    fn test_generate_multi() -> Result<(), Error> {
        let repo_url = "https://github.com/harbor-test-org/java-multi-module.git".to_string();
        let git = Git::new(repo_url);

        let repo_loc = get_tmp_location();
        let (full_name, commit_hash, _purl_type, cataloger, _) = get_test_data(None);

        git.clone_repo(repo_loc.as_str(), None)
            .map_err(|err| Error::Syft(format!("{}", err)))?;

        let poms: Vec<String> = git
            .find("pom.xml".to_string(), repo_loc.clone())
            .map_err(|err| Error::Syft(format!("{}", err)))?;

        let syft = Syft::new(repo_loc);

        for pom_path in poms.iter() {
            let trimmed_pom_path = pom_path.replace("/pom.xml", "");

            match syft.execute(
                full_name.clone(),
                commit_hash.clone(),
                Some(cataloger.clone()),
                Some(trimmed_pom_path.clone()),
            ) {
                Ok(sbom) => {
                    let purl_under_test = generate_purl(
                        full_name.clone(),
                        commit_hash.clone(),
                        Some(cataloger.clone()),
                        Some(trimmed_pom_path.to_string()),
                    );

                    let sbom = Bom::parse(sbom.as_str(), CdxFormat::Json)
                        .map_err(Error::Core)?;

                    let metadata = sbom.metadata.unwrap();
                    let component = metadata.component.unwrap();
                    let purl_opt = component.purl;
                    let purl = purl_opt.unwrap();

                    assert_eq!(purl, purl_under_test);
                }
                Err(err) => return Err(Error::Syft(format!("{}", err))),
            }
        }

        Ok(())
    }

    #[test]
    fn can_resolve_package_purl() -> Result<(), Error> {
        let raw = sbom_raw().map_err(|e| Error::Syft(e.to_string()))?;

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
        )
        .map_err(|e| Error::Syft(e.to_string()))?;

        match sbom.purl {
            None => {
                return Err(Error::Syft("could not resolve sbom purl".to_string()));
            }
            Some(purl) => {
                assert!(purl.starts_with("pkg:cargo"));
            }
        }

        Ok(())
    }
}
