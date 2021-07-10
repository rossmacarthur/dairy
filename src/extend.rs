use alloc::boxed::Box;
use alloc::string::String;

use crate::Cow;

#[cfg(feature = "std")]
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

////////////////////////////////////////////////////////////////////////////////
// Cow<str>
////////////////////////////////////////////////////////////////////////////////

fn extend_str<I, U>(cow: &mut Cow<str>, iter: I)
where
    I: IntoIterator<Item = U>,
    U: AsRef<str>,
{
    let iter = iter.into_iter();
    let (lower, _) = iter.size_hint();
    cow.apply(move |o| {
        o.reserve(lower);
        iter.for_each(move |item| o.push_str(item.as_ref()))
    });
}

impl<'a, 'b> Extend<&'b str> for Cow<'a, str> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b str>>(&mut self, iter: I) {
        extend_str(self, iter)
    }
}

impl Extend<String> for Cow<'_, str> {
    #[inline]
    fn extend<I: IntoIterator<Item = String>>(&mut self, iter: I) {
        extend_str(self, iter)
    }
}

impl<'a, 'b> Extend<&'b String> for Cow<'a, str> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b String>>(&mut self, iter: I) {
        extend_str(self, iter)
    }
}

impl<'a, 'b> Extend<Cow<'b, str>> for Cow<'a, str> {
    #[inline]
    fn extend<I: IntoIterator<Item = Cow<'b, str>>>(&mut self, iter: I) {
        extend_str(self, iter)
    }
}

impl Extend<Box<str>> for Cow<'_, str> {
    #[inline]
    fn extend<I: IntoIterator<Item = Box<str>>>(&mut self, iter: I) {
        extend_str(self, iter)
    }
}

impl Extend<char> for Cow<'_, str> {
    #[inline]
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        self.apply(move |o| {
            o.reserve(lower);
            iter.for_each(move |c| o.push(c))
        });
    }
}

impl<'a, 'b> Extend<&'b char> for Cow<'a, str> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b char>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<[T]>
////////////////////////////////////////////////////////////////////////////////

impl<'a, 'b, T: 'a + 'b + Copy> Extend<&'b T> for Cow<'a, [T]> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b T>>(&mut self, iter: I) {
        self.apply(move |o| o.extend(iter))
    }
}

impl<'a, 'b, T: 'a + Clone> Extend<T> for Cow<'a, [T]> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.apply(move |o| o.extend(iter))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<OsStr>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
fn extend_os_str<I, U>(cow: &mut Cow<OsStr>, iter: I)
where
    I: IntoIterator<Item = U>,
    U: AsRef<OsStr>,
{
    let iter = iter.into_iter();
    let (lower, _) = iter.size_hint();
    cow.apply(move |o| {
        o.reserve(lower);
        iter.for_each(move |item| o.push(item))
    });
}

#[cfg(feature = "std")]
impl<'a, 'b> Extend<&'b OsStr> for Cow<'a, OsStr> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b OsStr>>(&mut self, iter: I) {
        extend_os_str(self, iter)
    }
}

#[cfg(feature = "std")]
impl Extend<OsString> for Cow<'_, OsStr> {
    #[inline]
    fn extend<I: IntoIterator<Item = OsString>>(&mut self, iter: I) {
        extend_os_str(self, iter)
    }
}

#[cfg(feature = "std")]
impl<'a, 'b> Extend<&'b OsString> for Cow<'a, OsStr> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b OsString>>(&mut self, iter: I) {
        extend_os_str(self, iter)
    }
}

#[cfg(feature = "std")]
impl<'a, 'b> Extend<Cow<'b, OsStr>> for Cow<'a, OsStr> {
    #[inline]
    fn extend<I: IntoIterator<Item = Cow<'b, OsStr>>>(&mut self, iter: I) {
        extend_os_str(self, iter)
    }
}

#[cfg(feature = "std")]
impl Extend<Box<OsStr>> for Cow<'_, OsStr> {
    #[inline]
    fn extend<I: IntoIterator<Item = Box<OsStr>>>(&mut self, iter: I) {
        extend_os_str(self, iter)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<Path>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
fn extend_path<I, U>(cow: &mut Cow<Path>, iter: I)
where
    I: IntoIterator<Item = U>,
    U: AsRef<Path>,
{
    let iter = iter.into_iter();
    let (lower, _) = iter.size_hint();
    cow.apply(move |o| {
        o.reserve(lower);
        iter.for_each(move |item| o.push(item))
    });
}

#[cfg(feature = "std")]
impl<'a, 'b> Extend<&'b Path> for Cow<'a, Path> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b Path>>(&mut self, iter: I) {
        extend_path(self, iter)
    }
}

#[cfg(feature = "std")]
impl Extend<PathBuf> for Cow<'_, Path> {
    #[inline]
    fn extend<I: IntoIterator<Item = PathBuf>>(&mut self, iter: I) {
        extend_path(self, iter)
    }
}

#[cfg(feature = "std")]
impl<'a, 'b> Extend<&'b PathBuf> for Cow<'a, Path> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'b PathBuf>>(&mut self, iter: I) {
        extend_path(self, iter)
    }
}

#[cfg(feature = "std")]
impl<'a, 'b> Extend<Cow<'b, Path>> for Cow<'a, Path> {
    #[inline]
    fn extend<I: IntoIterator<Item = Cow<'b, Path>>>(&mut self, iter: I) {
        extend_path(self, iter)
    }
}

#[cfg(feature = "std")]
impl Extend<Box<Path>> for Cow<'_, Path> {
    #[inline]
    fn extend<I: IntoIterator<Item = Box<Path>>>(&mut self, iter: I) {
        extend_path(self, iter)
    }
}
