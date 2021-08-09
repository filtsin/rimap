//!

use crate::tag::Tag;
use std::convert::TryFrom;

use crate::error::{create_custom_error, Error};

#[derive(Debug)]
pub(crate) struct Greeting<'a> {
    pub(crate) status: GreetingStatus<'a>,
}

#[derive(Debug)]
pub(crate) enum GreetingStatus<'a> {
    Ok(RespText<'a>),
    Preauth(RespText<'a>),
    Bye(ByeResponse<'a>),
}

#[derive(Debug)]
pub(crate) struct ByeResponse<'a> {
    pub(crate) resp: RespText<'a>,
}

#[derive(Debug)]
pub(crate) enum ImapResponse<'a> {
    Greeting(Greeting<'a>),
    Continue,
    Response {
        tag: Tag,
        untagged_data: Vec<String>,
        status: ImapResult,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ImapResult {
    Ok,
    Bad,
    No,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum DefinedFlag {
    Seen,
    Answered,
    Flagged,
    Deleted,
    Draft,
    Recent,
}

impl TryFrom<&str> for DefinedFlag {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "\\Seen" => Ok(Self::Seen),
            "\\Answred" => Ok(Self::Answered),
            "\\Flagged" => Ok(Self::Flagged),
            "\\Deleted" => Ok(Self::Deleted),
            "\\Draft" => Ok(Self::Draft),
            "\\Recent" => Ok(Self::Recent),
            _ => Err(create_custom_error(format!(
                "Can not convert {} into DefinedFlag",
                value
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Flag<'a> {
    Defined(DefinedFlag),
    Keyword(&'a str),
    Extension(&'a str),
    Perm, // \*
}

impl<'a> From<&'a str> for Flag<'a> {
    fn from(s: &'a str) -> Self {
        if let Ok(v) = DefinedFlag::try_from(s) {
            Self::Defined(v)
        } else if s.starts_with('\\') {
            if s == "\\*" {
                Self::Perm
            } else {
                Self::Extension(s)
            }
        } else {
            Self::Keyword(s)
        }
    }
}

#[derive(Debug)]
pub(crate) enum Capability<'a> {
    // TODO: Create enum for common auth types
    Auth(&'a str),
    // TODO: Create enum for common other capabilities
    Other(&'a str),
}

#[derive(Debug)]
pub(crate) enum RespTextCode<'a> {
    Alert,
    BadCharset(Vec<&'a str>),
    Capability(Vec<Capability<'a>>),
    Parse,
    PermanentFlags(Vec<Flag<'a>>),
    ReadOnly,
    ReadWrite,
    TryCreate,
    UidNext(u32),
    UidValidity(u32),
    Unseen(u32),
    // TODO: add last branch
}

#[derive(Debug)]
pub(crate) struct RespText<'a> {
    pub(crate) code: Vec<RespTextCode<'a>>,
    pub(crate) text: &'a str,
}
