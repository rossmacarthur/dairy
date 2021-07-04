use alloc::borrow::ToOwned;
use alloc::boxed::Box;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, OsStr},
    path::Path,
};

/// Converts the owned version of self into boxed data.
pub trait ToBoxed: ToOwned {
    fn to_boxed(o: Self::Owned) -> Box<Self>;
}

impl ToBoxed for str {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_str()
    }
}

impl<T: Clone> ToBoxed for [T] {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_slice()
    }
}

#[cfg(feature = "std")]
impl ToBoxed for CStr {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_c_str()
    }
}

#[cfg(feature = "std")]
impl ToBoxed for OsStr {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_os_str()
    }
}

#[cfg(feature = "std")]
impl ToBoxed for Path {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_path()
    }
}
