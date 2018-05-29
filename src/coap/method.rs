use std::fmt;

use super::Code;

/// CoAP Method Code
///
/// [RFC 7252 §12.1.1](https://tools.ietf.org/html/rfc7252#section-12.1.1)
/// [IANA CoAP Method Codes Registry](https://www.iana.org/assignments/core-parameters/core-parameters.xhtml#method-codes)
#[derive(Debug, Copy, Clone)]
pub struct MethodCode(Code);

pub enum InvalidMethodCode {
    OutOfRange,
}

impl MethodCode {
    pub fn new(value: u8) -> Result<MethodCode, InvalidMethodCode> {
        let code = Code::new(value);

        // method codes are all class 0
        if code.class() != 0 {
            Err(InvalidMethodCode::OutOfRange)?;
        }

        // except 0.00 is not a method code
        if code.detail() == 0 {
            Err(InvalidMethodCode::OutOfRange)?;
        }

        Ok(MethodCode(code))
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

impl fmt::Display for MethodCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02}", self.class(), self.detail())
    }
}
