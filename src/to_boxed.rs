#![allow(clippy::wrong_self_convention)]

use alloc::borrow::ToOwned;
use alloc::boxed::Box;

/// Converts the owned version of self into boxed data.
///
/// This trait should not be used directly but instead through the
/// [`.into_boxed()`][crate::Cow::into_boxed] method on [`Cow`][crate::Cow].
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
impl ToBoxed for std::ffi::CStr {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_c_str()
    }
}

#[cfg(feature = "std")]
impl ToBoxed for std::ffi::OsStr {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_os_str()
    }
}

#[cfg(feature = "std")]
impl ToBoxed for std::path::Path {
    #[inline]
    fn to_boxed(o: Self::Owned) -> Box<Self> {
        o.into_boxed_path()
    }
}
