use std::borrow::Cow;

use super::LinkTargetAttribute;

/// Resource Type (rt=) Link Target Attribute
///
/// [RFC 6690 §7.4](https://tools.ietf.org/html/rfc6690#section-7.4)
pub struct ResourceType<'a>(LinkTargetAttribute<'a>);

pub struct InvalidResourceType;

impl<'a> ResourceType<'a> {
    pub fn new(value: impl Into<Cow<'a, str>>) -> Result<ResourceType<'a>, InvalidResourceType> {
        LinkTargetAttribute::new(value)
            .map(ResourceType)
            .map_err(|_| InvalidResourceType)
    }
}
