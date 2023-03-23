use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("config error")]
    Config(String),
    #[error("invalid format")]
    Format(String),
    #[error("migrations error")]
    Migrations(String),
    #[error("runtime error")]
    Runtime(String),
}


impl From<platform::Error> for Error {
    fn from(value: platform::Error) -> Self {
        Error::Runtime(format!("{:?}", value))
    }
}
