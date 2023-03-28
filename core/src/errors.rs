use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("config error: {0}")]
    Config(String),
    #[error("invalid format: {0}")]
    Format(String),
    #[error("migrations error: {0}")]
    Migrations(String),
    #[error("runtime error: {0}")]
    Runtime(String),
}


impl From<platform::Error> for Error {
    fn from(value: platform::Error) -> Self {
        Error::Runtime(format!("{:?}", value))
    }
}
