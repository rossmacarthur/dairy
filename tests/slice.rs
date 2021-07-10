#![allow(clippy::from_iter_instead_of_collect)]

use std::borrow::Borrow;
use std::ffi::OsStr;
use std::iter::FromIterator;

use dairy::Cow;

type T<'a> = Cow<'a, [&'a str]>;

#[test]
fn cow_slice_is_borrowed() {
    let c = T::borrowed(&["Hello", "World!"]);
    assert!(c.is_borrowed());
    assert!(!c.is_owned());
}

#[test]
fn cow_slice_is_owned() {
    let c = T::owned(vec!["Hello", "World!"]);
    assert!(!c.is_borrowed());
    assert!(c.is_owned());
}

#[test]
fn cow_slice_borrowed_into_owned() {
    let c = T::borrowed(&["Hello", "World!"]);
    let v: Vec<&str> = c.into_owned();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_owned_into_owned() {
    let c = T::owned(vec!["Hello", "World!"]);
    let v: Vec<&str> = c.into_owned();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_borrowed_ref() {
    let c = T::borrowed(&["Hello", "World!"]);

    // Deref
    let v: &[&str] = &*c;
    assert_eq!(v, &["Hello", "World!"]);

    // Borrow
    let v: &[&str] = c.borrow();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_owned_ref() {
    let c = T::owned(vec!["Hello", "World!"]);

    // Deref
    let v: &[&str] = &*c;
    assert_eq!(v, &["Hello", "World!"]);

    // Borrow
    let v: &[&str] = c.borrow();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_borrowed_clone() {
    let c1 = T::borrowed(&["Hello", "World!"]);
    let c2 = c1.clone();
    assert!(c1.is_borrowed());
    assert!(c2.is_borrowed());
}

#[test]
fn cow_slice_owned_clone() {
    let c1 = T::owned(vec!["Hello", "World!"]);
    let c2 = c1.clone();
    assert!(c1.is_owned());
    assert!(c2.is_owned());
}

#[test]
fn cow_slice_borrowed_as_ref() {
    let c = T::borrowed(&["Hello", "World!"]);

    let v: &[&str] = c.as_ref();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_owned_as_ref() {
    let c = T::owned(vec!["Hello", "World!"]);

    let v: &[&str] = c.as_ref();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_from() {
    Vec::from(T::borrowed(&["Hello", "World!"]));
    Vec::from(T::owned(vec!["Hello", "World!"]));

    Box::<[&str]>::from(T::borrowed(&["Hello", "World!"]));
    Box::<[&str]>::from(T::owned(vec!["Hello", "World!"]));

    assert!(Cow::<[&str]>::from(&["Hello", "World!"][..]).is_borrowed());
    assert!(Cow::<[&str]>::from(vec!["Hello", "World!"]).is_owned());
    assert!(Cow::<[&str]>::from(&vec!["Hello", "World!"]).is_borrowed());
    assert!(Cow::<[&str]>::from(vec!["Hello", "World!"].into_boxed_slice()).is_owned());
}

#[test]
fn cow_slice_borrowed_partial_eq() {
    let c = T::borrowed(&["Hello", "World!"]);

    assert_eq!(c, ["Hello", "World!"]);
    assert_eq!(c, &["Hello", "World!"]);
    assert_eq!(c, &["Hello", "World!"][..]);
    assert_eq!(c, vec!["Hello", "World!"]);
    assert_eq!(c, &vec!["Hello", "World!"]);
    // assert_eq!(c, Box::new(&["Hello", "World!"][..]));

    assert_eq!(c, [OsStr::new("Hello"), OsStr::new("World!")]);
    assert_eq!(c, &[OsStr::new("Hello"), OsStr::new("World!")]);
    assert_eq!(c, vec![OsStr::new("Hello"), OsStr::new("World!")]);
    assert_eq!(c, &vec![OsStr::new("Hello"), OsStr::new("World!")]);
}

#[test]
fn cow_slice_extend() {
    let mut c = T::default();
    c.extend(vec!["Hello", "World!"]);
    assert_eq!(c, &["Hello", "World!"]);

    let mut c = T::default();
    c.extend(&["Hello", "World!"]);
    assert_eq!(c, &["Hello", "World!"]);
}

#[test]
fn cow_slice_from_iter() {
    let c = T::from_iter(vec!["Hello", "World!"]);
    assert_eq!(c, &["Hello", "World!"]);

    let c = T::from_iter(&["Hello", "World!"]);
    assert_eq!(c, &["Hello", "World!"]);
}
