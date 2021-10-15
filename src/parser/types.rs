//!

use crate::tag::Tag;
use std::convert::TryFrom;

use crate::error::{create_custom_error, Error};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ImapResponse<'a> {
    Greeting(Greeting<'a>),
    Continue(ContinueReq<'a>),
    Response(TaggedResponse<'a>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ImapResult {
    Ok,
    Bad,
    No,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Greeting<'a> {
    pub(crate) status: GreetingStatus<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum GreetingStatus<'a> {
    Ok(RespText<'a>),
    Preauth(RespText<'a>),
    Bye(ByeResponse<'a>),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ContinueReq<'a> {
    Text(RespText<'a>),
    Base64(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TaggedResponse<'a> {
    pub(crate) tag: Tag,
    pub(crate) resp: RespCond<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum UntaggedResponse<'a> {
    RespCond(RespCond<'a>),
    RespBye(ByeResponse<'a>),
    MailBox(MailBoxData<'a>),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RespCond<'a> {
    pub(crate) status: ImapResult,
    pub(crate) text: RespText<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ByeResponse<'a> {
    pub(crate) resp: RespText<'a>,
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

// TODO: Flag should be without Perm branch
// Perm branch used only for resp_text_code
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

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ListMailBox<'a> {
    pub(crate) flags: Vec<ListFlag<'a>>,
    pub(crate) delimiter: Option<&'a str>,
    pub(crate) name: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Capability<'a> {
    // TODO: Create enum for common auth types
    Auth(&'a str),
    // TODO: Create enum for common other capabilities
    Other(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RespText<'a> {
    pub(crate) code: Vec<RespTextCode<'a>>,
    pub(crate) text: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum StatusInfo {
    Messages(u32),
    Recent(u32),
    UidNext(u32),
    UidValidity(u32),
    Unseen(u32),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct StatusResponse<'a> {
    pub(crate) name: &'a str,
    pub(crate) status: Vec<StatusInfo>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MailBoxData<'a> {
    Flags(Vec<Flag<'a>>),
    List(ListMailBox<'a>),
    Lsub(ListMailBox<'a>),
    Search(Vec<u32>),
    Status(StatusResponse<'a>),
    Exists(u32),
    Recent(u32),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MsgFlag<'a> {
    Common(Flag<'a>),
    Recent,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Address<'a> {
    pub(crate) name: Option<&'a str>,
    pub(crate) adl: Option<&'a str>,
    pub(crate) mailbox: Option<&'a str>,
    pub(crate) host: Option<&'a str>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Envelope<'a> {
    pub(crate) date: Option<&'a str>,
    pub(crate) subject: Option<&'a str>,
    pub(crate) from: Option<Vec<Address<'a>>>,
    pub(crate) sender: Option<Vec<Address<'a>>>,
    pub(crate) reply_to: Option<Vec<Address<'a>>>,
    pub(crate) to: Option<Vec<Address<'a>>>,
    pub(crate) cc: Option<Vec<Address<'a>>>,
    pub(crate) bcc: Option<Vec<Address<'a>>>,
    pub(crate) in_reply_to: Option<&'a str>,
    pub(crate) message_id: Option<&'a str>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MsgAtt<'a> {
    Envelope(Envelope<'a>),
    InternalDate(DateTime),
    Rfc822(Option<&'a str>),
    Rfc822Header(Option<&'a str>),
    Rfc822Text(Option<&'a str>),
    Rfc822Size(u32),
    Flags(Vec<MsgFlag<'a>>),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MessageData<'a> {
    Expunge(u32),
    Fetch(MsgAtt<'a>),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Time {
    pub(crate) hours: u8,
    pub(crate) minutes: u8,
    pub(crate) seconds: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct DateTime {
    pub(crate) day: u8,
    pub(crate) month: Month,
    pub(crate) year: u16,
    pub(crate) time: Time,
    pub(crate) zone: i16,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Body<'a> {
    Basic(BodyTypeBasic<'a>),
    Msg(BodyTypeMsg<'a>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum MediaType<'a> {
    Application,
    Audio,
    Image,
    Message,
    Video,
    Custom(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct MediaBasic<'a> {
    pub(crate) media_type: MediaType<'a>,
    pub(crate) subtype: &'a str,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum BodyEnc<'a> {
    N7bit,
    N8bit,
    Binary,
    Base64,
    QuotedPrintable,
    Custom(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BodyFields<'a> {
    pub(crate) param: Option<Vec<(&'a str, &'a str)>>,
    pub(crate) id: Option<&'a str>,
    pub(crate) desc: Option<&'a str>,
    pub(crate) enc: BodyEnc<'a>,
    pub(crate) octets: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BodyTypeBasic<'a> {
    pub(crate) media: MediaBasic<'a>,
    pub(crate) fields: BodyFields<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BodyTypeMsg<'a> {
    pub(crate) fields: BodyFields<'a>,
    pub(crate) envelope: Envelope<'a>,
    pub(crate) body: Box<Body<'a>>,
    pub(crate) lines: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BodyTypeText<'a> {
    pub(crate) subtype: &'a str,
    pub(crate) fields: BodyFields<'a>,
    pub(crate) lines: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BodyExt1Part<'a> {
    md5: Option<&'a str>,
}
