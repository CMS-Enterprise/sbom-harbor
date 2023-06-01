## Design Guidelines

The following sections contain a non-exhaustive list of design guidelines to consider when 
contributing to Harbor. Suggestions for additional guiding principles welcome!

## Code Organization

```
workspace/
├── api/
├── cli/
├── core/
└── platform/
```

### Modules

- `mod.rs` - This file should contain exports, unit tests, and types that are shared or fundamental 
  to the module as a whole. Examples of fundamental types include:
  - Enums
  - Traits
  - Errors

### Platform

Code in the `platform` crate is intended to be entirely generic and unrelated to the Harbor 
application. It should be thought of as a separate reusable library that can be leveraged by any 
application. When contributing or reviewing new features or modifications to the `platform`crate 
it is imperative to keep this in mind. 

### Services

- Services can consume external models or types but should not expose them. Make sure your
  service does not expose a non-Harbor type outside the service's containing module.
- If a function does not rely on shared state, consider making it a module level function 
  instead. It will likely be easier to test.
- If you need to repeatedly use a type that is expensive to create, such as an TLS-enabled HTTP 
  Client or a DB driver that establishes long-lived connections, consider wrapping it in a service.

## Task Providers

Some specific things to consider when designing a `TaskProvider` are:

- What kind of task is it?
  - Can one of the existing enumerations be extended or is it an altogether new thing?
  - Tasks typically map to a `cli` command. Does the new task naturally fit under an existing 
    command or is a new command needed?
- If you experience and unrecoverable error, make sure to set the error message on the `Task` entity
  so that the `Task.Status` is set to `failed`.
- Can the task tolerate partial failures?
  - If so, add errors to the `Task` entity to debug recoverable errors during a task run.
  - If not, set the error message on the `Task` entity so that the `Task.Status` is set to `failed`.
- Should affected entities maintain a reference to `Task` instances?
  - If so, add `task_refs` entries to entities affected by a run of a `TaskProvider` instance.

