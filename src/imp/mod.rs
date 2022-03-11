//! The underlying `Cow` implementations.

mod compact;
mod default;
mod sealed;

use alloc::borrow::ToOwned;

/// Defines a `Cow` implementation.
///
/// This trait allows us to provide multiple `Cow` implementations.
pub trait Cow<'a, T>: Clone
where
    T: ?Sized + ToOwned,
{
    fn borrowed(b: &'a T) -> Self;
    fn owned(o: T::Owned) -> Self;
    fn is_borrowed(&self) -> bool;
    fn is_owned(&self) -> bool;
    fn make_ref(&self) -> &T;
    fn into_owned(self) -> T::Owned;
    fn apply<F: FnOnce(&mut T::Owned)>(&mut self, f: F);
}

/// Internal trait which allows us to have different [`Cow`](crate::Cow)
/// implementations for the same type across different platforms.
///
/// This is a *sealed* trait so only this crate can implement it.
pub trait Dairy: ToOwned + sealed::Sealed {
    type Cow<'a>: Cow<'a, Self>;
}

impl Dairy for str {
    type Cow<'a> = compact::Cow<'a, Self>;
}

impl<'t, T: 't + Clone> Dairy for [T]
where
    't: 'a,
{
    type Cow<'a> = compact::Cow<'t, Self>;
}

#[cfg(feature = "std")]
impl Dairy for std::ffi::CStr {
    type Cow<'a> = compact::Cow<'a, Self>;
}

#[cfg(all(feature = "std", os_str_ext))]
impl Dairy for std::ffi::OsStr {
    type Cow<'a> = compact::Cow<'a, Self>;
}

#[cfg(all(feature = "std", not(os_str_ext)))]
impl Dairy for std::ffi::OsStr {
    type Cow<'a> = default::Cow<'a, Self>;
}

#[cfg(all(feature = "std", os_str_ext))]
impl Dairy for std::path::Path {
    type Cow<'a> = compact::Cow<'a, Self>;
}

#[cfg(all(feature = "std", not(os_str_ext)))]
impl<'a> Dairy for std::path::Path {
    type Cow = default::Cow<'a, Self>;
}
