use std::iter::FromIterator;
use std::vec::Vec;

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Into
////////////////////////////////////////////////////////////////////////////////

impl<'a, T: Clone> From<Cow<'a, [T]>> for Vec<T> {
    #[inline]
    fn from(s: Cow<'a, [T]>) -> Vec<T> {
        s.into_owned()
    }
}

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a, T: Clone> From<&'a [T]> for Cow<'a, [T]> {
    #[inline]
    fn from(s: &'a [T]) -> Cow<'a, [T]> {
        Cow::Borrowed(s)
    }
}

impl<'a, T: Clone> From<Vec<T>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: Vec<T>) -> Cow<'a, [T]> {
        Cow::Owned(v)
    }
}

impl<'a, T: Clone> From<&'a Vec<T>> for Cow<'a, [T]> {
    #[inline]
    fn from(v: &'a Vec<T>) -> Cow<'a, [T]> {
        Cow::Borrowed(v.as_slice())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From iterator
////////////////////////////////////////////////////////////////////////////////

impl<'a, T: Clone> FromIterator<T> for Cow<'a, [T]> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Cow<'a, [T]> {
        Cow::Owned(FromIterator::from_iter(it))
    }
}
