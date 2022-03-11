use alloc::borrow::Cow as StdCow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::{Cow, Dairy};

impl<'a, T> From<StdCow<'a, T>> for Cow<'a, T>
where
    T: Dairy,
{
    #[inline]
    fn from(c: StdCow<'a, T>) -> Self {
        match c {
            StdCow::Borrowed(b) => Self::borrowed(b),
            StdCow::Owned(o) => Self::owned(o),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<str>
////////////////////////////////////////////////////////////////////////////////

impl From<Cow<'_, str>> for String {
    #[inline]
    fn from(s: Cow<'_, str>) -> Self {
        s.into_owned()
    }
}

impl From<Cow<'_, str>> for Box<str> {
    #[inline]
    fn from(s: Cow<'_, str>) -> Self {
        s.into_boxed()
    }
}

impl<'a> From<&'a str> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Self::borrowed(s)
    }
}

impl From<String> for Cow<'_, str> {
    #[inline]
    fn from(s: String) -> Self {
        Self::owned(s)
    }
}

impl<'a> From<&'a String> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a String) -> Self {
        Cow::borrowed(s.as_str())
    }
}

impl From<Box<str>> for Cow<'_, str> {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Cow::owned(s.into_string())
    }
}

impl From<char> for Cow<'_, str> {
    #[inline]
    fn from(c: char) -> Self {
        Cow::owned(String::from(c))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<[T]>
////////////////////////////////////////////////////////////////////////////////

impl<'a, T: 'a + Clone> From<Cow<'a, [T]>> for Vec<T> {
    #[inline]
    fn from(v: Cow<'a, [T]>) -> Self {
        v.into_owned()
    }
}

impl<'a, T: 'a + Clone> From<Cow<'a, [T]>> for Box<[T]> {
    #[inline]
    fn from(v: Cow<'a, [T]>) -> Self {
        v.into_boxed()
    }
}

impl<'a, T: 'a + Clone> From<&'a [T]> for Cow<'a, [T]> {
    #[inline]
    fn from(v: &'a [T]) -> Self {
        Self::borrowed(v)
    }
}

impl<'a, T: 'a + Clone> From<Vec<T>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: Vec<T>) -> Self {
        Self::owned(v)
    }
}

impl<'a, T: 'a + Clone> From<&'a Vec<T>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: &'a Vec<T>) -> Self {
        Cow::borrowed(v.as_slice())
    }
}

impl<'a, T: 'a + Clone> From<Box<[T]>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: Box<[T]>) -> Self {
        Cow::owned(v.into_vec())
    }
}

impl<'a, T: 'a + Clone, const N: usize> From<[T; N]> for Cow<'a, [T]> {
    #[inline]
    fn from(v: [T; N]) -> Self {
        Cow::owned(<[T]>::into_vec(Box::new(v)))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<CStr>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl From<Cow<'_, CStr>> for CString {
    #[inline]
    fn from(s: Cow<'_, CStr>) -> Self {
        s.into_owned()
    }
}

#[cfg(feature = "std")]
impl From<Cow<'_, CStr>> for Box<CStr> {
    #[inline]
    fn from(s: Cow<'_, CStr>) -> Self {
        s.into_boxed()
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a CStr> for Cow<'a, CStr> {
    #[inline]
    fn from(s: &'a CStr) -> Self {
        Self::borrowed(s)
    }
}

#[cfg(feature = "std")]
impl From<CString> for Cow<'_, CStr> {
    #[inline]
    fn from(s: CString) -> Self {
        Self::owned(s)
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a CString> for Cow<'a, CStr> {
    #[inline]
    fn from(s: &'a CString) -> Self {
        Cow::borrowed(s.as_c_str())
    }
}

#[cfg(feature = "std")]
impl From<Box<CStr>> for Cow<'_, CStr> {
    #[inline]
    fn from(s: Box<CStr>) -> Self {
        Cow::owned(s.into_c_string())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<OsStr>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl From<Cow<'_, OsStr>> for OsString {
    #[inline]
    fn from(s: Cow<'_, OsStr>) -> Self {
        s.into_owned()
    }
}

#[cfg(feature = "std")]
impl From<Cow<'_, OsStr>> for Box<OsStr> {
    #[inline]
    fn from(s: Cow<'_, OsStr>) -> Self {
        s.into_boxed()
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a OsStr> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a OsStr) -> Self {
        Self::borrowed(s)
    }
}

