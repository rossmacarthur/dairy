use std::borrow::Borrow;
use std::ffi::OsStr;
use std::path::Path;

use dairy::Cow;

#[test]
fn cow_str_is_borrowed() {
    let c: Cow<str> = Cow::borrowed("Hello World!");
    assert!(c.is_borrowed());
    assert!(!c.is_owned());
}

#[test]
fn cow_str_is_owned() {
    let c: Cow<str> = Cow::owned(String::from("Hello World!"));
    assert!(!c.is_borrowed());
    assert!(c.is_owned());
}

#[test]
fn cow_str_borrowed_into_owned() {
    let c: Cow<str> = Cow::borrowed("Hello World!");
    let s: String = c.into_owned();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_owned_into_owned() {
    let c: Cow<str> = Cow::owned(String::from("Hello World!"));
    let s: String = c.into_owned();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_borrowed_ref() {
    let c: Cow<str> = Cow::borrowed("Hello World!");

    // Deref
    let s: &str = &*c;
    assert_eq!(s, "Hello World!");

    // Borrow
    let s: &str = c.borrow();
    assert_eq!(s, "Hello World!");

    // AsRef
    let s: &str = c.as_ref();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_owned_ref() {
    let c: Cow<str> = Cow::owned(String::from("Hello World!"));

    // Deref
    let s: &str = &*c;
    assert_eq!(s, "Hello World!");

    // Borrow
    let s: &str = c.borrow();
    assert_eq!(s, "Hello World!");

    // AsRef
    let s: &str = c.as_ref();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_borrowed_clone() {
    let c1: Cow<str> = Cow::borrowed("Hello World!");
    let c2 = c1.clone();
    assert!(c1.is_borrowed());
    assert!(c2.is_borrowed());
}

#[test]
fn cow_str_owned_clone() {
    let c1: Cow<str> = Cow::owned(String::from("Hello World!"));
    let c2 = c1.clone();
    assert!(c1.is_owned());
    assert!(c2.is_owned());
}

#[test]
fn cow_str_borrowed_as_ref() {
    let c: Cow<str> = Cow::borrowed("Hello World!");

    let s: &str = c.as_ref();
    assert_eq!(s, "Hello World!");

    let s: &OsStr = c.as_ref();
    assert_eq!(s, OsStr::new("Hello World!"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("Hello World!"));
}

#[test]
fn cow_str_owned_as_ref() {
    let c: Cow<str> = Cow::owned(String::from("Hello World!"));

    let s: &str = c.as_ref();
    assert_eq!(s, "Hello World!");

    let s: &OsStr = c.as_ref();
    assert_eq!(s, OsStr::new("Hello World!"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("Hello World!"));
}

#[test]
fn cow_str_basic_from() {
    String::from(Cow::<str>::borrowed("Hello World"));
    String::from(Cow::<str>::owned(String::from("Hello World")));
    Box::<str>::from(Cow::<str>::borrowed("Hello World"));
    Box::<str>::from(Cow::<str>::owned(String::from("Hello World")));
    Cow::<str>::from('H');
    Cow::<str>::from("Hello World!");
    Cow::<str>::from(String::from("Hello World!"));
    Cow::<str>::from(&String::from("Hello World!"));
    Cow::<str>::from(String::from("Hello World!").into_boxed_str());
}
