use alloc::boxed::Box;
use alloc::string::String;
use core::iter::FromIterator;
use core::ops::{Add, AddAssign};

use crate::Cow;

////////////////////////////////////////////////////////////////////////////////
// Into
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<Cow<'a, str>> for String {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        s.into_owned()
    }
}

impl<'a> From<Cow<'a, str>> for Box<str> {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        match s {
            Cow::Borrowed(b) => Box::from(b),
            Cow::Owned(o) => Box::from(o),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// From self
////////////////////////////////////////////////////////////////////////////////

impl<'a> From<char> for Cow<'a, str> {
    #[inline]
    fn from(c: char) -> Self {
        Cow::Owned(String::from(c))
    }
}

impl<'a> From<&'a str> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Cow::Borrowed(s)
    }
}

impl<'a> From<String> for Cow<'a, str> {
    #[inline]
    fn from(s: String) -> Self {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a String> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a String) -> Self {
        Cow::Borrowed(s.as_str())
    }
}

impl<'a> From<Box<str>> for Cow<'a, str> {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Cow::Owned(s.into_string())
    }
}

////////////////////////////////////////////////////////////////////////////////
// From iterator
////////////////////////////////////////////////////////////////////////////////

impl<'a> FromIterator<char> for Cow<'a, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(it: I) -> Self {
        Cow::Owned(String::from_iter(it))
    }
}

impl<'a> FromIterator<&'a char> for Cow<'a, str> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a char>>(it: I) -> Self {
        Cow::Owned(String::from_iter(it))
    }
}

impl<'a, 'b> FromIterator<&'b str> for Cow<'a, str> {
    fn from_iter<I: IntoIterator<Item = &'b str>>(it: I) -> Self {
        Cow::Owned(String::from_iter(it))
    }
}

impl<'a> FromIterator<String> for Cow<'a, str> {
    fn from_iter<I: IntoIterator<Item = String>>(it: I) -> Self {
        Cow::Owned(String::from_iter(it))
    }
}

impl<'a> FromIterator<Box<str>> for Cow<'a, str> {
    fn from_iter<I: IntoIterator<Item = Box<str>>>(it: I) -> Self {
        Cow::Owned(String::from_iter(it))
    }
}

impl<'a> FromIterator<Cow<'a, str>> for Cow<'a, str> {
    fn from_iter<I: IntoIterator<Item = Cow<'a, str>>>(it: I) -> Self {
        Cow::Owned(String::from_iter(it.into_iter().map(Cow::into_owned)))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Add / AddAssign
////////////////////////////////////////////////////////////////////////////////

impl<'a> Add<&'a str> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    #[inline]
    fn add(mut self, rhs: &'a str) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a> Add<Cow<'a, str>> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    #[inline]
    fn add(mut self, rhs: Cow<'a, str>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a> AddAssign<&'a str> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: &'a str) {
        if self.is_empty() {
            *self = Cow::Borrowed(rhs)
        } else if !rhs.is_empty() {
            if let Cow::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = Cow::Owned(s);
            }
            self.to_mut().push_str(rhs);
        }
    }
}

impl<'a> AddAssign<Cow<'a, str>> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: Cow<'a, str>) {
        if self.is_empty() {
            *self = rhs
        } else if !rhs.is_empty() {
            if let Cow::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = Cow::Owned(s);
            }
            self.to_mut().push_str(&rhs);
        }
    }
}
