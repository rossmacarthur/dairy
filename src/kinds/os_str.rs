#![cfg(feature = "std")]

use std::ffi::{OsStr, OsString};
use std::string::String;

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Into
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<Cow<'a, OsStr>> for OsString {
    #[inline]
    fn from(s: Cow<'a, OsStr>) -> OsString {
        s.into_owned()
    }
}

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a OsStr> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a OsStr) -> Cow<'a, OsStr> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<OsString> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: OsString) -> Cow<'a, OsStr> {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a OsString> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a OsString) -> Cow<'a, OsStr> {
        Cow::Borrowed(s.as_os_str())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From str
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a str> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a str) -> Cow<'a, OsStr> {
        Cow::Borrowed(OsStr::new(s))
    }
}

impl<'a> From<String> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: String) -> Cow<'a, OsStr> {
        Cow::Owned(OsString::from(s))
    }
}

impl<'a> From<&'a String> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a String) -> Cow<'a, OsStr> {
        Cow::Borrowed(OsStr::new(s.as_str()))
    }
}
