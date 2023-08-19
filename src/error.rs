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

    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("hyper error")]
    Hyper(#[from] hyper::Error),
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
    fn from(_err: sled::Error) -> Error {
        Error::Internal
    }
}

impl From<bincode::Error> for Error {
    fn from(_err: bincode::Error) -> Error {
        Error::Decode
    }
}
