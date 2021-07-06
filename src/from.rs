use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::Cow;

macro_rules! impl_from {
    ($(
        $(#[$attrs:meta])*
        {
            $Owned:ty, $Ty:ty, $({ $($clone:tt)+ }, { $($copy:tt)+ },)?
            $as_ref:ident, $into_owned:ident
        },
    )+) => {
        $(
            $(#[$attrs])*
            impl<'a $(, $($clone)+)?> From<Cow<'a, $Ty>> for $Owned {
                #[inline]
                fn from(s: Cow<'a, $Ty>) -> Self {
                    s.into_owned()
                }
            }

            $(#[$attrs])*
            impl<'a $(, $($copy)+)?> From<Cow<'a, $Ty>> for Box<$Ty> {
                #[inline]
                fn from(s: Cow<'a, $Ty>) -> Self {
                    s.into_boxed()
                }
            }

            $(#[$attrs])*
            impl<'a $(, $($clone)+)?> From<&'a $Ty> for Cow<'a, $Ty> {
                #[inline]
                fn from(t: &'a $Ty) -> Self {
                    Self::borrowed(t)
                }
            }

            $(#[$attrs])*
            impl<'a $(, $($clone)+)?> From<$Owned> for Cow<'a, $Ty> {
                #[inline]
                fn from(t: $Owned) -> Self {
                    Self::owned(t)
                }
            }

            $(#[$attrs])*
            impl<'a $(, $($clone)+)?> From<&'a $Owned> for Cow<'a, $Ty> {
                #[inline]
                fn from(t: &'a $Owned) -> Self {
                    Cow::borrowed(t.$as_ref())
                }
            }

            $(#[$attrs])*
            impl<'a $(, $($clone)+)?> From<Box<$Ty>> for Cow<'a, $Ty> {
                #[inline]
                fn from(t: Box<$Ty>) -> Self {
                    Cow::owned(t.$into_owned())
                }
            }
        )+
    };
}

macro_rules! impl_from_other {
    ($(
        $(#[$attrs:meta])*
        {  $FromOwned:ty, $FromTy:ty, $as_ref:ident => $Owned:ty, $Ty:ty }
    )+) => {
        $(
            $(#[$attrs])*
            impl<'a> From<&'a $FromTy> for Cow<'a, $Ty> {
                #[inline]
                fn from(s: &'a $FromTy) -> Self {
                    Cow::borrowed(<$Ty>::new(s))
                }
            }

            $(#[$attrs])*
            impl<'a> From<$FromOwned> for Cow<'a, $Ty> {
                #[inline]
                fn from(s: $FromOwned) -> Self {
                    Cow::owned(<$Owned>::from(s))
                }
            }

            $(#[$attrs])*
            impl<'a> From<&'a $FromOwned> for Cow<'a, $Ty> {
                #[inline]
                fn from(s: &'a $FromOwned) -> Self {
                    Cow::borrowed(<$Ty>::new(s.$as_ref()))
                }
            }
        )+
    };
}

impl_from! {
    { String, str, as_str, into_string },

    { Vec<T>, [T], { T: 'a + Clone }, { T: 'a + Copy }, as_slice, into_vec },

    #[cfg(feature = "std")]
    { CString, CStr, as_c_str, into_c_string },

    #[cfg(feature = "std")]
    { OsString, OsStr, as_os_str, into_os_string },

    #[cfg(feature = "std")]
    { PathBuf, Path, as_path, into_path_buf },
}

impl_from_other! {
    #[cfg(feature = "std")]
    { String, str, as_str => OsString, OsStr }

    #[cfg(feature = "std")]
    { PathBuf, Path, as_os_str => OsString, OsStr }

    #[cfg(feature = "std")]
    { String, str, as_str => PathBuf, Path }

    #[cfg(feature = "std")]
    { OsString, OsStr, as_os_str  => PathBuf, Path }
}

impl<'a> From<char> for Cow<'a, str> {
    #[inline]
    fn from(c: char) -> Self {
        Cow::owned(String::from(c))
    }
}

impl<'a, T: 'a + Clone, const N: usize> From<[T; N]> for Cow<'a, [T]> {
    #[inline]
    fn from(v: [T; N]) -> Self {
        Cow::owned(<[T]>::into_vec(Box::new(v)))
    }
}
