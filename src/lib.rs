//! IMAP client implementation

#![warn(rust_2018_idioms, /*missing_docs,*/ missing_debug_implementations)]

pub mod client;
pub mod error;
mod imapconnection;
mod parser;
mod tag;
