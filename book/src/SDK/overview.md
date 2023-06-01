## SDK

The Harbor SDK is currently comprised of a docker-compose based development environment, the
`platform` crate, an OpenAPI specification, and code generators. Details of each can be found in 
their respective subsections, but a brief overview of each is as follows.

### Development Environment

A fully functional development environment can be found in the `devenv` directory. To run the 
environment, run `docker compose up` from that directory.

### Platform crate

The `platform` crate contains reusable functionality related to completely generic programming 
tasks that are not and should not be coupled to the Harbor domain. Externalizing these features to
their own crate is designed to help promote code reuse, simplify dependency management, provide 
consistent implementations of common functions, and enforce a strict separation of concerns. 

### OpenAPI

The `openapi` directory contains an [OpenAPI Specification](https://www.openapis.org/) for 
Harbor. It also contains scripts and config that we use to generate our [UI](https://github.com/CMS-Enterprise/sbom-harbor-ui)
client as well as some tests for the `api`. The scripts use the [OpenAPI Generator](https://openapi-generator.tech/)
tool and may be useful as a model for organizations that need or want to generate a client for the 
Harbor API.

### Generators

The `generators` directory includes some simple code generation tooling built using 
[`cargo generate`](https://cargo-generate.github.io/cargo-generate/). The generators can be 
used to generate scaffolding for api endpoints, new cli commands, and task providers. The 
generators are not designed to provide complete, robust implementations as much as opinionated 
guidance and awareness relative to project structure and conventions.

