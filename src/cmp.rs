use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::{Cow, Dairy};

impl<'a, 'b, T, U> PartialEq<Cow<'b, U>> for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + PartialEq<U>,
    U: ?Sized + Dairy<'b>,
{
    #[inline]
    fn eq(&self, other: &Cow<'b, U>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<'a, T> Eq for Cow<'a, T> where T: ?Sized + Dairy<'a> + Eq {}

impl<'a, T> PartialOrd for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, T>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<'a, T> Ord for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

macro_rules! impl_basic {
    ($(
        $(#[$attrs:meta])*
        ($Ty:ty, $To:ty $(, { $($bound:tt)+ })?)
    )+) => {
        $(
            $(#[$attrs])*
            impl<'a $(, $($bound)+)?> PartialEq<$To> for Cow<'a, $Ty> {
                #[inline]
                fn eq(&self, other: &$To) -> bool {
                    PartialEq::eq(&**self, &*other)
                }
            }

            $(#[$attrs])*
            impl<'a $(, $($bound)+)?> PartialEq<&$To> for Cow<'a, $Ty> {
                #[inline]
                fn eq(&self, other: &&$To) -> bool {
                    PartialEq::eq(&**self, &**other)
                }
            }
        )+
    };
}

impl_basic! {
    // Cow<str>

    (str, str)

    (str, String)

    #[cfg(feature = "std")]
    (str, OsStr)

    #[cfg(feature = "std")]
    (str, OsString)

    // Cow<[T]>

    ([T], [U], { T: 'a + Clone + PartialEq<U>, U })

    ([T], Vec<U>, { T: 'a + Clone + PartialEq<U>, U })

    ([T], [U; N], { T: 'a + Clone + PartialEq<U>, U, const N: usize })

    // Cow<CStr>

    #[cfg(feature = "std")]
    (CStr, CStr)

    #[cfg(feature = "std")]
    (CStr, CString)

    // Cow<OsStr>

    #[cfg(feature = "std")]
    (OsStr, OsStr)

    #[cfg(feature = "std")]
    (OsStr, OsString)

    #[cfg(feature = "std")]
    (OsStr, Path)

    #[cfg(feature = "std")]
    (OsStr, PathBuf)

    // Cow<Path>

    #[cfg(feature = "std")]
    (Path, Path)

    #[cfg(feature = "std")]
    (Path, PathBuf)

    #[cfg(feature = "std")]
    (Path, OsStr)

    #[cfg(feature = "std")]
    (Path, OsString)
}
