use crate::Error;
use platform::config::{from_env, type_from_env};
use platform::mongodb::{Context, ContextKind};
use serde::__private::de::IdentifierDeserializer;
use std::str::FromStr;

pub enum Environ {
    Local,
    CI,
    Dev,
    Prod,
}

impl FromStr for Environ {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let environ = s.to_lowercase();
        match environ.as_str() {
            "local" => Ok(Self::Local),
            "ci" => Ok(Self::CI),
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            _ => Ok(Self::Local),
        }
    }
}

/// Returns a Mongo Context based on environment. Defaults to local dev environment if not set.
/// Optionally accepts a DB name. Defaults to `harbor`. Useful for overriding in tests.
pub fn mongo_context(db_name: Option<&str>) -> Result<Context, Error> {
    let db_name = match db_name {
        None => "harbor",
        Some(db_name) => db_name,
    };

    let mut cx = Context {
        kind: ContextKind::Mongo,
        host: "".to_string(),
        username: "".to_string(),
        password: "".to_string(),
        port: 0,
        db_name: db_name.to_string(),
        key_name: "id".to_string(),
        region: None,
        cluster_name: None,
        account_name: None,
    };

    build_context(&mut cx)?;

    Ok(cx)
}

fn build_context(cx: &mut Context) -> Result<(), Error> {
    let environ = environment();

    match environ {
        Environ::Local => {
            cx.host = "localhost".to_string();
            cx.port = 27017;
            cx.username = "root".to_string();
            cx.password = "harbor".to_string();
        }
        Environ::CI => {
            cx.host = "mongo".to_string();
            cx.port = 27017;
            cx.username = "root".to_string();
            cx.password = "harbor".to_string();
        }
        Environ::Dev | Environ::Prod => {
            cx.port = 27017;
            cx.username = from_env("DOCDB_USERNAME").expect("username required");
            cx.password = from_env("DOCDB_PASSWORD").expect("password required");
            cx.cluster_name = Some(from_env("DOCDB_CLUSTER_NAME").expect("cluster name required"));
            cx.region = Some(from_env("DOCDB_REGION").expect("region required"));
        }
    };

    Ok(())
}

/// Returns the runtime environment from environment variables. Defaults to [Environ::Local].
pub fn environment() -> Environ {
    match from_env("ENVIRONMENT") {
        None => Environ::Local,
        Some(environ) => match Environ::from_str(environ.as_str()) {
            Ok(e) => e,
            Err(e) => Environ::Local,
        },
    }
}

/// Returns the Snyk API token from an environment variable.
pub fn snyk_token() -> Result<String, Error> {
    match from_env("SNYK_TOKEN") {
        None => Err(Error::Config("Snyk token not set".to_string())),
        Some(v) => Ok(v),
    }
}

/// Returns the SBOM Upload S3 Bucket name.
pub fn sbom_upload_bucket() -> Result<String, Error> {
    match from_env("SBOM_UPLOAD_BUCKET") {
        None => Err(Error::Config("SBOM upload bucket not set".to_string())),
        Some(v) => Ok(v),
    }
}
