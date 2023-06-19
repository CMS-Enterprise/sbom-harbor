## SDK

The Harbor SDK is currently comprised of a docker-compose based development environment, the
`platform` and `core` crates, an OpenAPI specification, a custom extension template, and code 
generators. Details of the `platform` and `core` crates can be found in their respective 
subsections, but an overview of the other SDK resources is included below.

### Development Environment

A fully functional development environment can be found in the `devenv` directory. To run the 
environment, run `docker compose up` from that directory.

### Custom Extension Template

Operating environments will differ by organization. Harbor cannot anticipate every SBOM ingestion or
enrichment source or use case. Likewise, it cannot define a data model that meets the needs of every
imaginable scenario or is compatible with all existing and emerging SBOM specifications. What it can
and does do is define a core domain model related to the ingestion, storage, enrichment, and analysis
of SBOMs, and exposes mechanisms of extension.

The `TaskProvider` trait found in the `core` crate is specifically designed to enable 
extensibility from the beginning. If the massive success of [HashiCorp](https://www.hashicorp.com/)
[Terraform](https://www.terraform.io/) is any indicator, the engineering community prefers 
solutions they can extend and customize. Terraform adopts a [Provider](https://developer.hashicorp.com/terraform/language/providers)
model for enabling plugins. This is conceptually, is how we envision building and extending the 
upstream Harbor feature set. Contributions to upstream Harbor must conform to this model to be
considered. 

Community members that need to build solutions specific to their operating environment or use 
case(s) can use the `extension-template` example project found in the `sdk` directory to create 
a custom CLI that can be invoked by their task orchestrator. It includes an example of how to
leverage functionality exposed by the `core` crate in your custom Rust code. 

Organizations that cannot or do not want to write custom providers in Rust can leverage the OpenAPI
specification to write providers in other languages.

### OpenAPI

The `sdk/openapi` directory contains an [OpenAPI Specification](https://www.openapis.org/) for 
Harbor. It also contains scripts and config that we use to generate our [UI](https://github.com/CMS-Enterprise/sbom-harbor-ui)
client as well as some tests for the `api`. The scripts use the [OpenAPI Generator](https://openapi-generator.tech/)
tool and may be useful as a model for organizations that need or want to generate a client for the 
Harbor API.

### Generators

The `sdk/generators` directory includes some simple code generation tooling built using 
[`cargo generate`](https://cargo-generate.github.io/cargo-generate/). The generators can be 
used to generate scaffolding for api endpoints, new cli commands, and task providers. The 
generators are not designed to provide complete, robust implementations as much as opinionated 
guidance and awareness relative to project structure and conventions.

