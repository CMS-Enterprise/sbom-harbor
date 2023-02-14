use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Config Error")]
    Config(String),
    #[error("Git Error")]
    Git(String),
    #[error("Hyper Error")]
    Hyper(String),
    #[error("IO Error")]
    Io(String),
    #[error("Parse Error")]
    Parse(String),
    #[error("Pilot Error")]
    Pilot(String),
    #[error("Serde Error")]
    Serde(String),
    #[error("Syft Error")]
    Syft(String),
}

impl From<aqum::hyper::Error> for Error {
    fn from(value: aqum::hyper::Error) -> Self {
        Error::Hyper(format!("{:?}", value))
    }
}
