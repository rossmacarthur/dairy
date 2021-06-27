#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod from;
mod serde;

use alloc::borrow::ToOwned;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

use Cow::*;

/// A clone-on-write smart pointer.
///
/// The type `Cow` is a smart pointer providing clone-on-write functionality: it
/// can enclose and provide immutable access to borrowed data, and clone the
/// data lazily when mutation or ownership is required. The type is designed to
/// work with general borrowed data via the `Borrow` trait.
///
/// `Cow` implements `Deref`, which means that you can call
/// non-mutating methods directly on the data it encloses. If mutation
/// is desired, `to_mut` will obtain a mutable reference to an owned
/// value, cloning if necessary.
pub enum Cow<'a, T>
where
    T: ?Sized + ToOwned,
{
    /// Borrowed data.
    Borrowed(&'a T),

    /// Owned data.
    Owned(T::Owned),
}

impl<'a, T> Borrow<T> for Cow<'a, T>
where
    T: ?Sized + ToOwned,
{
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T> Clone for Cow<'_, T>
where
    T: ?Sized + ToOwned,
{
    fn clone(&self) -> Self {
        match *self {
            Borrowed(b) => Borrowed(b),
            Owned(ref o) => {
                let b: &T = o.borrow();
                Owned(b.to_owned())
            }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (&mut Owned(ref mut dest), &Owned(ref o)) => *dest = o.borrow().to_owned(),
            (t, s) => *t = s.clone(),
        }
    }
}

impl<T> Cow<'_, T>
where
    T: ?Sized + ToOwned,
{
    /// Acquires a mutable reference to the owned form of the data.
    ///
    /// Clones the data if it is not already owned.
    pub fn to_mut(&mut self) -> &mut T::Owned {
        match *self {
            Borrowed(borrowed) => {
                *self = Owned(borrowed.to_owned());
                match *self {
                    Borrowed(..) => unreachable!(),
                    Owned(ref mut owned) => owned,
                }
            }
            Owned(ref mut owned) => owned,
        }
    }

    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    pub fn into_owned(self) -> T::Owned {
        match self {
            Borrowed(borrowed) => borrowed.to_owned(),
            Owned(owned) => owned,
        }
    }
}

impl<T> Deref for Cow<'_, T>
where
    T: ?Sized + ToOwned,
{
    type Target = T;

    fn deref(&self) -> &T {
        match *self {
            Borrowed(borrowed) => borrowed,
            Owned(ref owned) => owned.borrow(),
        }
    }
}

impl<T> Eq for Cow<'_, T> where T: ?Sized + ToOwned + Eq {}

impl<T> Ord for Cow<'_, T>
where
    T: ?Sized + ToOwned + Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<'a, 'b, T, U> PartialEq<Cow<'b, U>> for Cow<'a, T>
where
    T: ?Sized + ToOwned + PartialEq<U>,
    U: ?Sized + ToOwned,
{
    #[inline]
    fn eq(&self, other: &Cow<'b, U>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<'a, T> PartialOrd for Cow<'a, T>
where
    T: ?Sized + ToOwned + PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, T>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<T> fmt::Debug for Cow<'_, T>
where
    T: ?Sized + ToOwned + fmt::Debug,
    T::Owned: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Borrowed(ref b) => fmt::Debug::fmt(b, f),
            Owned(ref o) => fmt::Debug::fmt(o, f),
        }
    }
}

impl<T> fmt::Display for Cow<'_, T>
where
    T: ?Sized + ToOwned + fmt::Display,
    T::Owned: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Borrowed(ref b) => fmt::Display::fmt(b, f),
            Owned(ref o) => fmt::Display::fmt(o, f),
        }
    }
}

impl<T> Default for Cow<'_, T>
where
    T: ?Sized + ToOwned,
    T::Owned: Default,
{
    fn default() -> Self {
        Owned(T::Owned::default())
    }
}

impl<T> Hash for Cow<'_, T>
where
    T: ?Sized + ToOwned + Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T> AsRef<T> for Cow<'_, T>
where
    T: ?Sized + ToOwned,
{
    fn as_ref(&self) -> &T {
        self
    }
}
