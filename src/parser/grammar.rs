//! IMAP grammar [rfc3501]

use std::char::from_u32;

use crate::parser::response::ListFlag;
use crate::tag::Tag;

use super::response::{
    ByeResponse, Capability, ContinueReq, Flag, Greeting, GreetingStatus, ImapResult, ListMailBox,
    MailBoxData, RespCond, RespText, RespTextCode, TaggedResponse, UntaggedResponse,
};
use super::types::*;
use nom::branch::alt;
use nom::bytes::streaming::{tag, tag_no_case, take_while_m_n};
use nom::character::streaming::crlf;
use nom::combinator::{map, map_res, opt, value};
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

// greeting = "*" SP (resp_cond_auth | resp_cond_bye) CRLF
pub(super) fn greeting(i: &[u8]) -> IResult<&[u8], Greeting<'_>> {
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
pub(super) fn continue_req(i: &[u8]) -> IResult<&[u8], ContinueReq<'_>> {
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
pub(super) fn response_tagged(i: &[u8]) -> IResult<&[u8], TaggedResponse<'_>> {
    map(
        tuple((imap_tag, tag(" "), resp_cond_state, crlf)),
        |(tag, _, resp, _)| TaggedResponse { tag, resp },
    )(i)
}

//response-data = '*' SP (resp-cond-state | resp-cond-bye | mailbox-data |
//                        message-data | capability-data) CRLF
pub(super) fn response_data(i: &[u8]) -> IResult<&[u8], UntaggedResponse<'_>> {
    delimited(
        tag("* "),
        alt((
            map(resp_cond_state, |res| UntaggedResponse::RespCond(res)),
            map(resp_cond_bye, |res| UntaggedResponse::RespBye(res)),
        )),
        crlf,
    )(i)
}

//mailbox-data = 'FLAGS' SP flag-list | 'LIST' SP mailbox-list | 'LSUB' SP mailbox-list |
//               'SEARCH' *(SP nz-number) | 'STATUS' SP mailbox SP '(' [status-att-list] ')' |
//               number SP 'EXISTS' | number SP 'RECENT'
fn mailbox_data(i: &[u8]) -> IResult<&[u8], MailBoxData<'_>> {
    alt((
        map(preceded(tag_no_case("FLAGS "), flag_list), |flags| {
            MailBoxData::Flags(flags)
        }),
        map(preceded(tag_no_case("LIST "), mailbox_list), |list| {
            MailBoxData::List(list)
        }),
    ))(i)
}

//flag-list = '(' [flag *(SP flag)] ')'
fn flag_list(i: &[u8]) -> IResult<&[u8], Vec<Flag<'_>>> {
    delimited(
        tag("("),
        separated_list0(tag(" "), map(flag, Flag::from)),
        tag(")"),
    )(i)
}

//mailbox-list = '(' [mbx-list-flags] ')' SP (DQUOTE QUOTED-CHAR DQUOTE | nil) SP mailbox
fn mailbox_list(i: &[u8]) -> IResult<&[u8], ListMailBox<'_>> {
    // TODO: I do not like how it looks now
    map(
        tuple((
            delimited(tag("("), opt(mbx_list_flags), tag(")")),
            tag(" "),
            alt((
                delimited(tag("\""), take_while_m_n(1, 1, is_quoted_char), tag("\"")),
                tag_no_case("NIL"),
            )),
            tag(" "),
            mailbox,
        )),
        |(flags, _, delimiter, _, name)| ListMailBox {
            flags: match flags {
                Some(v) => v,
                None => vec![],
            },
            delimiter: unsafe { std::str::from_utf8_unchecked(delimiter) },
            name,
        },
    )(i)
}

//mailbox = 'INBOX' | astring
fn mailbox(i: &[u8]) -> IResult<&[u8], &str> {
    astring(i)
}

//mbx-list-flags = *(mbx-list-oflag SP) mbx-list-sflag *(SP mbx-list-oflag) |
//                 mbx-list-oflag *(SP mbx-list-oflag)
fn mbx_list_flags(i: &[u8]) -> IResult<&[u8], Vec<ListFlag<'_>>> {
    // Allow multiple mbx-list-sflag for easy writing of parser, it is not lethal
    separated_list1(tag(" "), alt((mbx_list_oflag, mbx_list_sflag)))(i)
}

