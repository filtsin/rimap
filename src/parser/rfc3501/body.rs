//! Body IMAP grammar

use super::{core::*, grammar::envelope};

use crate::parser::types::{
    Body, BodyEnc, BodyFields, BodyTypeBasic, BodyTypeMsg, BodyTypeText, MediaBasic, MediaType,
};

use nom::{
    branch::alt,
    bytes::streaming::{tag, tag_no_case},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

// body = '(' (body-type-1part | body-type-mpart) ')'
pub(crate) fn body(i: &[u8]) -> IResult<&[u8], Body> {
    todo!()
}

// body-type-1part = (body-type-basic | body-type-msg | body-type-text) [SP body-ext-1part]
pub(crate) fn body_type_1part(i: &[u8]) -> IResult<&[u8], Body<'_>> {
    todo!()
}

// body_type_basic = media-basic SP body-fields
pub(crate) fn body_type_basic(i: &[u8]) -> IResult<&[u8], BodyTypeBasic<'_>> {
    map(
        separated_pair(media_basic, tag(" "), body_fields),
        |(media, fields)| BodyTypeBasic { media, fields },
    )(i)
}

// media-basic = ((DQUOTE ('APPLICATION' | 'AUDIO' | 'IMAGE' | 'MESSAGE' | 'VIDEO')
//                 DQUOTE) | string) SP media-subtype
// media-subtype = string
pub(crate) fn media_basic(i: &[u8]) -> IResult<&[u8], MediaBasic<'_>> {
    map(
        separated_pair(
            alt((
                delimited(
                    tag("\""),
                    alt((
                        value(MediaType::Application, tag_no_case("APPLICATION")),
                        value(MediaType::Audio, tag_no_case("AUDIO")),
                        value(MediaType::Image, tag_no_case("IMAGE")),
                        value(MediaType::Message, tag_no_case("MESSAGE")),
                        value(MediaType::Video, tag_no_case("VIDEO")),
                    )),
                    tag("\""),
                ),
                map(string, MediaType::Custom),
            )),
            tag(" "),
            string,
        ),
        |(media_type, subtype)| MediaBasic {
            media_type,
            subtype,
        },
    )(i)
}

// body-fields = body-fld-param SP body-fld-id SP body-fld-desc SP
//               body-fld-enc SP body-fld-octets
// body-fld-id, body-fld-desc = nstring
// body-fld-octets = number
pub(crate) fn body_fields(i: &[u8]) -> IResult<&[u8], BodyFields<'_>> {
    map(
        tuple((
            body_fld_param,
            tag(" "),
            nstring,
            tag(" "),
            nstring,
            tag(" "),
            body_fld_enc,
            tag(" "),
            number,
        )),
        |(param, _, id, _, desc, _, enc, _, octets)| BodyFields {
            param,
            id,
            desc,
            enc,
            octets,
        },
    )(i)
}

// body-fld-param = '(' string SP string *(SP string SP string) ')' | nil
pub(crate) fn body_fld_param(i: &[u8]) -> IResult<&[u8], Option<Vec<(&str, &str)>>> {
    alt((
        map(
            delimited(
                tag("("),
                separated_list1(tag(" "), separated_pair(string, tag(" "), string)),
                tag(")"),
            ),
            Some,
        ),
        nil,
    ))(i)
}

// body-fld-enc = (DQUOTE ('7BIT' | '8BIT' | 'BINARY' | 'BASE64' | 'QUOTED-PRINTABLE')
//                 DQUOTE) | string
pub(crate) fn body_fld_enc(i: &[u8]) -> IResult<&[u8], BodyEnc<'_>> {
    alt((
        delimited(
            tag("\""),
            alt((
                value(BodyEnc::N7bit, tag_no_case("7BIT")),
                value(BodyEnc::N8bit, tag_no_case("8BIT")),
                value(BodyEnc::Binary, tag_no_case("BINARY")),
                value(BodyEnc::Base64, tag_no_case("BASE64")),
                value(BodyEnc::QuotedPrintable, tag_no_case("QUOTED-PRINTABLE")),
            )),
            tag("\""),
        ),
        map(string, BodyEnc::Custom),
    ))(i)
}

// body-type-msg = media-message SP body-fields SP envelope SP body SP body-fld-lines
// media-message = DQUOTE 'MESSAGE' DQUOTE SP DQUOTE 'RFC822' DQUOTE
// body-fld-lines = number
pub(crate) fn body_type_msg(i: &[u8]) -> IResult<&[u8], BodyTypeMsg<'_>> {
    map(
        tuple((
            tag_no_case("\"MESSAGE\" \"RFC822\" "),
            body_fields,
            tag(" "),
            envelope,
            tag(" "),
            body,
            tag(" "),
            number,
        )),
        |(_, fields, _, envelope, _, body, _, lines)| BodyTypeMsg {
            fields,
            envelope,
            body: Box::new(body),
            lines,
        },
    )(i)
}

// body-type-text = media-text SP body-fields SP body-fld-lines
// media-text = DQUOTE 'TEXT' DQUOTE SP media-subtype
// media-subtype = string
// body-fld-lines = number
pub(crate) fn body_type_text(i: &[u8]) -> IResult<&[u8], BodyTypeText<'_>> {
    map(
        tuple((
            tag_no_case("\"TEXT\" "),
            string,
            tag(" "),
            body_fields,
            tag(" "),
            number,
        )),
        |(_, subtype, _, fields, _, lines)| BodyTypeText {
            subtype,
            fields,
            lines,
        },
    )(i)
}

// body-ext-1part = body-fld-md5 [SP body-fld-dsp [SP body-fld-lang [SP body-fld-loc *(SP
//                  body-extension)]]]
// ; MUST NOT be returned on non-extensible "BODY" fetch
// body-fld-md5, body_fld_loc = nstring
pub(crate) fn body_ext_1part(i: &[u8]) -> IResult<&[u8], ()> {
    todo!()
}

// body-fld-dsp = '(' string SP body-fld-param ')' | nil

// body-fld-lang = nstring | '(' string *(SP string) ')'

// body-extension = nstring | number | '(' body-extension *(SP body-extension) ')'
// ; Future expansion. Client implemenations MUST accept body-extension fields.
// Server implemenations MUST NOT generate body-extension fields except
// as defined by future standart or standards-track revisions of rfc3501
