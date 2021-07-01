//! A more compact, user friendly clone-on-write smart pointer.
//!
//! ```
//! use std::path::Path;
//! use dairy::Cow;
//!
//! let borrowed: Cow<str> = Cow::borrowed("Hello World!");
//! let owned: Cow<str> = Cow::owned(String::from("Hello World!"));
//! ```
//!
//! [`dairy::Cow`][Cow] is an improved version of the standard library
//! [`std::borrow::Cow`]. It is just 2 words wide, storing the length, capacity,
//! and the ownership tag all in one word.
//!
//! [`dairy::Cow`][Cow] has many more [`From`] and [`PartialEq`] implementations.
//! Most notably for `Cow<Path>` making `Into<Cow<Path>>` just as nice to use as
//! `Cow<str>`.
//!
//! Unfortunately these benefits come with some caveats:
//!
//! - Only `str`, `[T]`, `OsStr`, `CStr` and `Path` types are supported.
//! - Additionally, `OsStr` and `Path` are only supported on Unix.
//! - On 32-bit operating systems the maximum length is `u16::MAX` which is
//!   not sufficient for all use cases.
//!
//! ## Acknowledgements
//!
//! Some implementation details taken from the excellent
//! [beef](https://github.com/maciejhirsz/beef) crate.

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod as_ref;
mod cmp;
mod convert;
mod from;
mod serde;

use core::borrow::Borrow;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::{PhantomData, Unpin};
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ptr::NonNull;

use alloc::boxed::Box;

pub use crate::convert::Convert;
use crate::convert::IsOwned;

/// A clone-on-write smart pointer.
///
/// The type `Cow` is a smart pointer providing clone-on-write functionality: it
/// can enclose and provide immutable access to borrowed data, and clone the
/// data lazily when mutation or ownership is required.
///
/// `Cow` implements `Deref`, which means that you can call non-mutating methods
/// directly on the data it encloses. If mutation is desired, `to_mut` will
/// obtain a mutable reference to an owned value, cloning if necessary.
pub struct Cow<'a, T>
where
    T: ?Sized + Convert,
{
    /// Pointer to the data.
    ptr: NonNull<T::Ptr>,

    /// Any extra data that is required to reconstruct and owned or borrowed
    /// variant of this type. For example: length and capacity.
    extra: T::Extra,

    /// For the lifetime.
    marker: PhantomData<&'a T>,
}

impl<'a, T> Cow<'a, T>
where
    T: ?Sized + Convert,
{
    /// Construct from borrowed data.
    #[inline]
    pub fn borrowed(b: &'a T) -> Self {
        let (ptr, extra) = T::unmake_borrowed(b);
        Self {
            ptr,
            extra,
            marker: PhantomData,
        }
    }

    /// Construct from owned data.
    #[inline]
    pub fn owned(o: T::Owned) -> Self {
        let (ptr, extra) = T::unmake_owned(o);
        Self {
            ptr,
            extra,
            marker: PhantomData,
        }
    }

    #[inline]
    fn make_borrowed(&self) -> &'a T {
        // SAFETY: This is valid for both owned and borrowed variants.
        unsafe { &*T::make_ptr(self.ptr, self.extra) }
    }

    /// Returns true if the data is borrowed.
    #[inline]
    pub fn is_borrowed(&self) -> bool {
        !self.extra.is_owned()
    }

    /// Returns true if the data is owned.
    #[inline]
    pub fn is_owned(&self) -> bool {
        self.extra.is_owned()
    }

    /// Converts this `Cow<T>` into owned data.
    ///
    /// Clones the data if it is not already owned.
    pub fn into_owned(self) -> T::Owned {
        if self.is_owned() {
            let cow = ManuallyDrop::new(self);
            unsafe { T::make_owned(cow.ptr, cow.extra) }
        } else {
            self.make_borrowed().to_owned()
        }
    }

    /// Converts this `Cow<T>` into a [`Box<T>`].
    ///
    /// Clones the data if it is not already owned.
    #[inline]
    fn into_boxed(self) -> Box<T> {
        T::to_boxed(self.into_owned())
    }
}

impl<T> Drop for Cow<'_, T>
where
    T: ?Sized + Convert,
{
    #[inline]
    fn drop(&mut self) {
        if self.is_owned() {
            unsafe { T::make_owned(self.ptr, self.extra) };
        }
    }
}

impl<T> Deref for Cow<'_, T>
where
    T: ?Sized + Convert,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.make_borrowed()
    }
}

impl<'a, T> Borrow<T> for Cow<'a, T>
where
    T: ?Sized + Convert,
{
    #[inline]
    fn borrow(&self) -> &T {
        self.make_borrowed()
    }
}

impl<T> Clone for Cow<'_, T>
where
    T: ?Sized + Convert,
{
    #[inline]
    fn clone(&self) -> Self {
        if self.is_owned() {
            Self::owned(self.make_borrowed().to_owned())
        } else {
            Self { ..*self }
        }
    }
}

impl<T> fmt::Debug for Cow<'_, T>
where
    T: ?Sized + Convert + fmt::Debug,
    T::Owned: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T> fmt::Display for Cow<'_, T>
where
    T: ?Sized + Convert + fmt::Display,
    T::Owned: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T> Default for Cow<'_, T>
where
    T: ?Sized + Convert,
    T::Owned: Default,
{
    #[inline]
    fn default() -> Self {
        Self::owned(T::Owned::default())
    }
}

impl<T> Hash for Cow<'_, T>
where
    T: ?Sized + Convert + Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

unsafe impl<T> Send for Cow<'_, T>
where
    T: ?Sized + Convert + Sync,
    T::Owned: Send,
{
}

unsafe impl<T> Sync for Cow<'_, T>
where
    T: ?Sized + Convert + Sync,
    T::Owned: Sync,
{
}

impl<T> Unpin for Cow<'_, T>
where
    T: ?Sized + Convert,
    T::Owned: Unpin,
{
}
