use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error running pilot")]
    Pilot(String),
}
