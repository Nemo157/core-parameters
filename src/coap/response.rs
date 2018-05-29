use std::fmt;

use super::Code;

/// CoAP Response Code
///
/// [RFC 7252 §12.1.2](https://tools.ietf.org/html/rfc7252#section-12.1.2)
/// [IANA CoAP Response Codes Registry](https://www.iana.org/assignments/core-parameters/core-parameters.xhtml#response-codes)
#[derive(Debug, Copy, Clone)]
pub struct ResponseCode(Code);

pub enum InvalidResponseCode {
    OutOfRange,
}

impl ResponseCode {
    pub fn new(value: u8) -> Result<ResponseCode, InvalidResponseCode> {
        let code = Code::new(value);

        // response codes cover all of classes 2 to 5
        if code.class() < 2 || code.class() > 5 {
            Err(InvalidResponseCode::OutOfRange)?;
        }

        Ok(ResponseCode(code))
    }

    pub fn class(self) -> u8 {
        self.0.class()
    }

    pub fn detail(self) -> u8 {
        self.0.detail()
    }

    pub fn into_code(self) -> Code {
        self.0
    }
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02}", self.class(), self.detail())
    }
}
