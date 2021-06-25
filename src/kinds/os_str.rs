#![cfg(feature = "std")]

use std::boxed::Box;
use std::ffi::{OsStr, OsString};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
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

impl<'a> From<Box<OsStr>> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: Box<OsStr>) -> Self {
        Cow::Owned(s.into_os_string())
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

////////////////////////////////////////////////////////////////////////////////
// From Path
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a Path> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a Path) -> Self {
        Cow::Borrowed(s.as_os_str())
    }
}

impl<'a> From<PathBuf> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: PathBuf) -> Self {
        Cow::Owned(OsString::from(s))
    }
}

impl<'a> From<&'a PathBuf> for Cow<'a, OsStr> {
    #[inline]
    fn from(p: &'a PathBuf) -> Self {
        Cow::Borrowed(p.as_os_str())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From iterator
////////////////////////////////////////////////////////////////////////////////

impl<'a> FromIterator<&'a OsStr> for Cow<'a, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a OsStr>>(it: I) -> Self {
        Cow::Owned(OsString::from_iter(it))
    }
}

impl<'a> FromIterator<OsString> for Cow<'a, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = OsString>>(it: I) -> Self {
        Cow::Owned(OsString::from_iter(it))
    }
}

impl<'a> FromIterator<Cow<'a, OsStr>> for Cow<'a, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Cow<'a, OsStr>>>(it: I) -> Self {
        Cow::Owned(OsString::from_iter(it.into_iter().map(Cow::into_owned)))
    }
}
