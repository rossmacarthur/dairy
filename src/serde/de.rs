use core::fmt;
use core::marker::PhantomData;
use core::str;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use serde::de;
use serde::{Deserialize, Deserializer};

use crate::{Cow, Dairy};

struct Visitor<'de, 'a, T: ?Sized + Dairy>(PhantomData<fn() -> (&'de T, Cow<'a, T>)>);

////////////////////////////////////////////////////////////////////////////////
// Cow<str>
////////////////////////////////////////////////////////////////////////////////

impl<'de, 'a> de::Visitor<'de> for Visitor<'de, 'a, str>
where
    'de: 'a,
{
    type Value = Cow<'a, str>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match str::from_utf8(v) {
            Ok(s) => Ok(Cow::owned(s.to_owned())),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match str::from_utf8(v) {
            Ok(s) => Ok(Cow::borrowed(s)),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match String::from_utf8(v) {
            Ok(s) => Ok(Cow::owned(s)),
            Err(e) => Err(de::Error::invalid_value(
                de::Unexpected::Bytes(&e.into_bytes()),
                &self,
            )),
        }
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Cow::owned(value.to_owned()))
    }

    #[inline]
    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Cow::borrowed(value))
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Cow::owned(value))
    }
}

impl<'de, 'a> Deserialize<'de> for Cow<'a, str>
where
    'de: 'a,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::<'de, 'a, str>(PhantomData))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<[T]>
////////////////////////////////////////////////////////////////////////////////

impl<'de, 'a, T: 'a + Clone> Deserialize<'de> for Cow<'a, [T]>
where
    T: Deserialize<'de>,
    'de: 'a,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::deserialize(deserializer).map(Cow::owned)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<CStr>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl<'de, 'a> Deserialize<'de> for Cow<'a, CStr>
where
    'de: 'a,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        CString::deserialize(deserializer).map(Cow::owned)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<OsStr>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl<'de, 'a> Deserialize<'de> for Cow<'a, OsStr>
where
    'de: 'a,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        OsString::deserialize(deserializer).map(Cow::owned)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cow<Path>
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl<'de, 'a> Deserialize<'de> for Cow<'a, Path>
where
    'de: 'a,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        PathBuf::deserialize(deserializer).map(Cow::owned)
    }
}
