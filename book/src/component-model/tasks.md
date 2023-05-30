## Tasks

A `Task` records an instance of a `TaskProvider` process that is run against a Harbor instance. 


Some task

You can  think of this as unit of work. The enrichment engine is modeled as a set of 
`TaskProvider` services. You can build a service that implements the `TaskProvider` trait and record 
details about each instantiation of the process using a `Task` entity.

The `TaskProvider` trait has the following interface. 

```rust
pub trait TaskProvider: Service<Task> {
    /// Implement this with your custom logic.
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error>;

    /// Runs the transaction script and store the results. Usually invoked by a CLI command handler.
    async fn execute(&self, task: &mut Task) -> Result<(), Error> {
        // See codebase for default implementation. You should probably not need to override this.
    }

    /// Inserts the [Task] record at the start of the transaction script.
    async fn init(&self, task: &mut Task) -> Result<(), Error> {
        // See codebase for default implementation. You should probably not need to override this.
    }

    /// Updates the [Task] record at the end of the transaction script.
    async fn complete(&self, task: &mut Task) -> Result<(), Error> {
        // See codebase for default implementation. You should probably not need to override this.
    }
}
```

The default implementations of the `execute`, `init`, and`complete` function provide a consistent way 
to track and debug `Task` services. You most likely only need to implement the `run` function. This 
is where your custom business logic runs. You can review existing `TaskProvider` implementations 
for example of how to implement a `TaskProvider`. 

## Implementation Considerations

Some specific things you should consider when designing a `TaskProvider` are:

### What kind of task is it?


### Can the `Task` tolerate partial failures


### Should affected entities maintain a reference to `Task` instances?

- Adding errors to the `Task` entity for debugging purposes.
- Adding `TaskRef` entries to entities affected by a run of a `TaskProvider` instance.
