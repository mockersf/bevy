use alloc::borrow::Cow;
use alloc::string::String;
use core::any::type_name;
use disqualified::ShortName;
use std::fmt;

/// Zut
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugName<'a> {
    name: Cow<'a, str>,
}

impl<'a> fmt::Display for DebugName<'a> {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl<'a> DebugName<'a> {
    /// Zut
    pub fn borrowed(value: &'a str) -> Self {
        DebugName {
            name: Cow::Borrowed(value),
        }
    }

    /// Zut
    pub fn owned(value: String) -> Self {
        DebugName {
            name: Cow::Owned(value),
        }
    }

    // pub fn new(value: Cow<'a, str>) -> Self {
    //     DebugName { name: value }
    // }

    /// Zut
    pub fn as_shortname(&self) -> ShortName {
        ShortName(self.name.as_ref())
    }
}

impl DebugName<'static> {
    /// Zut
    pub fn type_name<T>() -> Self {
        DebugName {
            name: Cow::Borrowed(type_name::<T>()),
        }
    }
}

impl<'a> From<Cow<'a, str>> for DebugName<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self { name: value }
    }
}
