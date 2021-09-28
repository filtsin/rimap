//! IMAP client implementation

#![warn(rust_2018_idioms, /*missing_docs,*/ missing_debug_implementations)]
#![allow(dead_code)] /* allow on develop stage */

pub mod client;
pub mod error;
mod imapconnection;
mod parser;
mod tag;
