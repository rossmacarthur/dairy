use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use dairy::Cow;

type T<'a> = Cow<'a, Path>;

#[test]
fn cow_path_is_borrowed() {
    let c = T::borrowed(Path::new("/hello/world"));
    assert!(c.is_borrowed());
    assert!(!c.is_owned());
}

#[test]
fn cow_path_is_owned() {
    let c = T::owned(PathBuf::from("/hello/world"));
    assert!(!c.is_borrowed());
    assert!(c.is_owned());
}

#[test]
fn cow_path_borrowed_into_owned() {
    let c = T::borrowed(Path::new("/hello/world"));
    let p: PathBuf = c.into_owned();
    assert_eq!(p, Path::new("/hello/world"));
}

#[test]
fn cow_path_owned_into_owned() {
    let c = T::owned(PathBuf::from("/hello/world"));
    let p: PathBuf = c.into_owned();
    assert_eq!(p, Path::new("/hello/world"));
}

#[test]
fn cow_path_borrowed_ref() {
    let c = T::borrowed(Path::new("/hello/world"));

    // Deref
    let p: &Path = &*c;
    assert_eq!(p, Path::new("/hello/world"));

    // Borrow
    let p: &Path = c.borrow();
    assert_eq!(p, Path::new("/hello/world"));
}

#[test]
fn cow_path_owned_ref() {
    let c = T::owned(PathBuf::from("/hello/world"));

    // Deref
    let p: &Path = &*c;
    assert_eq!(p, Path::new("/hello/world"));

    // Borrow
    let p: &Path = c.borrow();
    assert_eq!(p, Path::new("/hello/world"));
}

#[test]
fn cow_path_borrowed_clone() {
    let c1 = T::borrowed(Path::new("/hello/world"));
    let c2 = c1.clone();
    assert!(c1.is_borrowed());
    assert!(c2.is_borrowed());
}

#[test]
fn cow_path_owned_clone() {
    let c1 = T::owned(PathBuf::from("/hello/world"));
    let c2 = c1.clone();
    assert!(c1.is_owned());
    assert!(c2.is_owned());
}

#[test]
fn cow_path_borrowed_as_ref() {
    let c = T::borrowed(Path::new("/hello/world"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("/hello/world"));

    let p: &OsStr = c.as_ref();
    assert_eq!(p, Path::new("/hello/world"));
}

#[test]
fn cow_path_owned_as_ref() {
    let c = T::owned(PathBuf::from("/hello/world"));

    let p: &Path = c.as_ref();
    assert_eq!(p, Path::new("/hello/world"));

    let p: &OsStr = c.as_ref();
    assert_eq!(p, Path::new("/hello/world"));
}

#[test]
fn cow_path_from() {
    PathBuf::from(T::borrowed(Path::new("/hello/world")));
    PathBuf::from(T::owned(PathBuf::from("/hello/world")));

    Box::<Path>::from(T::borrowed(Path::new("/hello/world")));
    Box::<Path>::from(T::owned(PathBuf::from("/hello/world")));

    assert!(T::from(Path::new("/hello/world")).is_borrowed());
    assert!(T::from(PathBuf::from("/hello/world")).is_owned());
    assert!(T::from(&PathBuf::from("/hello/world")).is_borrowed());
    assert!(T::from(PathBuf::from("/hello/world").into_boxed_path()).is_owned());

    assert!(T::from("/hello/world").is_borrowed());
    assert!(T::from(String::from("/hello/world")).is_owned());
    assert!(T::from(&String::from("/hello/world")).is_borrowed());
    assert!(T::from(String::from("/hello/world").into_boxed_str()).is_owned());

    assert!(T::from(OsStr::new("/hello/world")).is_borrowed());
    assert!(T::from(OsString::from("/hello/world")).is_owned());
    assert!(T::from(&OsString::from("/hello/world")).is_borrowed());
    assert!(T::from(OsString::from("/hello/world").into_boxed_os_str()).is_owned());
}

#[test]
fn cow_path_borrowed_partial_eq() {
    let c = T::borrowed(Path::new("/hello/world"));

    assert_eq!(c, T::from("/hello/world"));
    // assert_eq!(c, Cow::<str>::from("/hello/world"));
    assert_eq!(c, Cow::<OsStr>::from("/hello/world"));

    assert_eq!(c, *Path::new("/hello/world"));
    assert_eq!(c, Path::new("/hello/world"));
    assert_eq!(c, PathBuf::from("/hello/world"));
    assert_eq!(c, &PathBuf::from("/hello/world"));
    // assert_eq!(c, Box::new(Path::new("/hello/world")));

    // assert_eq!(c, *"/hello/world");
    // assert_eq!(c, "/hello/world");
    // assert_eq!(c, String::from("/hello/world"));
    // assert_eq!(c, &String::from("/hello/world"));
    // assert_eq!(c, Box::new("/hello/world"));

    assert_eq!(c, *OsStr::new("/hello/world"));
    assert_eq!(c, OsStr::new("/hello/world"));
    assert_eq!(c, OsString::from("/hello/world"));
    assert_eq!(c, &OsString::from("/hello/world"));
    // assert_eq!(c, Box::new(OsStr::new("/hello/world")));
}
