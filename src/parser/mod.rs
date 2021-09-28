//! IMAP parser implementation

mod rfc3501;
#[cfg(test)]
mod tests;
mod types;

use nom::{combinator::map, IResult};
use rfc3501::greeting;
use types::ImapResponse;

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], ImapResponse<'_>> {
    map(greeting, |v| ImapResponse::Greeting(v))(i)
}
