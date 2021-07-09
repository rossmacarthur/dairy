#[cfg(feature = "std")]
use std::{ffi::OsStr, path::Path};

use crate::{Cow, Dairy};

impl<'a, T> AsRef<T> for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.make_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<str>
////////////////////////////////////////////////////////////////////////////////

impl AsRef<[u8]> for Cow<'_, str> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.make_ref().as_ref()
    }
}

#[cfg(feature = "std")]
impl AsRef<OsStr> for Cow<'_, str> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.make_ref().as_ref()
    }
}

#[cfg(feature = "std")]
impl AsRef<Path> for Cow<'_, str> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.make_ref().as_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<OsStr>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl AsRef<Path> for Cow<'_, OsStr> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.make_ref().as_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<Path>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl AsRef<OsStr> for Cow<'_, Path> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.make_ref().as_ref()
    }
}
