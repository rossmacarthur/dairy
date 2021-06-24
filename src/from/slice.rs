use std::vec::Vec;

use crate::Cow;

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