//mbx-list-oflag = '\Noinferiors' | flag-extension;
// Other flags; multiple possible per LIST response
fn mbx_list_oflag(i: &[u8]) -> IResult<&[u8], ListFlag<'_>> {
    map(
        alt((
            map_res(tag_no_case("\\Noinferiors"), std::str::from_utf8),
            flag_extension,
        )),
        ListFlag::from,
    )(i)
}

//mbx-list-sflag = '\Noselect' | '\Marked' | '\Unmarked'
// Selectability flags; only one per LIST response
fn mbx_list_sflag(i: &[u8]) -> IResult<&[u8], ListFlag<'_>> {
    map(
        alt((
            tag_no_case("\\Noselect"),
            tag_no_case("\\Marked"),
            tag_no_case("\\Unmarked"),
        )),
        |flag| {
            // SAFETY: flag is \Noselect, \Marked or \Unmarked ASCII texts, so it is valid UTF-8
            let s = unsafe { std::str::from_utf8_unchecked(flag) };
            ListFlag::from(s)
        },
    )(i)
}

// tag(rfc) = 1*<any ASTRING-CHAR except '+'>
// tag(this) = ASTRING-CHAR number
// We use our own (tag)[tag::Tag] definition of tag
// with one prefix letter and u32 id
fn imap_tag(i: &[u8]) -> IResult<&[u8], Tag> {
    map(
        tuple((take_while_m_n(1, 1, is_astring_char), number)),
        |(letter, index)| {
            let prefix = from_u32(letter[0] as u32).unwrap();
            Tag::new(prefix, index)
        },
    )(i)
}

// resp-cond-state = ("OK" | "NO" | "BAD") SP resp_text;
// Status condition
fn resp_cond_state(i: &[u8]) -> IResult<&[u8], RespCond<'_>> {
    map(
        separated_pair(
            alt((
                value(ImapResult::Ok, tag_no_case("OK")),
                value(ImapResult::No, tag_no_case("NO")),
                value(ImapResult::Bad, tag_no_case("BAD")),
            )),
            tag(" "),
            resp_text,
        ),
        |(status, text)| RespCond { status, text },
    )(i)
}

// flag-keyword = atom
fn flag_keyword(i: &[u8]) -> IResult<&[u8], &str> {
    atom(i)
}

// flag-extension = '\' atom;
// Future expansion
fn flag_extension(i: &[u8]) -> IResult<&[u8], &str> {
    // TODO: incorrect removing suffix '\'
    map(tuple((tag("\\"), atom)), |(_, result)| result)(i)
}

// flag-perm = flag | '\*'
fn flag_perm(i: &[u8]) -> IResult<&[u8], Flag<'_>> {
    map(flag, Flag::from)(i)
}

// flag = '\Answered' | '\Flagged' | '\Deleted' | '\Seen' | '\Draft' | flag_keyword | flag_extension
fn flag(i: &[u8]) -> IResult<&[u8], &str> {
    // \Answered, \Flagged are handle by flag_extension parser
    alt((flag_extension, flag_keyword))(i)
}

// auth-type = atom
fn auth_type(i: &[u8]) -> IResult<&[u8], &str> {
    // TODO: Create enum for auth type
    atom(i)
}

// capability-data = "CAPABILITY" *(SP CAPABILITY) SP "IMAP4rev1" *(SP capability)
fn capability_data(i: &[u8]) -> IResult<&[u8], Vec<Capability<'_>>> {
    // Grammar is not exactly as in rfc3501.
    // Just take all capabilities delimited by space
    // hoping that IMAP4rev1 is present
    map(
        tuple((
            tag_no_case("CAPABILITY"),
            many1(map(tuple((tag(" "), capability)), |(_, data)| data)),
        )),
        |(_, capability)| capability,
    )(i)
}

// capability = ('AUTH=' auth-type) | atom;
fn capability(i: &[u8]) -> IResult<&[u8], Capability<'_>> {
    let auth_parser = map(
        tuple((tag_no_case("AUTH="), auth_type)),
        |(_, auth_type)| auth_type,
    );

    alt((
        map(auth_parser, |s| Capability::Auth(s)),
        map(atom, |s| Capability::Other(s)),
    ))(i)
}

// resp-text-code branches

// 'ALERT'
fn rtc_alert(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(tag_no_case("ALERT"), |_| RespTextCode::Alert)(i)
}

