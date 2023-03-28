mod pilot;

pub use pilot::*;

/// Allows specifying the output format.
#[derive(Clone)]
pub enum OutputFormat {
    /// Output as JSON.
    Json,
    /// Output as plaintext.
    Text,
}

/// Generic trait that all command options must implement.
trait Opts {
    fn format(&self) -> OutputFormat {
        OutputFormat::Text
    }
}

/// Generic Command trait that all command handlers must implement.
trait Command<T>
where T: Opts + Send + Sync
{
    fn execute(opts: T) -> i32;
}
