//! Error type

use std::borrow::Cow;
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
    #[error("Parser error [slice: {0:?}, str: {}]", vec_to_string(.0))]
    Parser(Vec<u8>),
}

pub fn create_custom_error(msg: String) -> Error {
    Error::Custom(msg)
}

fn vec_to_string(v: &Vec<u8>) -> String {
    std::string::String::from_utf8_lossy(&v[..]).into_owned()
}
