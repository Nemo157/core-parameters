use std::borrow::Cow;

use uri::Uri;

/// Link Target Attribute
///
/// [RFC 6690 §7.4](https://tools.ietf.org/html/rfc6690#section-7.4)
pub(crate) struct LinkTargetAttribute<'a>(Cow<'a, str>);

pub(crate) struct InvalidLinkTargetAttribute;

impl<'a> LinkTargetAttribute<'a> {
    fn new_ext_rel_type(value: Cow<'a, str>) -> Result<Self, Cow<'a, str>> {
        if Uri::new(&value).is_err() {
            return Err(value);
        }

        Ok(LinkTargetAttribute(value))
    }

    fn new_reg_rel_type(value: Cow<'a, str>) -> Result<Self, Cow<'a, str>> {
        if value.is_empty() {
            return Err(value);
        }

        if !value.is_ascii() {
            return Err(value);
        }

        let ok = 'ok: loop {
            let bytes = value.as_bytes();

            if !bytes[0].is_ascii_lowercase() {
                break false;
            }

            for &byte in bytes {
                if !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'.' || byte == b'-') {
                    break 'ok false;
                }
            }

            break true;
        };

        if !ok {
            return Err(value);
        }

        Ok(LinkTargetAttribute(value))
    }

    pub fn new(value: impl Into<Cow<'a, str>>) -> Result<Self, InvalidLinkTargetAttribute> {
        let value = value.into();

        // Must be a `relation-type` from RFC 6690 §2
        //
        // relation-type  = reg-rel-type / ext-rel-type
        // reg-rel-type   = LOALPHA *( LOALPHA / DIGIT / "." / "-" )
        // ext-rel-type   = URI
        // LOALPHA        = %x61-7A   ; a-z

        Self::new_ext_rel_type(value)
            .or_else(Self::new_reg_rel_type)
            .map_err(|_| InvalidLinkTargetAttribute)
    }
}
