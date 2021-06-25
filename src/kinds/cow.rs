use alloc::borrow::{Cow as StdCow, ToOwned};

use crate::Cow;

impl<'a, T> From<Cow<'a, T>> for StdCow<'a, T>
where
    T: ?Sized + ToOwned,
{
    #[inline]
    fn from(c: Cow<'a, T>) -> Self {
        match c {
            Cow::Borrowed(b) => StdCow::Borrowed(b),
            Cow::Owned(o) => StdCow::Owned(o),
        }
    }
}

impl<'a, T> From<StdCow<'a, T>> for Cow<'a, T>
where
    T: ?Sized + ToOwned,
{
    #[inline]
    fn from(c: StdCow<'a, T>) -> Self {
        match c {
            StdCow::Borrowed(b) => Cow::Borrowed(b),
            StdCow::Owned(o) => Cow::Owned(o),
        }
    }
}
