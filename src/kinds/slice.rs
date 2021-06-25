use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::FromIterator;

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Into
////////////////////////////////////////////////////////////////////////////////

impl<'a, T: Clone> From<Cow<'a, [T]>> for Vec<T> {
    #[inline]
    fn from(s: Cow<'a, [T]>) -> Self {
        s.into_owned()
    }
}

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a, T: Clone> From<&'a [T]> for Cow<'a, [T]> {
    #[inline]
    fn from(s: &'a [T]) -> Self {
        Cow::Borrowed(s)
    }
}

impl<'a, T: Clone> From<Vec<T>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: Vec<T>) -> Self {
        Cow::Owned(v)
    }
}

impl<'a, T: Clone> From<&'a Vec<T>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: &'a Vec<T>) -> Self {
        Cow::Borrowed(v.as_slice())
    }
}

impl<'a, T: Clone, const N: usize> From<[T; N]> for Cow<'a, [T]> {
    #[inline]
    fn from(v: [T; N]) -> Self {
        Cow::Owned(<[T]>::into_vec(Box::new(v)))
    }
}

impl<'a, T: Clone> From<Box<[T]>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: Box<[T]>) -> Self {
        Cow::Owned(v.into_vec())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From iterator
////////////////////////////////////////////////////////////////////////////////

impl<'a, T: Clone> FromIterator<&'a T> for Cow<'a, [T]> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(it: I) -> Self {
        Cow::Owned(Vec::from_iter(it.into_iter().cloned()))
    }
}

impl<'a, T: Clone> FromIterator<T> for Cow<'a, [T]> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        Cow::Owned(Vec::from_iter(it))
    }
}

impl<'a, T: Clone> FromIterator<Cow<'a, T>> for Cow<'a, [T]> {
    fn from_iter<I: IntoIterator<Item = Cow<'a, T>>>(it: I) -> Self {
        Cow::Owned(Vec::from_iter(it.into_iter().map(Cow::into_owned)))
    }
}
