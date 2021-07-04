//! The underlying `Cow` implementations.

mod better;
mod orig;
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

/// Internal trait to provide a layer of indirection allows us to have different
/// [`Cow`](crate::Cow) implementations for the same type across different
/// platforms.
pub trait Dairy<'a>: ToOwned + sealed::Sealed {
    type Cow: Cow<'a, Self>;
}

impl<'a> Dairy<'a> for str {
    type Cow = better::Cow<'a, Self>;
}

impl<'a, T: Clone + 'a> Dairy<'a> for [T] {
    type Cow = better::Cow<'a, Self>;
}

#[cfg(feature = "std")]
impl<'a> Dairy<'a> for std::ffi::CStr {
    type Cow = better::Cow<'a, Self>;
}

#[cfg(unix)]
#[cfg(feature = "std")]
impl<'a> Dairy<'a> for std::ffi::OsStr {
    type Cow = better::Cow<'a, Self>;
}

#[cfg(windows)]
#[cfg(feature = "std")]
impl<'a> Dairy<'a> for std::ffi::OsStr {
    type Cow = alloc::borrow::Cow<'a, Self>;
}

#[cfg(unix)]
#[cfg(feature = "std")]
impl<'a> Dairy<'a> for std::path::Path {
    type Cow = better::Cow<'a, Self>;
}

#[cfg(windows)]
#[cfg(feature = "std")]
impl<'a> Dairy<'a> for std::path::Path {
    type Cow = alloc::borrow::Cow<'a, Self>;
}
