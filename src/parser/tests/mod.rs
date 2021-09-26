use crate::parser::{
    parse,
    types::{Greeting, GreetingStatus, ImapResponse, RespText},
};

fn resp_text(s: &str) -> RespText<'_> {
    RespText {
        code: vec![],
        text: s,
    }
}

#[test]
fn parse_greeting() {
    let response = b"* OK IMAP4rev1 Service Ready\r\n";

    let result = parse(response).unwrap();

    let greeting = ImapResponse::Greeting(Greeting {
        status: GreetingStatus::Ok(resp_text("IMAP4rev1 Service Ready")),
    });

    assert_eq!(result.0.len(), 0);
    assert_eq!(result.1, greeting);
}
