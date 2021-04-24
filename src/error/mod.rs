//! Error type

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// A set of errors
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("This is simple example message")]
    ErrorExample,
    #[error("An IO error")]
    Io(#[from] std::io::Error),
}
