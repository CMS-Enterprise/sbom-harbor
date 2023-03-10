pub mod pilot;

pub use pilot::*;

#[derive(Clone)]
pub enum OutputFormat {
    Json,
    Text,
}

trait Opts {
    fn format(&self) -> OutputFormat {
        OutputFormat::Text
    }
}

///
trait Command<T>
where
    T: Opts + Send + Sync,
{
    fn execute(opts: T) -> i32;
}
