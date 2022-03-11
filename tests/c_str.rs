use std::borrow::Borrow;
use std::ffi::{CStr, CString};

use serde_derive::{Deserialize, Serialize};

use dairy::Cow;

type T<'a> = Cow<'a, CStr>;

fn c_str() -> &'static CStr {
    unsafe { CStr::from_bytes_with_nul_unchecked(b"Hello World!\x00") }
}
fn c_string() -> CString {
    CString::new("Hello World!").unwrap()
}

#[test]
fn cow_c_str_is_borrowed() {
    let c = T::borrowed(c_str());
    assert!(c.is_borrowed());
    assert!(!c.is_owned());
}

#[test]
fn cow_c_str_is_owned() {
    let c = T::owned(CString::new("Hello World!").unwrap());
    assert!(!c.is_borrowed());
    assert!(c.is_owned());
}

#[test]
fn cow_c_str_borrowed_into_owned() {
    let c = T::borrowed(c_str());
    let s: CString = c.into_owned();
    assert_eq!(s, c_string());
}

#[test]
fn cow_c_str_owned_into_owned() {
    let c = T::owned(c_string());
    let s: CString = c.into_owned();
    assert_eq!(s, c_string());
}

#[test]
fn cow_c_str_borrowed_ref() {
    let c = T::borrowed(c_str());

    // Deref
    let s: &CStr = &*c;
    assert_eq!(s, c_str());

    // Borrow
    let s: &CStr = c.borrow();
    assert_eq!(s, c_str());
}

#[test]
fn cow_c_str_owned_ref() {
    let c = T::owned(c_string());

    // Deref
    let s: &CStr = &*c;
    assert_eq!(s, c_str());

    // Borrow
    let s: &CStr = c.borrow();
    assert_eq!(s, c_str());
}

#[test]
fn cow_c_str_borrowed_clone() {
    let c1 = T::borrowed(c_str());
    let c2 = c1.clone();
    assert!(c1.is_borrowed());
    assert!(c2.is_borrowed());
}

#[test]
fn cow_c_str_owned_clone() {
    let c1 = T::owned(c_string());
    let c2 = c1.clone();
    assert!(c1.is_owned());
    assert!(c2.is_owned());
}

#[test]
fn cow_c_str_borrowed_as_ref() {
    let c = T::borrowed(c_str());

    let s: &CStr = c.as_ref();
    assert_eq!(s, c_str());
}

#[test]
fn cow_c_str_owned_as_ref() {
    let c = T::owned(c_string());

    let s: &CStr = c.as_ref();
    assert_eq!(s, c_str());
}

#[test]
fn cow_c_str_from() {
    CString::from(T::borrowed(c_str()));
    CString::from(T::owned(c_string()));

    Box::<CStr>::from(T::borrowed(c_str()));
    Box::<CStr>::from(T::owned(c_string()));

    assert!(T::from(c_str()).is_borrowed());
    assert!(T::from(c_string()).is_owned());
    assert!(T::from(&c_string()).is_borrowed());
    assert!(T::from(c_string().into_boxed_c_str()).is_owned());
}

#[test]
fn cow_c_str_borrowed_partial_eq() {
    let c = T::borrowed(c_str());

    assert_eq!(c, T::borrowed(c_str()));

    assert_eq!(c, *c_str());
    assert_eq!(c, c_str());
    assert_eq!(c, c_string());
    assert_eq!(c, &c_string());
    // assert_eq!(c, Box::new(c_str()));
}

#[test]
fn cow_c_str_serde() {
    #[derive(Serialize, Deserialize)]
    struct Test<'a> {
        #[serde(borrow)]
        borrowed: Cow<'a, CStr>,
        owned: Cow<'a, CStr>,
    }

    let t: Test = serde_json::from_str(
        r#"{
            "borrowed: "Hello World!",
            "owned":"Hello World!"
        }"#,
    )
    .unwrap();

    assert_eq!(t.borrowed, c_str());
    assert!(t.borrowed.is_borrowed());
    assert_eq!(t.owned, c_str());
    assert!(t.owned.is_owned());
}
