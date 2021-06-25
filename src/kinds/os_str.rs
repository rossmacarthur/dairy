#![cfg(feature = "std")]

use std::ffi::{OsStr, OsString};
use std::string::String;

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Into
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<Cow<'a, OsStr>> for OsString {
    #[inline]
    fn from(s: Cow<'a, OsStr>) -> Self {
        s.into_owned()
    }
}

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a OsStr> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a OsStr) -> Self {
        Cow::Borrowed(s)
    }
}

impl<'a> From<OsString> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: OsString) -> Self {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a OsString> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a OsString) -> Self {
        Cow::Borrowed(s.as_os_str())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From str
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a str> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Cow::Borrowed(OsStr::new(s))
    }
}

impl<'a> From<String> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: String) -> Self {
        Cow::Owned(OsString::from(s))
    }
}

impl<'a> From<&'a String> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a String) -> Self {
        Cow::Borrowed(OsStr::new(s.as_str()))
    }
}
