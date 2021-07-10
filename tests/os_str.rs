#![allow(clippy::from_iter_instead_of_collect)]

use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use dairy::Cow;

type T<'a> = Cow<'a, OsStr>;

#[test]
fn cow_os_str_is_borrowed() {
    let c = T::borrowed(OsStr::new("Hello World!"));
    assert!(c.is_borrowed());
    assert!(!c.is_owned());
}

#[test]
fn cow_os_str_is_owned() {
    let c = T::owned(OsString::from("Hello World!"));
    assert!(!c.is_borrowed());
    assert!(c.is_owned());
}

#[test]
fn cow_os_str_borrowed_into_owned() {
    let c = T::borrowed(OsStr::new("Hello World!"));
    let s: OsString = c.into_owned();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_os_str_owned_into_owned() {
    let c = T::owned(OsString::from("Hello World!"));
    let s: OsString = c.into_owned();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_os_str_borrowed_ref() {
    let c = T::borrowed(OsStr::new("Hello World!"));

    // Deref
    let s: &OsStr = &*c;
    assert_eq!(s, "Hello World!");

    // Borrow
    let s: &OsStr = c.borrow();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_os_str_owned_ref() {
    let c = T::owned(OsString::from("Hello World!"));

    // Deref
    let s: &OsStr = &*c;
    assert_eq!(s, "Hello World!");

    // Borrow
    let s: &OsStr = c.borrow();
    assert_eq!(s, "Hello World!");
}

#[test]
fn cow_os_str_borrowed_clone() {
    let c1 = T::borrowed(OsStr::new("Hello World!"));
    let c2 = c1.clone();
    assert!(c1.is_borrowed());
    assert!(c2.is_borrowed());
}

#[test]
fn cow_os_str_owned_clone() {
    let c1 = T::owned(OsString::from("Hello World!"));
    let c2 = c1.clone();
    assert!(c1.is_owned());
    assert!(c2.is_owned());
}

#[test]
fn cow_os_str_borrowed_as_ref() {
    let c = T::borrowed(OsStr::new("Hello World!"));

    let s: &OsStr = c.as_ref();
    assert_eq!(s, OsStr::new("Hello World!"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("Hello World!"));
}

#[test]
fn cow_os_str_owned_as_ref() {
    let c = T::owned(OsString::from("Hello World!"));

    let s: &OsStr = c.as_ref();
    assert_eq!(s, OsStr::new("Hello World!"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("Hello World!"));
}

#[test]
fn cow_os_str_from() {
    OsString::from(T::borrowed(OsStr::new("Hello World!")));
    OsString::from(T::owned(OsString::from("Hello World!")));

    Box::<OsStr>::from(T::borrowed(OsStr::new("Hello World!")));
    Box::<OsStr>::from(T::owned(OsString::from("Hello World!")));

    assert!(T::from('H').is_owned());
    assert!(T::from(OsStr::new("Hello World!")).is_borrowed());
    assert!(T::from(OsString::from("Hello World!")).is_owned());
    assert!(T::from(&OsString::from("Hello World!")).is_borrowed());
    assert!(T::from(OsString::from("Hello World!").into_boxed_os_str()).is_owned());

    assert!(T::from("Hello World!").is_borrowed());
    assert!(T::from(String::from("Hello World!")).is_owned());
    assert!(T::from(&String::from("Hello World!")).is_borrowed());
    assert!(T::from(String::from("Hello World!").into_boxed_str()).is_owned());

    assert!(T::from(Path::new("Hello World!")).is_borrowed());
    assert!(T::from(PathBuf::from("Hello World!")).is_owned());
    assert!(T::from(&PathBuf::from("Hello World!")).is_borrowed());
    assert!(T::from(PathBuf::from("Hello World!").into_boxed_path()).is_owned());
}

#[test]
fn cow_os_str_borrowed_partial_eq() {
    let c = T::borrowed(OsStr::new("Hello World!"));

    assert_eq!(c, T::from("Hello World!"));
    assert_eq!(c, Cow::<str>::from("Hello World!"));
    assert_eq!(c, Cow::<Path>::from("Hello World!"));

    assert_eq!(c, *OsStr::new("Hello World!"));
    assert_eq!(c, OsStr::new("Hello World!"));
    assert_eq!(c, OsString::from("Hello World!"));
    assert_eq!(c, &OsString::from("Hello World!"));
    // assert_eq!(c, Box::new(OsStr::new("Hello World!")));

    // assert_eq!(c, *"Hello World!");
    // assert_eq!(c, "Hello World!");
    // assert_eq!(c, String::from("Hello World!"));
    // assert_eq!(c, &String::from("Hello World!"));
    // assert_eq!(c, Box::new("Hello World!"));

    assert_eq!(c, *Path::new("Hello World!"));
    assert_eq!(c, Path::new("Hello World!"));
    assert_eq!(c, PathBuf::from("Hello World!"));
    assert_eq!(c, &PathBuf::from("Hello World!"));
    // assert_eq!(c, Box::new(Path::new("Hello World!")));
}

#[test]
fn cow_os_str_extend() {
    let borrowed: Vec<&OsStr> = vec![
        OsStr::new("Hel"),
        OsStr::new("lo "),
        OsStr::new("Wor"),
        OsStr::new("ld!"),
    ];
    let owned: Vec<OsString> = borrowed.iter().copied().map(OsString::from).collect();

    let mut c = T::default();
    c.extend(borrowed.iter().copied());
    assert_eq!(c, OsStr::new("Hello World!"));

    let mut c = T::default();
    c.extend(owned.iter().cloned());
    assert_eq!(c, OsStr::new("Hello World!"));

    let mut c = T::default();
    c.extend(owned.iter());
    assert_eq!(c, OsStr::new("Hello World!"));

    let mut c = T::default();
    c.extend(owned.iter().map(T::from));
    assert_eq!(c, OsStr::new("Hello World!"));

    let mut c = T::default();
    c.extend(owned.iter().cloned().map(OsString::into_boxed_os_str));
    assert_eq!(c, OsStr::new("Hello World!"));
}

#[test]
fn cow_os_str_from_iter() {
    let borrowed: Vec<&OsStr> = vec![
        OsStr::new("Hel"),
        OsStr::new("lo "),
        OsStr::new("Wor"),
        OsStr::new("ld!"),
    ];
    let owned: Vec<OsString> = borrowed.iter().copied().map(OsString::from).collect();

    let c = T::from_iter(borrowed.iter().copied());
    assert_eq!(c, OsStr::new("Hello World!"));

    let c = T::from_iter(owned.iter().cloned());
    assert_eq!(c, OsStr::new("Hello World!"));

    let c = T::from_iter(owned.iter());
    assert_eq!(c, OsStr::new("Hello World!"));

    let c = T::from_iter(owned.iter().map(T::from));
    assert_eq!(c, OsStr::new("Hello World!"));

    let c = T::from_iter(owned.iter().cloned().map(OsString::into_boxed_os_str));
    assert_eq!(c, OsStr::new("Hello World!"));
}
