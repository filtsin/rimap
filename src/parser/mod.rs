//! IMAP parser implementation

mod rfc3501;
#[cfg(test)]
mod tests;
mod types;

use rfc3501::{continue_req, greeting, response_tagged};
use types::ImapResponse;

use nom::{branch::alt, combinator::map, IResult};

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], ImapResponse<'_>> {
    alt((
        map(greeting, |v| ImapResponse::Greeting(v)),
        map(continue_req, |v| ImapResponse::Continue(v)),
        map(response_tagged, |v| ImapResponse::Response(v)),
    ))(i)
}
