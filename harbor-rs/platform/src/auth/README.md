## Overview

The `auth` module provides a reusable struct model and `Authorizer` trait that can be used with any datastore in the case
that you need to role your own RBAC. It is based on the AWS IAM model. The `Authorizer` trait is decoupled from the
underlying datastore. Consumers of this crate have to implement their own data storage/access strategy. See the
`DefaultAuthorizer` in the `mongo` package in this crate for an example.

See the `mongodb.auth.init_default_auth` module for a re-usable database migration that will initialize
an opinionated authorization configuration.

This package does not currently provide any authentication features but likely will.

```mermaid
classDiagram
    Group "1" --> "1..*" User
    Group "1" --> "1..*" Role
    Role "1" --> "1..*" Policy
    Policy "1" --> "1" Resource
    Policy "1" --> "1" Action
    Policy "1" --> "1" Effect
    Resource "1" --> "1" ResourceKind

    class Authorizer {
        <<Service>>
        +Bool assert(User, Resource, Effect)
        -List~Group~ groups(User)
    }

    class User {
      +String id
      +String email
    }

    class Group {
      +List~User~
      +List~Role~
    }

    class Role {
      List~Policy~
    }

    class Policy {
        +Resource
        +Action
        +Effect
    }

    class Resource {
        +String id
        +ResourceKind
    }

    class ResourceKind
    <<enumeration>> ResourceKind

    class Action
    <<enumeration>> Action

    class Effect
    <<enumeration>> Effect
```
