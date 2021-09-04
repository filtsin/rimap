//!

use crate::tag::Tag;
use std::convert::TryFrom;

use crate::error::{create_custom_error, Error};

#[derive(Debug, PartialEq)]
pub(crate) struct Greeting<'a> {
    pub(crate) status: GreetingStatus<'a>,
}

#[derive(Debug)]
pub(crate) enum ContinueReq<'a> {
    Text(RespText<'a>),
    Base64(&'a str),
}

#[derive(Debug)]
pub(crate) struct TaggedResponse<'a> {
    pub(crate) tag: Tag,
    pub(crate) resp: RespCond<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum GreetingStatus<'a> {
    Ok(RespText<'a>),
    Preauth(RespText<'a>),
    Bye(ByeResponse<'a>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum UntaggedResponse<'a> {
    RespCond(RespCond<'a>),
    RespBye(ByeResponse<'a>),
}

#[derive(Debug, PartialEq)]
pub(crate) struct RespCond<'a> {
    pub(crate) status: ImapResult,
    pub(crate) text: RespText<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ByeResponse<'a> {
    pub(crate) resp: RespText<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ImapResponse<'a> {
    Greeting(Greeting<'a>),
    Continue,
    UntaggedResponse(UntaggedResponse<'a>),
    Response {
        tag: Tag,
        untagged_data: Vec<String>,
        status: ImapResult,
    },
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

// TODO: It is incorrect, because flags are case-insesitive
// Add function for insesitive cmp
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

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ListDefinedFlag {
    Noinferiors,
    Noselect,
    Marked,
    Unmarked,
}

// TODO: It is incorrect, because flags are case-insesitive
// Add function for insesitive cmp
impl TryFrom<&str> for ListDefinedFlag {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "\\Noinferiors" => Ok(Self::Noinferiors),
            "\\Noselect" => Ok(Self::Noselect),
            "\\Marked" => Ok(Self::Marked),
            "\\Unmarked" => Ok(Self::Unmarked),
            _ => Err(create_custom_error(format!(
                "Can not convert {} into ListDefinedFlag",
                value
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ListFlag<'a> {
    Defined(ListDefinedFlag),
    Extension(&'a str),
}

impl<'a> From<&'a str> for ListFlag<'a> {
    fn from(s: &'a str) -> Self {
        if let Ok(v) = ListDefinedFlag::try_from(s) {
            Self::Defined(v)
        } else {
            Self::Extension(s)
        }
    }
}

#[derive(Debug)]
pub(crate) struct ListMailBox<'a> {
    pub(crate) flags: Vec<ListFlag<'a>>,
    pub(crate) delimiter: &'a str,
    pub(crate) name: &'a str,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Capability<'a> {
    // TODO: Create enum for common auth types
    Auth(&'a str),
    // TODO: Create enum for common other capabilities
    Other(&'a str),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub(crate) struct RespText<'a> {
    pub(crate) code: Vec<RespTextCode<'a>>,
    pub(crate) text: &'a str,
}

#[derive(Debug)]
pub(crate) enum MailBoxData<'a> {
    Flags(Vec<Flag<'a>>),
    List(ListMailBox<'a>),
}
