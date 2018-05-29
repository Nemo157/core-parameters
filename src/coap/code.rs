use std::fmt;

use super::{MethodCode, ResponseCode};

/// CoAP Code
///
/// [RFC 7252 §12.1](https://tools.ietf.org/html/rfc7252#section-12.1)
/// [IANA CoAP Codes Registry](https://www.iana.org/assignments/core-parameters/core-parameters.xhtml#codes)
#[derive(Debug, Copy, Clone)]
pub struct Code(u8);

impl Code {
    pub fn new(value: u8) -> Code {
        Code(value)
    }

    pub fn class(self) -> u8 {
        self.0 >> 5
    }

    pub fn detail(self) -> u8 {
        self.0 & 0b00011111
    }

    pub fn into_method(self) -> Option<MethodCode> {
        MethodCode::new(self.0).ok()
    }

    pub fn into_response(self) -> Option<ResponseCode> {
        ResponseCode::new(self.0).ok()
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02}", self.class(), self.detail())
    }
}
