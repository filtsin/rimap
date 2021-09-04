use crate::parser::{
    parse,
    response::{
        ByeResponse, Greeting, GreetingStatus, ImapResult, RespCond, RespText, RespTextCode,
        UntaggedResponse,
    },
    ImapResponse,
};

#[test]
fn parse_greeting() {
    let greeting_repsonse = b"* OK IMAP4rev1 Service Ready\r\n";
    let resp_text = RespText {
        code: Vec::new(),
        text: "IMAP4rev1 Service Ready",
    };
    let greeting_status = GreetingStatus::Ok(resp_text);
    let expected = ImapResponse::Greeting(Greeting {
        status: greeting_status,
    });

    let result = parse(greeting_repsonse).unwrap().1;

    assert_eq!(result, expected);
}

#[test]
fn parse_preauth() {
    let preauth_repsonse = b"* PREAUTH IMAP4rev1 server logged in as Smith\r\n";
    let resp_text = RespText {
        code: Vec::new(),
        text: "IMAP4rev1 server logged in as Smith",
    };
    let greeting_status = GreetingStatus::Preauth(resp_text);
    let expected = ImapResponse::Greeting(Greeting {
        status: greeting_status,
    });

    let result = parse(preauth_repsonse).unwrap().1;

    assert_eq!(result, expected);
}

#[test]
fn parse_bye() {
    let bye_repsonse = b"* BYE IDLE for too long\r\n";
    let bye_resp = ByeResponse {
        resp: RespText {
            code: Vec::new(),
            text: "IDLE for too long",
        },
    };
    let greeting_status = GreetingStatus::Bye(bye_resp);
    let expected = ImapResponse::Greeting(Greeting {
        status: greeting_status,
    });

    let result = parse(bye_repsonse).unwrap().1;

    assert_eq!(result, expected);
}

#[test]
fn parse_ok() {
    let ok_repsonse = b"* OK [ALERT] System shutdown in 10 minutes\r\n";
    let resp_text = RespText {
        code: vec![RespTextCode::Alert],
        text: "System shutdown in 10 minutes",
    };
    let resp_cond = RespCond {
        status: ImapResult::Ok,
        text: resp_text,
    };
    let expected = ImapResponse::UntaggedResponse(UntaggedResponse::RespCond(resp_cond));

    let result = parse(ok_repsonse).unwrap().1;

    assert_eq!(result, expected);
}

#[test]
fn parse_no() {
    let no_repsonse = b"* NO COPY failed: disk is full\r\n";
    let resp_text = RespText {
        code: vec![],
        text: "NO COPY failed: disk is full",
    };
    let resp_cond = RespCond {
        status: ImapResult::No,
        text: resp_text,
    };
    let expected = ImapResponse::UntaggedResponse(UntaggedResponse::RespCond(resp_cond));

    let result = parse(no_repsonse).unwrap().1;

    assert_eq!(result, expected);
}
