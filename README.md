[![TESTS](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/tests.yaml/badge.svg)](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/tests.yaml) [![BACKEND-BUILD](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/build.yaml/badge.svg?branch=main&event=workflow_run)](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/build.yaml)
# Overview

This project contains the Harbor application and enrichment providers that support it.

## Status

Version 2 of this project is in early stages of development.  We are rapidly iterating towards a v2.0.0 MVP,
but at this time all features are not yet operational, and the usage documentation is not available.

## System Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Protobuf](https://grpc.io/docs/protoc-installation/)
- [docker](https://docs.docker.com/get-docker/) (optional - used for tests)
- [docker compose](https://docs.docker.com/compose/install/) (optional - used for local environment)
- [GitLeaks](https://github.com/gitleaks/gitleaks/tree/master#installing)
- [pre-commit](https://pre-commit.com/index.html#install)

## Environment

The following environment variables are referenced in code. When possible, defaults are provided that
support the `docker-compose` configuration found in the `sdk/devenv` folder.

- `SNYK_TOKEN` - A valid Snyk API token. Required if using the Snyk integrations.
- `HARBOR_FILE_STORE` - Path specification for file storage. When using an `S3StorageProvider`
  this should be the bucket name with path prefix where you wish to store generated files. When
  using a `FileSystemStorageProvider` this should be a valid directory on the host machine
  running the job.
- `DOCDB_CONFIG` - DocumentDB connection configuration. If not set, tests will default to the
  configuration that supports the `docker-compose.yaml` environment specified in the `sdk/devenv`
  folder. The primary Harbor installation is backed by DocumentDB, but any MongoDB 5.0 compliant
  database should be usable. Dynamic configuration is not yet implemented, but pull requests
  are welcome if community members need this capability before we can get to it. The
  current DocumentDB config expects a JSON document with the following schema:

```json
{
  "password":"<redacted>",
  "engine":"mongo",
  "port":27017,
  "dbInstanceIdentifier":"<documentdb-instance-identifier>",
  "host":"<documentdb-host-name>",
  "ssl":true,
  "username":"<redacted>"
}
```

## Getting Started as a Contributor

1. Clone the repository and `cd` into its directory.

```shell
git clone git@github.com:cms-enterprise/sbom-harbor`

cd sbom-harbor
```

2. Install git pre-commit hooks.

```shell
pre-commit install
```

3. Depending on your development environment, you may also need to add the following to your
   `/etc/hosts` file.

```shell
# Harbor DevEnv
127.0.0.1 mongo
```

## Crate Documentation

The documentation for each crate can be generated from source using `cargo` or `rustdoc`.

To generate the documentation, clone the repository, and then run the
following command from this directory.

```shell
cargo doc --no-deps
```

Documentation for each crate will be generated and output to the `target/doc` subdirectory.

## Building

To build all workspace targets on the local machine run the following from the root directory.

```shell
cargo build
```

To build a single crate run the following from the root directory.

```shell
cargo build --workspace -p <crate-name> # e.g. use harbor-api or harbor-cli as the final argument.
```

By default, this will produce a debug build in the `target/debug` directory. To produce a release
binary run the following.

```shell
cargo build --release
```

The release build can be found in the `target/release` directory.

## Try it out

There are several use cases addressed by this repository. The following sections detail how to try
out each one.

### Local Development Environment

If you wish to run Harbor locally using the development environment found in the `sdk/devenv` 
directory,
open a new terminal and run the following command.

```shell
cd sdk/devenv && docker compose up
```

### Sbom Ingestion & Enrichment

For organizations that use Snyk, Harbor can import SBOMs using the Snyk API. Make sure all environment
variables are set and then run the following command.

**Note:** this assumes you are running the command from the root directory of the repository and
that you have run a `release` build as described above.

```shell
./target/release/harbor-cli sbom -p snyk
```

Once you have ingested the SBOMs from the Snyk API, you can then use Harbor to call the API for all
identified packages, and store any known vulnerability findings for each package.

```shell
./target/release/harbor-cli enrich -p snyk
```

If you wish to run the above commands against the local development environment provided in
the `sdk/devenv` directory, add the `--debug` flag.

```shell
./target/release/harbor-cli sbom --debug -p snyk
```
