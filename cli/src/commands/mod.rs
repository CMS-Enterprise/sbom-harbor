pub mod pilot;

use std::env;
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

/// Extracts the value of an environment variable
///
fn get_env_var(variable_name: &str) -> String {
    return match env::var(variable_name) {
        Ok(v) => v,
        Err(e) => panic!("{} is not set ({})", variable_name, e),
    };
}