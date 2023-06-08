## Service

The `mongodb::Service` trait provides consistent, generic persistence capabilities for types that 
implement the`MongoDocument` trait. It is specialized to the opinionated conventions adopted by the 
Harbor Team. It can be thought of as a pre-and-post-processor that ensures mandatory generic logic 
is consistently applied across all operations against a `Store`. Application code should not need 
to be aware of the `Store` and should perform operations against a `Service` instead.

### MongoDocument Trait

The `MongoDocument` trait must be applied to any entities you wish to persist to MongoDB as the 
root document of a collection. The trait provides two capabilities. First, it provides the 
`Store` a way to retrieve the type name of a passed struct dynamically at run time. The type 
name is used to resolve the collection the struct belongs to. Second, since Rust traits cannot 
contain fields, the trait provides default getter and setter functions for the `id` field. This 
is required, because the `Service` trait is designed around the convention that all entity keys 
are uniformly named.

This implies two things:

- Harbor's MongoDB collections will be named after the struct they contain.
- Entities must have a public `id` field of type `String`.


The `MongoDocument` trait can be applied to any conformant entity by applying the `mongo_doc` macro 
as show here.

```rust
use platform::persistence::mongodb::mongo_doc;

mongo_doc!(Package);
```

