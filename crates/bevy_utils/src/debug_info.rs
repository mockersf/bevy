use alloc::borrow::Cow;
use alloc::string::String;
use core::any::type_name;
use disqualified::ShortName;
use std::fmt;

/// Zut
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugName {
    name: Cow<'static, str>,
}

impl fmt::Display for DebugName {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl DebugName {
    /// Zut
    pub fn borrowed(value: &'static str) -> Self {
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

    /// Zut
    pub fn type_name<T>() -> Self {
        DebugName {
            name: Cow::Borrowed(type_name::<T>()),
        }
    }
}

impl From<Cow<'static, str>> for DebugName {
    fn from(value: Cow<'static, str>) -> Self {
        Self { name: value }
    }
}
