#![allow(clippy::from_iter_instead_of_collect)]

use core::iter::FromIterator;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Cow<str>
////////////////////////////////////////////////////////////////////////////////

impl<'a, 'b> FromIterator<&'b str> for Cow<'a, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b str>>(iter: I) -> Self {
        Cow::owned(String::from_iter(iter))
    }
}

impl FromIterator<String> for Cow<'_, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Cow::owned(String::from_iter(iter))
    }
}

impl<'a, 'b> FromIterator<&'b String> for Cow<'a, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b String>>(iter: I) -> Self {
        Cow::owned(String::from_iter(iter.into_iter().map(|s| s.as_str())))
    }
}

impl<'a> FromIterator<Cow<'a, str>> for Cow<'a, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Cow<'a, str>>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            None => Cow::default(),
            Some(mut cow) => {
                cow.extend(iter);
                cow
            }
        }
    }
}

impl FromIterator<Box<str>> for Cow<'_, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Box<str>>>(iter: I) -> Self {
        Cow::owned(String::from_iter(iter))
    }
}

impl FromIterator<char> for Cow<'_, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        Cow::owned(String::from_iter(iter))
    }
}

impl<'a, 'b> FromIterator<&'b char> for Cow<'a, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b char>>(iter: I) -> Self {
        Cow::owned(String::from_iter(iter))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<[T]>
////////////////////////////////////////////////////////////////////////////////

impl<'a, 'b, T: 'a + 'b + Copy> FromIterator<&'b T> for Cow<'a, [T]> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b T>>(iter: I) -> Self {
        Cow::owned(Vec::from_iter(iter.into_iter().copied()))
    }
}

impl<'a, 'b, T: 'a + Clone> FromIterator<T> for Cow<'a, [T]> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Cow::owned(Vec::from_iter(iter))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<OsStr>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl<'a, 'b> FromIterator<&'b OsStr> for Cow<'a, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b OsStr>>(iter: I) -> Self {
        Cow::owned(OsString::from_iter(iter))
    }
}

#[cfg(feature = "std")]
impl FromIterator<OsString> for Cow<'_, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = OsString>>(iter: I) -> Self {
        Cow::owned(OsString::from_iter(iter))
    }
}

#[cfg(feature = "std")]
impl<'a, 'b> FromIterator<&'b OsString> for Cow<'a, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b OsString>>(iter: I) -> Self {
        Cow::owned(OsString::from_iter(iter.into_iter().map(|s| s.as_os_str())))
    }
}

#[cfg(feature = "std")]
impl<'a> FromIterator<Cow<'a, OsStr>> for Cow<'a, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Cow<'a, OsStr>>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            None => Cow::default(),
            Some(mut cow) => {
                cow.extend(iter);
                cow
            }
        }
    }
}

#[cfg(feature = "std")]
impl FromIterator<Box<OsStr>> for Cow<'_, OsStr> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Box<OsStr>>>(iter: I) -> Self {
        Cow::owned(OsString::from_iter(
            iter.into_iter().map(|b| b.into_os_string()),
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<Path>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl<'a, 'b> FromIterator<&'b Path> for Cow<'a, Path> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b Path>>(iter: I) -> Self {
        Cow::owned(PathBuf::from_iter(iter))
    }
}

#[cfg(feature = "std")]
impl FromIterator<PathBuf> for Cow<'_, Path> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = PathBuf>>(iter: I) -> Self {
        Cow::owned(PathBuf::from_iter(iter))
    }
}

#[cfg(feature = "std")]
impl<'a, 'b> FromIterator<&'b PathBuf> for Cow<'a, Path> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'b PathBuf>>(iter: I) -> Self {
        Cow::owned(PathBuf::from_iter(iter.into_iter().map(|s| s.as_os_str())))
    }
}

#[cfg(feature = "std")]
impl<'a> FromIterator<Cow<'a, Path>> for Cow<'a, Path> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Cow<'a, Path>>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            None => Cow::default(),
            Some(mut cow) => {
                cow.extend(iter);
                cow
            }
        }
    }
}

#[cfg(feature = "std")]
impl FromIterator<Box<Path>> for Cow<'_, Path> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Box<Path>>>(iter: I) -> Self {
        Cow::owned(PathBuf::from_iter(
            iter.into_iter().map(|b| b.into_path_buf()),
        ))
    }
}