#[cfg(feature = "std")]
impl From<OsString> for Cow<'_, OsStr> {
    #[inline]
    fn from(s: OsString) -> Self {
        Self::owned(s)
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a OsString> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a OsString) -> Self {
        Cow::borrowed(s.as_os_str())
    }
}

#[cfg(feature = "std")]
impl From<Box<OsStr>> for Cow<'_, OsStr> {
    #[inline]
    fn from(s: Box<OsStr>) -> Self {
        Cow::owned(s.into_os_string())
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a str> for Cow<'a, OsStr> {
    #[inline]
    fn from(p: &'a str) -> Self {
        Cow::borrowed(OsStr::new(p))
    }
}

#[cfg(feature = "std")]
impl From<String> for Cow<'_, OsStr> {
    #[inline]
    fn from(s: String) -> Self {
        Cow::owned(OsString::from(s))
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a String> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a String) -> Self {
        Cow::borrowed(OsStr::new(s.as_str()))
    }
}

#[cfg(feature = "std")]
impl From<Box<str>> for Cow<'_, OsStr> {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Cow::owned(OsString::from(String::from(s)))
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a Path> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a Path) -> Self {
        Cow::borrowed(OsStr::new(s))
    }
}

#[cfg(feature = "std")]
impl<'a> From<PathBuf> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: PathBuf) -> Self {
        Cow::owned(OsString::from(s))
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a PathBuf> for Cow<'a, OsStr> {
    #[inline]
    fn from(s: &'a PathBuf) -> Self {
        Cow::borrowed(OsStr::new(s.as_os_str()))
    }
}

#[cfg(feature = "std")]
impl From<Box<Path>> for Cow<'_, OsStr> {
    #[inline]
    fn from(s: Box<Path>) -> Self {
        Cow::owned(OsString::from(s.into_path_buf()))
    }
}

#[cfg(feature = "std")]
impl From<char> for Cow<'_, OsStr> {
    #[inline]
    fn from(c: char) -> Self {
        Cow::owned(OsString::from(String::from(c)))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<Path>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl From<Cow<'_, Path>> for PathBuf {
    #[inline]
    fn from(p: Cow<'_, Path>) -> Self {
        p.into_owned()
    }
}

#[cfg(feature = "std")]
impl From<Cow<'_, Path>> for Box<Path> {
    #[inline]
    fn from(p: Cow<'_, Path>) -> Self {
        p.into_boxed()
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a Path> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a Path) -> Self {
        Self::borrowed(p)
    }
}

#[cfg(feature = "std")]
impl From<PathBuf> for Cow<'_, Path> {
    #[inline]
    fn from(p: PathBuf) -> Self {
        Self::owned(p)
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a PathBuf> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a PathBuf) -> Self {
        Cow::borrowed(p.as_path())
    }
}

#[cfg(feature = "std")]
impl From<Box<Path>> for Cow<'_, Path> {
    #[inline]
    fn from(p: Box<Path>) -> Self {
        Cow::owned(p.into_path_buf())
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a str> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a str) -> Self {
        Cow::borrowed(Path::new(p))
    }
}

#[cfg(feature = "std")]
impl From<String> for Cow<'_, Path> {
    #[inline]
    fn from(p: String) -> Self {
        Cow::owned(PathBuf::from(p))
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a String> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a String) -> Self {
        Cow::borrowed(Path::new(p.as_str()))
    }
}

#[cfg(feature = "std")]
impl From<Box<str>> for Cow<'_, Path> {
    #[inline]
    fn from(p: Box<str>) -> Self {
        Cow::owned(PathBuf::from(p.into_string()))
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a OsStr> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a OsStr) -> Self {
        Cow::borrowed(Path::new(p))
    }
}

#[cfg(feature = "std")]
impl From<OsString> for Cow<'_, Path> {
    #[inline]
    fn from(p: OsString) -> Self {
        Cow::owned(PathBuf::from(p))
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a OsString> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a OsString) -> Self {
        Cow::borrowed(Path::new(p.as_os_str()))
    }
}

#[cfg(feature = "std")]
impl From<Box<OsStr>> for Cow<'_, Path> {
    #[inline]
    fn from(p: Box<OsStr>) -> Self {
        Cow::owned(PathBuf::from(p.into_os_string()))
    }
}
