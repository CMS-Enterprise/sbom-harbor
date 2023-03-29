[![TESTS - Lint, Unit, Integration](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/tests.yaml/badge.svg)](https://github.com/CMS-Enterprise/sbom-harbor/actions/workflows/tests.yaml)

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

## Environment

The following environment variables are referenced in code. When possible, defaults are provided that
support the `docker-compose` configuration found in the `devenv` folder.

- `DB_CONNECTION` - Mongo connection configuration. If not set, tests will default to the configuration that supports the
  `docker-compose.yaml` environment specified in the `devenv` folder.

Expects a JSON document with the following schema:

```json
{
  "host": "<instance host name/resolvable DNS name>",
  "username": "<username>",
  "password": "<password>",
  "port": "<TCP port number>"
}
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
