//! IMAP core types

use nom::{
    branch::alt,
    bytes::streaming::{tag, take_while, take_while1, take_while_m_n},
    character::is_alphanumeric,
    character::streaming::{crlf, u32},
    combinator::{map_res, not, opt, peek},
    multi::{length_data, many1_count},
    sequence::{delimited, tuple},
    IResult,
};

// strings

// CORE rules from rfc 5234

// CHAR = %x01-7F;
// any 7-bit US-ASCII character excluding 0
fn is_char(i: u8) -> bool {
    (0x01..=0x7f).contains(&i)
}

// CTL = %x00-1F;
// controls
fn is_ctl(i: u8) -> bool {
    (0x00..=0x1f).contains(&i) || i == 0x7f
}

// SP = %x20;
// space
fn is_space(i: u8) -> bool {
    i == b' '
}

// rfc 3501

// CHAR8 = %x01-ff;
// any octet except 0
fn is_char8(i: u8) -> bool {
    i != 0x00
}

// TEXT-CHAR = <any CHAR except CR and LF>
fn is_text_char(i: u8) -> bool {
    is_char(i) && i != b'\r' && i != b'\n'
}

// quoted-specials = '"' | '\'
fn is_quoted_specials(i: u8) -> bool {
    i == b'"' || i == b'\\'
}

// TODO: incorrect 2nd or: '\' quoted-specials
// QUOTED-CHAR = <any TEXT-CHAR except quoted-specials> | quoted-specials
pub(crate) fn is_quoted_char(i: u8) -> bool {
    is_quoted_specials(i) || is_text_char(i)
}

// list-wildcards = '%' | '*'
fn is_list_wildcards(i: u8) -> bool {
    i == b'%' || i == b'*'
}

// resp-specials = ']'
fn is_resp_specials(i: u8) -> bool {
    i == b']'
}

// base64-char = ALPHA | DIGIT | '+' | '/'
fn is_base64_char(i: u8) -> bool {
    is_alphanumeric(i) || i == b'+' || i == b'/'
}

// atom-specials = '(' | ')' | '{' | SP | CTL | list-wildcards | quoted-specials | resp-specials
fn is_atom_specials(i: u8) -> bool {
    i == b')'
        || i == b')'
        || i == b'{'
        || is_space(i)
        || is_ctl(i)
        || is_list_wildcards(i)
        || is_quoted_specials(i)
        || is_resp_specials(i)
}

// ATOM-CHAR = <any CHAR except atom-specials>
fn is_atom_char(i: u8) -> bool {
    !is_atom_specials(i) && is_char(i)
}

// ASTRING-CHAR = ATOM-CHAR | resp-specials
pub(crate) fn is_astring_char(i: u8) -> bool {
    is_atom_char(i) || is_resp_specials(i)
}

// astring = 1*ASTRING-CHAR | string
pub(crate) fn astring(i: &[u8]) -> IResult<&[u8], &str> {
    alt((
        string,
        map_res(take_while1(is_astring_char), std::str::from_utf8),
    ))(i)
}

// atom = 1*ATOM-CHAR
pub(crate) fn atom(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(take_while1(is_atom_char), std::str::from_utf8)(i)
}

// literal = "{" number "}" CRLF *CHAR8;
// number represents the number of CHAR8s
pub(crate) fn literal(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, (_, count, _, _)) = tuple((tag("{"), number, tag("}"), crlf))(i)?;
    let parser = take_while_m_n(count as usize, count as usize, is_char8);

    map_res(parser, std::str::from_utf8)(i)
}

// text = 1*TEXT-CHAR
//
pub(crate) fn text(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(take_while1(is_text_char), std::str::from_utf8)(i)
}

// quoted = DQUOTE *QUOTED-CHAR DQUOTE;
// quoted text
pub(crate) fn quoted(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        delimited(tag("\""), take_while(is_quoted_char), tag("\"")),
        std::str::from_utf8,
    )(i)
}

// string = quoted | literal
//
pub(crate) fn string(i: &[u8]) -> IResult<&[u8], &str> {
    alt((quoted, literal))(i)
}

// base64-terminal = (2base64-char '==') | (3base64-char '=')
pub(crate) fn base64_terminal(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        length_data(many1_count(peek(alt((
            tuple((take_while_m_n(2, 2, is_base64_char), tag("=="))),
            tuple((take_while_m_n(3, 3, is_base64_char), tag("="))),
        ))))),
        std::str::from_utf8,
    )(i)
}

// base64 = *(4base64_char) [base64_terminal]
pub(crate) fn base64(i: &[u8]) -> IResult<&[u8], &str> {
    // ODO: Check it
    map_res(
        length_data(many1_count(peek(tuple((
            take_while_m_n(4, 4, is_base64_char),
            opt(base64_terminal),
        ))))),
        std::str::from_utf8,
    )(i)
}

// numbers

// number = 1*DIGIT;
// unsigned 32-bit integer
pub(crate) fn number(i: &[u8]) -> IResult<&[u8], u32> {
    u32(i)
}

// nz-number = digit-nz *DIGIT;
// non-zero unsigned 32-bit integer
pub(crate) fn nz_number(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, (_, result)) = tuple((not(tag("0")), number))(i)?;
    Ok((i, result))
}
