//! Error type

use thiserror::Error;

/// A convenience wrapper around `Result` for [Error][Error]
pub type Result<T> = std::result::Result<T, Error>;

/// A set of errors
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Fail write or read to a network stream
    #[error("An IO error")]
    Io(#[from] std::io::Error),
    /// Error with custom message
    #[error("An error has occured: {0}")]
    Custom(String),
}

pub fn create_custom_error(msg: String) -> Error {
    Error::Custom(msg)
}
