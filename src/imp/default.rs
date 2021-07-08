//! The standard library [`Cow`] implementation.

pub use alloc::borrow::Cow;
use alloc::borrow::ToOwned;

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
        &*self
    }

    #[inline]
    fn into_owned(self) -> T::Owned {
        self.into_owned()
    }

    #[inline]
    fn apply<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T::Owned),
    {
        match self {
            Self::Borrowed(b) => {
                let mut o = b.to_owned();
                f(&mut o);
                *self = Self::Owned(o);
            }
            Self::Owned(ref mut o) => f(o),
        }
    }
}
