use std::borrow::Cow;

use super::LinkTargetAttribute;

/// Interface Description (rt=) Link Target Attribute
///
/// [RFC 6690 §7.4](https://tools.ietf.org/html/rfc6690#section-7.4)
pub struct InterfaceDescription<'a>(LinkTargetAttribute<'a>);

pub struct InvalidInterfaceDescription;

impl<'a> InterfaceDescription<'a> {
    pub fn new(value: impl Into<Cow<'a, str>>) -> Result<InterfaceDescription<'a>, InvalidInterfaceDescription> {
        LinkTargetAttribute::new(value)
            .map(InterfaceDescription)
            .map_err(|_| InvalidInterfaceDescription)
    }
}
