use std::ffi::{CStr, CString};

use crate::Cow;

impl<'a> From<&'a CStr> for Cow<'a, CStr> {
    #[inline]
    fn from(s: &'a CStr) -> Cow<'a, CStr> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<CString> for Cow<'a, CStr> {
    #[inline]
    fn from(s: CString) -> Cow<'a, CStr> {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a CString> for Cow<'a, CStr> {
    #[inline]
    fn from(s: &'a CString) -> Cow<'a, CStr> {
        Cow::Borrowed(s.as_c_str())
    }
}
