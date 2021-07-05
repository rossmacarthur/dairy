//! A more compact, user friendly clone-on-write smart pointer.
//!
//! ```
//! use dairy::Cow;
//! let borrowed: Cow<str> = Cow::borrowed("Hello World!");
//! let owned: Cow<str> = Cow::owned(String::from("Hello World!"));
//! ```
//!
//! [`dairy::Cow`][Cow] is an improved version of the standard library
//! [`std::borrow::Cow`]. Depending on the platform and type this crate provides
//! a better underlying implementation which will be more compact. This crate
//! currently supports the following types: [`str`], [`[T]`][slice], [`CStr`],
//! [`OsStr`], and [`Path`].
//!
//! [`dairy::Cow`][Cow] is also able to provide many more [`From`]
//! implementations; some which are not possible for the standard library to
//! provide due to the `alloc`, `std` split. For example `Cow<Path>` now has
//! the useful `From<&str>` implementation.
//!
//! ### Underlying implementation
//!
//! - On 64-bit platforms the *compact* implementation of [`Cow`] is two words
//!   wide, storing the length, capacity, and the ownership tag in the same
//!   word.
//! - On 32-bit platforms the *compact* implementation of [`Cow`] is three words
//!   wide, storing the capacity and the ownership tag in the same word.
//! - The **default** implementation simply uses the the standard library
//!   implementation which is four words wide. This is typically required in
//!   cases where the standard library does not provide an `.into_raw_parts()`
//!   or equivalent method for the owned version of types.
//!
//! The following table documents how `Cow<T>` is implemented for each type on
//! depending on the platform.
//!
//! | `Cow<T>`     | Unix/WASI | Other          |
//! | ------------ | --------- | -------------- |
//! | `Cow<str>`   | *compact* | *compact*      |
//! | `Cow<[T]>`   | *compact* | *compact*      |
//! | `Cow<CStr>`  | *compact* | *compact*      |
//! | `Cow<OsStr>` | *compact* | **default**    |
//! | `Cow<Path>`  | *compact* | **default**    |

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod as_ref;
mod cmp;
mod from;
mod imp;
mod serde;
mod to_boxed;

use core::borrow::Borrow;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::Unpin;
use core::ops::Deref;

use alloc::boxed::Box;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, OsStr},
    path::Path,
};

use crate::imp::Cow as _;
pub use crate::imp::Dairy;
pub use crate::to_boxed::ToBoxed;

/// Convenient type alias for a clone-on-write [`str`].
pub type String<'a> = Cow<'a, str>;

/// Convenient type alias for a clone-on-write [`[T]`][slice].
pub type Vec<'a, T> = Cow<'a, [T]>;

/// Convenient type alias for a clone-on-write [`CStr`].
#[cfg(feature = "std")]
pub type CString<'a> = Cow<'a, CStr>;

/// Convenient type alias for a clone-on-write [`OsStr`].
#[cfg(feature = "std")]
pub type OsString<'a> = Cow<'a, OsStr>;

/// Convenient type alias for a clone-on-write [`Path`].
#[cfg(feature = "std")]
pub type PathBuf<'a> = Cow<'a, Path>;

/// A clone-on-write smart pointer.
///
/// The type `Cow` is a smart pointer providing clone-on-write functionality: it
/// can enclose and provide immutable access to borrowed data, and clone the
/// data lazily when mutation or ownership is required.
///
/// `Cow` implements [`Deref`], which means that you can call non-mutating
/// methods directly on the data it encloses.
pub struct Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
{
    inner: T::Cow,
}

impl<'a, T> Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
{
    /// Construct from borrowed data.
    #[inline]
    pub fn borrowed(b: &'a T) -> Self {
        Self {
            inner: T::Cow::borrowed(b),
        }
    }

    /// Construct from owned data.
    #[inline]
    pub fn owned(o: T::Owned) -> Self {
        Self {
            inner: T::Cow::owned(o),
        }
    }

    /// Returns true if the data is borrowed.
    #[inline]
    pub fn is_borrowed(&self) -> bool {
        self.inner.is_borrowed()
    }

    /// Returns true if the data is owned.
    #[inline]
    pub fn is_owned(&self) -> bool {
        self.inner.is_owned()
    }

    #[inline]
    fn make_ref(&self) -> &T {
        self.inner.make_ref()
    }

    /// Converts into owned data.
    ///
    /// Clones the data if it is not already owned.
    #[inline]
    pub fn into_owned(self) -> T::Owned {
        self.inner.into_owned()
    }

    /// Converts into boxed data.
    ///
    /// Clones the data if it is not already owned.
    #[inline]
    pub fn into_boxed(self) -> Box<T>
    where
        T: ToBoxed,
    {
        T::to_boxed(self.into_owned())
    }
}

impl<'a, T> Deref for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.make_ref()
    }
}

impl<'a, T> Borrow<T> for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
{
    #[inline]
    fn borrow(&self) -> &T {
        self.make_ref()
    }
}

impl<'a, T> Clone for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> fmt::Debug for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + fmt::Debug,
    T::Owned: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T> fmt::Display for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + fmt::Display,
    T::Owned: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T> Default for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
    T::Owned: Default,
{
    #[inline]
    fn default() -> Self {
        Self::owned(T::Owned::default())
    }
}

impl<'a, T> Hash for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

unsafe impl<'a, T> Send for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + Sync,
    T::Owned: Send,
{
}

unsafe impl<'a, T> Sync for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + Sync,
    T::Owned: Sync,
{
}

impl<'a, T> Unpin for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
    T::Owned: Unpin,
{
}
