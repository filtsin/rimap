use crate::{
    parser::{
        parse,
        types::{
            ContinueReq, Greeting, GreetingStatus, ImapResponse, ImapResult, RespCond, RespText,
            TaggedResponse,
        },
    },
    tag::Tag,
};

fn resp_text(s: &str) -> RespText<'_> {
    RespText {
        code: vec![],
        text: s,
    }
}

fn assert_eq((remainder, result): (&[u8], ImapResponse<'_>), target: ImapResponse<'_>) {
    assert_eq!(remainder.len(), 0);
    assert_eq!(result, target);
}

#[test]
fn parse_greeting() {
    let response = b"* OK IMAP4rev1 Service Ready\r\n";

    let result = parse(response).unwrap();

    let greeting = ImapResponse::Greeting(Greeting {
        status: GreetingStatus::Ok(resp_text("IMAP4rev1 Service Ready")),
    });

    assert_eq(result, greeting);
}

#[test]
fn parse_tagged_answer() {
    let response = b"a0017 OK CAPABILITY completed\r\n";

    let result = parse(response).unwrap();

    let tagged_response = ImapResponse::Response(TaggedResponse {
        tag: Tag::new('a', 17),
        resp: RespCond {
            status: ImapResult::Ok,
            text: resp_text("CAPABILITY completed"),
        },
    });

    assert_eq(result, tagged_response);
}

#[test]
fn parse_continue_req() {
    let response = b"+ Ready\r\n";

    let result = parse(response).unwrap();

    let continue_req = ImapResponse::Continue(ContinueReq::Text(resp_text("Ready")));

    assert_eq(result, continue_req);
}
