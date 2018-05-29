use std::fmt;

/// CoAP Option
///
/// [RFC 7252 §12.2](https://tools.ietf.org/html/rfc7252#section-12.2)
/// [IANA CoAP Options Registry](https://www.iana.org/assignments/core-parameters/core-parameters.xhtml#option-numbers)
#[derive(Debug, Copy, Clone)]
pub struct Option(u16);

impl Option {
    pub fn new(value: u16) -> Option {
        Option(value)
    }
}

impl fmt::Display for Option {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
