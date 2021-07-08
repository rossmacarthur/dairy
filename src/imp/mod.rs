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
}

/// Internal trait which allows us to have different [`Cow`](crate::Cow)
/// implementations for the same type across different platforms.
///
/// This is a *sealed* trait so only this crate can implement it.
pub trait Dairy<'a>: ToOwned + sealed::Sealed {
    type Cow: Cow<'a, Self>;
}

impl<'a> Dairy<'a> for str {
    type Cow = compact::Cow<'a, Self>;
}

impl<'a, T: Clone + 'a> Dairy<'a> for [T] {
    type Cow = compact::Cow<'a, Self>;
}

#[cfg(feature = "std")]
impl<'a> Dairy<'a> for std::ffi::CStr {
    type Cow = compact::Cow<'a, Self>;
}

#[cfg(all(feature = "std", os_str_ext))]
impl<'a> Dairy<'a> for std::ffi::OsStr {
    type Cow = compact::Cow<'a, Self>;
}

#[cfg(all(feature = "std", not(os_str_ext)))]
impl<'a> Dairy<'a> for std::ffi::OsStr {
    type Cow = default::Cow<'a, Self>;
}

#[cfg(all(feature = "std", os_str_ext))]
impl<'a> Dairy<'a> for std::path::Path {
    type Cow = compact::Cow<'a, Self>;
}

#[cfg(all(feature = "std", not(os_str_ext)))]
impl<'a> Dairy<'a> for std::path::Path {
    type Cow = default::Cow<'a, Self>;
}
