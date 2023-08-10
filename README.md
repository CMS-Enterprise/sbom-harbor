[![TESTS](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/tests.yaml/badge.svg)](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/tests.yaml) [![BACKEND-BUILD](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/build.yaml/badge.svg?branch=main&event=workflow_run)](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/build.yaml)
# Overview

This project contains the Harbor application and enrichment providers that support it. Please refer
to our [book](https://cms-enterprise.github.io/sbom-harbor/) for a comprehensive explanation.

## Status

Harbor is under active development. We are currently working at a `0.1.0` pre-release semver level.
There is no guarantee of stable interfaces or backward compatability at this time. We would be
thrilled to have additional contributors and end-users, but we want to make sure you are aware
of that before you decide to invest your time and resources.

## Developer System Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Protobuf](https://grpc.io/docs/protoc-installation/)
- [docker](https://docs.docker.com/get-docker/) (optional - used for tests)
- [docker compose](https://docs.docker.com/compose/install/) (optional - used for local environment)
- [GitLeaks](https://github.com/gitleaks/gitleaks/tree/master#installing)
- [pre-commit](https://pre-commit.com/index.html#install)
- [direnv](https://direnv.net/)
- [aws cli](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)

## Environment

The following environment variables are referenced in code. When possible, defaults are provided that
support the `docker-compose` configuration found in the `sdk/devenv` folder.

- `SNYK_TOKEN` - A valid Snyk API token. Required if using the Snyk integrations.
- `HARBOR_FILE_STORE` - Path specification for file storage. When using an `S3StorageProvider`
  this should be the bucket name with path prefix where you wish to store generated files. When
  using a `FileSystemStorageProvider` this should be a valid directory or volume on the host machine
  or container running the job.
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

Secrets are programattically pulled into the environment via `direnv` and the script in `sdk/devenv/.envrc`. On the terminal, when you `cd sdk/devenv`, the `direnv` shell extension will automatically load the secrets into the  necessary environment variables. Once you change to another directory they will be automatically unloaded.
1. Copy `sdk/devenv/.env.example` into `sdk/devenv/.env` 
2. Add values for the aws profile and secret names
3. `cd sdk/devenv`
4. `direnv allow .`

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

## Project Documentation

Project documentation and additional developer guidance can be found on our [GitPage](https://cms-enterprise.github.io/sbom-harbor/).

## Crate Documentation

The documentation for each crate can be generated from source using `cargo` or `rustdoc`. We
plan to integrate the `rustdoc` output with the[project documentation](#project-documentation)
in time. However, that requires additional tooling that we haven't gotten to yet. That would
make a great first contribution. If you are willing, a PR will definitely be considered.

To generate the crate documentation, clone the repository, and then run the
following command from the root directory.

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

Many teams at CMS have been onboarded to Snyk. That fact made a Snyk integration an appealing
first target. Currently, Harbor supports ingesting SBOMs using the Snyk API. A generic GitHub
ingestion provider is imminent. Similarly, an enrichment provider based on an Open Source
vulnerability data provider is on the short-term roadmap. Stay tuned for updates on how to get
started with purely Open Source tools.

Make sure all environment variables are set and then run the following command.

**Note:** this assumes you are running the command from the root directory of the repository and
that you have run a `release` build as described above.

```shell
./target/release/harbor sbom -p snyk
```

Once you have ingested the SBOMs from the Snyk API, you can then use Harbor to call the API for all
identified packages, and store any known vulnerability findings for each package.

```shell
./target/release/harbor enrich -p snyk
```

If you wish to run the above commands against the local development environment provided in
the `sdk/devenv` directory, add the `--debug` flag.

```shell
./target/release/harbor sbom --debug -p snyk
```
