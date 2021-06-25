use std::iter::FromIterator;
use std::string::String;

use crate::Cow;

impl<'a> From<&'a str> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<String> for Cow<'a, str> {
    #[inline]
    fn from(s: String) -> Cow<'a, str> {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a String> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a String) -> Cow<'a, str> {
        Cow::Borrowed(s.as_str())
    }
}

impl<'a> FromIterator<char> for Cow<'a, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(it: I) -> Cow<'a, str> {
        Cow::Owned(FromIterator::from_iter(it))
    }
}

impl<'a, 'b> FromIterator<&'b str> for Cow<'a, str> {
    fn from_iter<I: IntoIterator<Item = &'b str>>(it: I) -> Cow<'a, str> {
        Cow::Owned(FromIterator::from_iter(it))
    }
}

impl<'a> FromIterator<String> for Cow<'a, str> {
    fn from_iter<I: IntoIterator<Item = String>>(it: I) -> Cow<'a, str> {
        Cow::Owned(FromIterator::from_iter(it))
    }
}
