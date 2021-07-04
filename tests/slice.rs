use std::borrow::Borrow;
use std::ffi::OsStr;

use dairy::Cow;

#[test]
fn cow_slice_is_borrowed() {
    let c: Cow<[&str]> = Cow::borrowed(&["Hello", "World!"]);
    assert!(c.is_borrowed());
    assert!(!c.is_owned());
}

#[test]
fn cow_slice_is_owned() {
    let c: Cow<[&str]> = Cow::owned(vec!["Hello", "World!"]);
    assert!(!c.is_borrowed());
    assert!(c.is_owned());
}

#[test]
fn cow_slice_borrowed_into_owned() {
    let c: Cow<[&str]> = Cow::borrowed(&["Hello", "World!"]);
    let v: Vec<&str> = c.into_owned();
    assert_eq!(v, vec!["Hello", "World!"]);
}

#[test]
fn cow_slice_owned_into_owned() {
    let c: Cow<[&str]> = Cow::owned(vec!["Hello", "World!"]);
    let v: Vec<&str> = c.into_owned();
    assert_eq!(v, vec!["Hello", "World!"]);
}

#[test]
fn cow_slice_borrowed_ref() {
    let c: Cow<[&str]> = Cow::borrowed(&["Hello", "World!"]);

    // Deref
    let v: &[&str] = &*c;
    assert_eq!(v, &["Hello", "World!"]);

    // Borrow
    let v: &[&str] = c.borrow();
    assert_eq!(v, &["Hello", "World!"]);

    // AsRef
    let v: &[&str] = c.as_ref();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_owned_ref() {
    let c: Cow<[&str]> = Cow::owned(vec!["Hello", "World!"]);

    // Deref
    let v: &[&str] = &*c;
    assert_eq!(v, &["Hello", "World!"]);

    // Borrow
    let v: &[&str] = c.borrow();
    assert_eq!(v, &["Hello", "World!"]);

    // AsRef
    let v: &[&str] = c.as_ref();
    assert_eq!(v, &["Hello", "World!"]);
}

#[test]
fn cow_slice_borrowed_clone() {
    let c1: Cow<[&str]> = Cow::borrowed(&["Hello", "World!"]);
    let c2 = c1.clone();
    assert!(c1.is_borrowed());
    assert!(c2.is_borrowed());
}

#[test]
fn cow_slice_owned_clone() {
    let c1: Cow<[&str]> = Cow::owned(vec!["Hello", "World!"]);
    let c2 = c1.clone();
    assert!(c1.is_owned());
    assert!(c2.is_owned());
}

#[test]
fn cow_slice_basic_from() {
    Vec::<&str>::from(Cow::<[&str]>::borrowed(&["Hello", "World!"]));
    Vec::<&str>::from(Cow::<[&str]>::owned(vec!["Hello", "World!"]));
    Box::<[&str]>::from(Cow::<[&str]>::borrowed(&["Hello", "World!"]));
    Box::<[&str]>::from(Cow::<[&str]>::owned(vec!["Hello", "World!"]));
    Cow::<[&str]>::from(&["Hello", "World!"][..]);
    Cow::<[&str]>::from(vec!["Hello", "World!"]);
    Cow::<[&str]>::from(&vec!["Hello", "World!"]);
    Cow::<[&str]>::from(vec!["Hello", "World!"].into_boxed_slice());
}

#[test]
fn cow_slice_borrowed_partial_eq() {
    let c: Cow<[&str]> = Cow::borrowed(&["Hello", "World!"]);

    assert_eq!(c, ["Hello", "World!"]);
    assert_eq!(c, &["Hello", "World!"]);
    assert_eq!(c, vec!["Hello", "World!"]);
    assert_eq!(c, &vec!["Hello", "World!"]);

    assert_eq!(c, [OsStr::new("Hello"), OsStr::new("World!")]);
    assert_eq!(c, &[OsStr::new("Hello"), OsStr::new("World!")]);
    assert_eq!(c, vec![OsStr::new("Hello"), OsStr::new("World!")]);
    assert_eq!(c, &vec![OsStr::new("Hello"), OsStr::new("World!")]);
}
