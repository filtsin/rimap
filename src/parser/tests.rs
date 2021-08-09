use super::parse;
use super::response::{Greeting, GreetingStatus, ImapResponse};

#[test]
fn parse_greeting() {
    let result = parse(b"* OK IMAP4rev1 Service Ready\r\n").unwrap().1;

    match result {
        ImapResponse::Greeting(greeting) => match greeting.status {
            GreetingStatus::Ok(resp) => {
                assert_eq!(resp.code.len(), 0);
                assert_eq!(resp.text, "IMAP4rev1 Service Ready");
            }
            _ => panic!("Wrong answer {:?}", greeting),
        },
        _ => panic!("Wrong answer {:?}", result),
    }
}
