use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod client;
pub mod resource;
pub mod server;
pub mod session;
pub mod tensorflow;

#[derive(Debug, Deserialize, Serialize, Error)]
pub enum Error {
    /// An invalid argument was passed by the user
    #[error("Invalid argument")]
    InvalidArgument,
    /// Error while performing I/O
    #[error("I/O error")]
    IOError,
    /// Undefined error
    #[error("BUG: Undefined error")]
    UndefinedError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Error {
        Error::IOError
    }
}
