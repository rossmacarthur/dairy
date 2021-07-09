use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::path::Path;

use dairy::Cow;

type T<'a> = Cow<'a, str>;

#[test]
fn cow_str_is_borrowed() {
    let c = T::borrowed("Hello World!");
    assert!(c.is_borrowed());
    assert!(!c.is_owned());
}

#[test]
fn cow_str_is_owned() {
    let c = T::owned(String::from("Hello World!"));
    assert!(!c.is_borrowed());
    assert!(c.is_owned());
}

#[test]
fn cow_str_borrowed_into_owned() {
    let c = T::borrowed("Hello World!");
    let s: String = c.into_owned();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_owned_into_owned() {
    let c = T::owned(String::from("Hello World!"));
    let s: String = c.into_owned();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_borrowed_ref() {
    let c = T::borrowed("Hello World!");

    // Deref
    let s: &str = &*c;
    assert_eq!(s, "Hello World!");

    // Borrow
    let s: &str = c.borrow();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_owned_ref() {
    let c = T::owned(String::from("Hello World!"));

    // Deref
    let s: &str = &*c;
    assert_eq!(s, "Hello World!");

    // Borrow
    let s: &str = c.borrow();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_str_borrowed_clone() {
    let c1 = T::borrowed("Hello World!");
    let c2 = c1.clone();
    assert!(c1.is_borrowed());
    assert!(c2.is_borrowed());
}

#[test]
fn cow_str_owned_clone() {
    let c1 = T::owned(String::from("Hello World!"));
    let c2 = c1.clone();
    assert!(c1.is_owned());
    assert!(c2.is_owned());
}

#[test]
fn cow_str_borrowed_as_ref() {
    let c = T::borrowed("Hello World!");

    let s: &str = c.as_ref();
    assert_eq!(s, "Hello World!");

    let s: &OsStr = c.as_ref();
    assert_eq!(s, OsStr::new("Hello World!"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("Hello World!"));
}

#[test]
fn cow_str_owned_as_ref() {
    let c = T::owned(String::from("Hello World!"));

    let s: &str = c.as_ref();
    assert_eq!(s, "Hello World!");

    let s: &OsStr = c.as_ref();
    assert_eq!(s, OsStr::new("Hello World!"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("Hello World!"));
}

#[test]
fn cow_str_from() {
    String::from(T::borrowed("Hello World!"));
    String::from(T::owned(String::from("Hello World!")));

    Box::<str>::from(T::borrowed("Hello World!"));
    Box::<str>::from(T::owned(String::from("Hello World!")));

    assert!(T::from('H').is_owned());
    assert!(T::from("Hello World!").is_borrowed());
    assert!(T::from(String::from("Hello World!")).is_owned());
    assert!(T::from(&String::from("Hello World!")).is_borrowed());
    assert!(T::from(String::from("Hello World!").into_boxed_str()).is_owned());
}

#[test]
fn cow_str_borrowed_partial_eq() {
    let c = T::borrowed("Hello World!");

    assert_eq!(c, T::from("Hello World!"));
    assert_eq!(c, Cow::<OsStr>::from("Hello World!"));

    assert_eq!(c, *"Hello World!");
    assert_eq!(c, "Hello World!");
    assert_eq!(c, String::from("Hello World!"));
    assert_eq!(c, &String::from("Hello World!"));
    // assert_eq!(c, Box::new("Hello World!"));

    assert_eq!(c, *OsStr::new("Hello World!"));
    assert_eq!(c, OsStr::new("Hello World!"));
    assert_eq!(c, OsString::from("Hello World!"));
    assert_eq!(c, &OsString::from("Hello World!"));
    // assert_eq!(c, Box::new(OsStr::new("Hello World!")));

    // assert_eq!(c, *Path::new("Hello World!"));
    // assert_eq!(c, Path::new("Hello World!"));
    // assert_eq!(c, PathBuf::from("Hello World!"));
    // assert_eq!(c, &PathBuf::from("Hello World!"));
}
