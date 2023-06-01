## Platform Crate

The `platform` crate contains reusable functionality related to completely generic programming
tasks that are not and should not be coupled to the Harbor domain. Externalizing these features to
their own crate is designed to help promote code reuse, simplify dependency management, provide
consistent implementations of common functions, and enforce a strict separation of concerns.

Code in this crate typically falls into one of three categories:

- features that can be mentally modeled as extensions to the Rust standard library
- features that reduce boilerplate
- features that encapsulate infrastructure resources like databases, storage systems, and networks.

Examples of the types code that should belong in this crate include:

- Cryptography
- Encoding
- Networking
- Database access & migrations
- Authorization/Authentication

> ### A Note on Cargo Features
> Harbor does not currently implement `cargo` features, but has been designed in a way that
> should allow introducing them with minimal refactoring. Keeping this in mind when developing,
> especially when working with the `platform` crate is an effective mental discipline that can help
> in designing modules that enforce a strict separation of concerns.


