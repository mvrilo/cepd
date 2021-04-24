use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cache miss")]
    CacheMiss,

    #[error("not found")]
    NotFound,

    #[error("decode error")]
    Decode,

    #[error("internal error")]
    Internal,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        if err.is_decode() {
            Error::Decode
        } else {
            Error::Internal
        }
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Error {
        Error::Internal
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Decode
    }
}