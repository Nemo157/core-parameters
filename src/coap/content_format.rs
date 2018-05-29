use std::fmt;

/// CoAP Content-Format
///
/// [RFC 7252 §12.2](https://tools.ietf.org/html/rfc7252#section-12.2)
/// [IANA CoAP Content-Formats Registry](https://www.iana.org/assignments/core-parameters/core-parameters.xhtml#content-formats)
#[derive(Debug, Copy, Clone)]
pub struct ContentFormat(u16);

impl ContentFormat {
    pub fn new(value: u16) -> ContentFormat {
        ContentFormat(value)
    }
}

impl fmt::Display for ContentFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
