## Store

The `Store` is a facade that provides a coarsely-grained way to perform common operations 
against a MongoDB compliant data store. The `Store` is designed to work generically against any 
type that implements the `MongoDocument` trait. Harbor application code should typically not 
reference a`Store` directly, but instead should call a type that implements the `Service` trait.

The `Store` is the type that wraps the long-lived MongoDB client. Because of that, the `Store` 
should also be long-lived. You should wrap the `Store` in an `Arc` and clone the `Arc` whenever 
you need to access the `Store`. There are numerous examples that can be referenced already in 
the codebase.
