use core::convert::Infallible;

use alloc::str::FromStr;
use alloc::string::String;

#[cfg(feature = "std")]
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::Cow;

impl<'a> FromStr for Cow<'a, str> {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::owned(String::from(s)))
    }
}

#[cfg(feature = "std")]
impl<'a> FromStr for Cow<'a, OsStr> {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::owned(OsString::from(s)))
    }
}

#[cfg(feature = "std")]
impl<'a> FromStr for Cow<'a, Path> {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::owned(PathBuf::from(s)))
    }
}
