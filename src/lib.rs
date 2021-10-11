#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

pub mod resource;
pub mod server;
pub mod session;
pub mod tensorflow;

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    /// An invalid argument was passed by the user
    InvalidArgument,
    /// Undefined error
    UndefinedError,
    /// Error while performing I/O
    IOError,
}

pub type Result<T> = std::result::Result<T, Error>;
