use super::StatusCode;
use std::io::{Result as IoResult, Write};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        //impl Write per accettare come param qualsiasi che abbiamo come trait Write
        let body = match &self.body {
            //uso &self perché non posso dare a bofy l'ownership di self
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(), // in questo caso non uso & perché sono in sola lettura dei valori
            body
        )
    }
}
