#![cfg(feature = "std")]

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::string::String;

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Into
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<Cow<'a, Path>> for PathBuf {
    #[inline]
    fn from(s: Cow<'a, Path>) -> Self {
        s.into_owned()
    }
}

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a Path> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a Path) -> Self {
        Cow::Borrowed(s)
    }
}

impl<'a> From<PathBuf> for Cow<'a, Path> {
    #[inline]
    fn from(s: PathBuf) -> Self {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a PathBuf> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a PathBuf) -> Self {
        Cow::Borrowed(p.as_path())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From OsStr
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a OsStr> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a OsStr) -> Self {
        Cow::Borrowed(Path::new(s))
    }
}

impl<'a> From<OsString> for Cow<'a, Path> {
    #[inline]
    fn from(s: OsString) -> Self {
        Cow::Owned(PathBuf::from(s))
    }
}

impl<'a> From<&'a OsString> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a OsString) -> Self {
        Cow::Borrowed(Path::new(s.as_os_str()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// From str
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a str> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Cow::Borrowed(Path::new(s))
    }
}

impl<'a> From<String> for Cow<'a, Path> {
    #[inline]
    fn from(s: String) -> Self {
        Cow::Owned(PathBuf::from(s))
    }
}

impl<'a> From<&'a String> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a String) -> Self {
        Cow::Borrowed(Path::new(s.as_str()))
    }
}
