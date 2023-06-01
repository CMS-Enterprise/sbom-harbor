## Tasks

A `Task` records that an instance of a process was run in a Harbor environment. It may be helpful to
think of a task as unit of work. The enrichment engine is implemented as a set of services that 
implement the `TaskProvider` trait. You can build a service that implements the `TaskProvider` 
trait and unless you override the key default functions, Harbor will record details about each 
instantiation of the process by storing an instance of a `Task` entity.

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

The default implementations of the `execute`, `init`, and`complete` functions provide a consistent 
way to track and debug `Task` services. Contributors most likely only need to implement the `run` 
function. This is where task specific logic runs. Review existing `TaskProvider` implementations 
for example of how to implement a `TaskProvider`. 
