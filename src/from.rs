use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::Cow;

macro_rules! impl_basic {
    ($(
        $(#[$attrs:meta])*
        (
            $Owned:ty, $Ty:ty, $({ $($clone:tt)+ }, { $($copy:tt)+ },)?
            $as_ref:ident, $into_owned:ident),
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

macro_rules! impl_from {
    ($(
        $(#[$attrs:meta])*
        (($IntoOwned:ty, $IntoTy:ty) <= ($Owned:ty, $Ty:ty, $as_ref:ident) )
    )+) => {
        $(
            $(#[$attrs])*
            impl<'a> From<&'a $Ty> for Cow<'a, $IntoTy> {
                #[inline]
                fn from(s: &'a $Ty) -> Self {
                    Cow::borrowed(<$IntoTy>::new(s))
                }
            }

            $(#[$attrs])*
            impl<'a> From<$Owned> for Cow<'a, $IntoTy> {
                #[inline]
                fn from(s: $Owned) -> Self {
                    Cow::owned(<$IntoOwned>::from(s))
                }
            }

            $(#[$attrs])*
            impl<'a> From<&'a $Owned> for Cow<'a, $IntoTy> {
                #[inline]
                fn from(s: &'a $Owned) -> Self {
                    Cow::borrowed(<$IntoTy>::new(s.$as_ref()))
                }
            }
        )+
    };
}

impl_basic! {
    (String, str, as_str, into_string),

    (Vec<T>, [T], { T: 'a + Clone }, { T: 'a + Copy }, as_slice, into_vec),

    #[cfg(feature = "std")]
    (CString, CStr, as_c_str, into_c_string),

    #[cfg(feature = "std")]
    (OsString, OsStr, as_os_str, into_os_string),

    #[cfg(feature = "std")]
    (PathBuf, Path, as_path, into_path_buf),
}

impl_from! {
    #[cfg(feature = "std")]
    ((OsString, OsStr) <= (String, str, as_str))

    #[cfg(feature = "std")]
    ((OsString, OsStr) <= (PathBuf, Path, as_os_str))

    #[cfg(feature = "std")]
    ((PathBuf, Path) <= (String, str, as_str))

    #[cfg(feature = "std")]
    ((PathBuf, Path) <= (OsString, OsStr, as_os_str))
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
