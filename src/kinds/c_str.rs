#![cfg(feature = "std")]

use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::num::NonZeroU8;
use std::vec::Vec;

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Into
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<Cow<'a, CStr>> for CString {
    #[inline]
    fn from(s: Cow<'a, CStr>) -> Self {
        s.into_owned()
    }
}

impl<'a> From<Cow<'a, CStr>> for Box<CStr> {
    #[inline]
    fn from(s: Cow<'a, CStr>) -> Self {
        match s {
            Cow::Borrowed(b) => Box::from(b),
            Cow::Owned(o) => Box::from(o),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a CStr> for Cow<'a, CStr> {
    #[inline]
    fn from(s: &'a CStr) -> Self {
        Cow::Borrowed(s)
    }
}

impl<'a> From<CString> for Cow<'a, CStr> {
    #[inline]
    fn from(s: CString) -> Self {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a CString> for Cow<'a, CStr> {
    #[inline]
    fn from(s: &'a CString) -> Self {
        Cow::Borrowed(s.as_c_str())
    }
}

impl<'a> From<Box<CStr>> for Cow<'a, CStr> {
    #[inline]
    fn from(s: Box<CStr>) -> Self {
        Cow::Owned(s.into_c_string())
    }
}

impl<'a> From<Vec<NonZeroU8>> for Cow<'a, CStr> {
    #[inline]
    fn from(s: Vec<NonZeroU8>) -> Self {
        Cow::Owned(CString::from(s))
    }
}
