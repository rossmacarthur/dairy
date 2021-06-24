use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a Path> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a Path) -> Cow<'a, Path> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<PathBuf> for Cow<'a, Path> {
    #[inline]
    fn from(s: PathBuf) -> Cow<'a, Path> {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a PathBuf> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a PathBuf) -> Cow<'a, Path> {
        Cow::Borrowed(p.as_path())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From OsStr
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a OsStr> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a OsStr) -> Cow<'a, Path> {
        Cow::Borrowed(Path::new(s))
    }
}

impl<'a> From<OsString> for Cow<'a, Path> {
    #[inline]
    fn from(s: OsString) -> Cow<'a, Path> {
        Cow::Owned(PathBuf::from(s))
    }
}

impl<'a> From<&'a OsString> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a OsString) -> Cow<'a, Path> {
        Cow::Borrowed(Path::new(s.as_os_str()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// From str
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a str> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a str) -> Cow<'a, Path> {
        Cow::Borrowed(Path::new(s))
    }
}

impl<'a> From<String> for Cow<'a, Path> {
    #[inline]
    fn from(s: String) -> Cow<'a, Path> {
        Cow::Owned(PathBuf::from(s))
    }
}

impl<'a> From<&'a String> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a String) -> Cow<'a, Path> {
        Cow::Borrowed(Path::new(s.as_str()))
    }
}