// 'BADCHARSET' [SP '(' astring *(SP astring) ')']
fn rtc_bad_charset(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(
        preceded(
            tag_no_case("BADCHARSET"),
            opt(delimited(
                tag(" ("),
                many1(terminated(astring, opt(tag(" ")))),
                tag(")"),
            )),
        ),
        |v| match v {
            Some(v) => RespTextCode::BadCharset(v),
            None => RespTextCode::BadCharset(vec![]),
        },
    )(i)
}

// capability-data
fn rtc_capability_data(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(capability_data, |v| RespTextCode::Capability(v))(i)
}

// 'PARSE'
fn rtc_parse(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(tag_no_case("PARSE"), |_| RespTextCode::Parse)(i)
}

// 'PERMANENTFLAGS' SP '(' [flag-perm *(SP flag-perm)] ')'
fn rtc_permanent_flags(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(
        delimited(
            tag_no_case("PERMANENTFLAGS ("),
            // TODO: Use nom::multi::separated_list0 for parsing lists
            separated_list0(tag(" "), flag_perm),
            tag(")"),
        ),
        |v| RespTextCode::PermanentFlags(v),
    )(i)
}

// 'READ-ONLY'
fn rtc_read_only(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(tag_no_case("READ-ONLY"), |_| RespTextCode::ReadOnly)(i)
}

// 'READ-WRITE'
fn rtc_read_write(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(tag_no_case("READ-WRITE"), |_| RespTextCode::ReadWrite)(i)
}

// 'TRYCREATE'
fn rtc_try_create(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(tag_no_case("TRYCREATE"), |_| RespTextCode::TryCreate)(i)
}

// 'UIDNEXT' SP nz-number
fn rtc_uidnext(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(preceded(tag_no_case("UIDNEXT "), nz_number), |result| {
        RespTextCode::UidNext(result)
    })(i)
}

// 'UIDVALIDITY' SP nz-number
fn rtc_uidvalidity(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(preceded(tag_no_case("UIDVALIDITY "), nz_number), |result| {
        RespTextCode::UidValidity(result)
    })(i)
}

// 'UNSEEN' SP nz-number
fn rtc_unseen(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    map(preceded(tag_no_case("UNSEEN "), nz_number), |result| {
        RespTextCode::Unseen(result)
    })(i)
}

// resp-text-code = "ALERT" | "BADCHARSET" [SP "(" astring *(SP astring) ")" ] |
//                  capability-data | "PARSE" | "PERMANENTFLAGS" SP "("
//                  [ flag-perm *(SP flag-perm)] ")" | "READ-ONLY" |
//                  "READ-WRITE" | "TRYCREATE" | "UIDNEXT" SP nz-number |
//                  "UIDVALIDITY" SP nz-number | "UNSEEN" SP nz_number |
//                  atom [ SP 1*<any TEXT-CHAR except "]"> ]
fn resp_text_code(i: &[u8]) -> IResult<&[u8], RespTextCode<'_>> {
    alt((
        rtc_alert,
        rtc_bad_charset,
        rtc_capability_data,
        rtc_parse,
        rtc_permanent_flags,
        rtc_read_only,
        rtc_read_write,
        rtc_try_create,
        rtc_uidnext,
        rtc_uidvalidity,
        rtc_unseen,
        // TODO: add last branch from rfc3501 resp-text-code
    ))(i)
}

// resp-text = [ "[" resp-text-code "]" SP ] text
fn resp_text(i: &[u8]) -> IResult<&[u8], RespText<'_>> {
    map(
        tuple((many0(delimited(tag("["), resp_text_code, tag("] "))), text)),
        |(code, text)| RespText { code, text },
    )(i)
}

// resp-cond-auth = ("OK" | "PREAUTH") SP resp-text;
// Authentication condition
fn resp_cond_auth(i: &[u8]) -> IResult<&[u8], (&str, RespText<'_>)> {
    map(
        separated_pair(
            alt((tag_no_case("OK"), tag_no_case("PREAUTH"))),
            tag(" "),
            resp_text,
        ),
        |(status, resp_text)| {
            // SAFETY: status is "OK" either "PREAUTH" strings, so it is valid utf-8
            let status = unsafe { std::str::from_utf8_unchecked(status) };
            (status, resp_text)
        },
    )(i)
}

// resp-cond-bye = "BYE" SP resp-text
fn resp_cond_bye(i: &[u8]) -> IResult<&[u8], ByeResponse<'_>> {
    map(preceded(tag_no_case("BYE "), resp_text), |resp| {
        ByeResponse { resp }
    })(i)
}
