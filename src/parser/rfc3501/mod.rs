pub(crate) mod core;
pub(crate) mod grammar;

use self::core::base64;
use crate::parser::types::{
    ContinueReq, Greeting, GreetingStatus, TaggedResponse, UntaggedResponse,
};
use grammar::{imap_tag, resp_cond_auth, resp_cond_bye, resp_cond_state, resp_text};
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::crlf,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

// greeting = "*" SP (resp_cond_auth | resp_cond_bye) CRLF
pub(crate) fn greeting(i: &[u8]) -> IResult<&[u8], Greeting<'_>> {
    map(
        delimited(
            tag("* "),
            alt((
                map(resp_cond_auth, |(status, resp_text)| {
                    if status == "OK" {
                        GreetingStatus::Ok(resp_text)
                    } else {
                        GreetingStatus::Preauth(resp_text)
                    }
                }),
                map(resp_cond_bye, |bye_resp| GreetingStatus::Bye(bye_resp)),
            )),
            crlf,
        ),
        |status| Greeting { status },
    )(i)
}

// continue-req = '+' SP (resp-text | base64) CRLF
pub(crate) fn continue_req(i: &[u8]) -> IResult<&[u8], ContinueReq<'_>> {
    delimited(
        tag("+ "),
        alt((
            map(resp_text, |text| ContinueReq::Text(text)),
            map(base64, |base64| ContinueReq::Base64(base64)),
        )),
        crlf,
    )(i)
}

// response-tagged = tag SP resp-cond-state CRLF
pub(crate) fn response_tagged(i: &[u8]) -> IResult<&[u8], TaggedResponse<'_>> {
    map(
        tuple((imap_tag, tag(" "), resp_cond_state, crlf)),
        |(tag, _, resp, _)| TaggedResponse { tag, resp },
    )(i)
}

//response-data = '*' SP (resp-cond-state | resp-cond-bye | mailbox-data |
//                        message-data | capability-data) CRLF
pub(crate) fn response_data(i: &[u8]) -> IResult<&[u8], UntaggedResponse<'_>> {
    delimited(
        tag("* "),
        alt((
            map(resp_cond_state, |res| UntaggedResponse::RespCond(res)),
            map(resp_cond_bye, |res| UntaggedResponse::RespBye(res)),
        )),
        crlf,
    )(i)
}
