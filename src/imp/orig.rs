//! The standard library [`Cow`] implementation.

use alloc::borrow::{Borrow, Cow, ToOwned};

use crate::imp;

impl<'a, T> imp::Cow<'a, T> for Cow<'a, T>
where
    T: ?Sized + ToOwned,
{
    #[inline]
    fn borrowed(b: &'a T) -> Self {
        Self::Borrowed(b)
    }

    #[inline]
    fn owned(o: T::Owned) -> Self {
        Self::Owned(o)
    }

    #[inline]
    fn is_borrowed(&self) -> bool {
        matches!(*self, Self::Borrowed(_))
    }

    #[inline]
    fn is_owned(&self) -> bool {
        matches!(*self, Self::Owned(_))
    }

    #[inline]
    fn make_ref(&self) -> &T {
        match *self {
            Self::Borrowed(b) => b,
            Self::Owned(ref o) => o.borrow(),
        }
    }

    #[inline]
    fn into_owned(self) -> T::Owned {
        match self {
            Self::Borrowed(b) => b.to_owned(),
            Self::Owned(o) => o,
        }
    }
}
