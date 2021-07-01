use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::ffi::{CStr, CString};

#[cfg(feature = "unix")]
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::{Convert, Cow};

impl<'a, 'b, T, U> PartialEq<Cow<'b, U>> for Cow<'a, T>
where
    T: ?Sized + Convert + PartialEq<U>,
    U: ?Sized + Convert,
{
    #[inline]
    fn eq(&self, other: &Cow<'b, U>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<T> Eq for Cow<'_, T> where T: ?Sized + Convert + Eq {}

impl<'a, T> PartialOrd for Cow<'a, T>
where
    T: ?Sized + Convert + PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, T>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<T> Ord for Cow<'_, T>
where
    T: ?Sized + Convert + Ord,
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

    #[cfg(feature = "unix")]
    (str, OsStr)

    #[cfg(feature = "unix")]
    (str, OsString)

    // Cow<[T]>

    ([T], [U], { T: Clone + PartialEq<U>, U })

    ([T], Vec<U>, { T: Clone + PartialEq<U>, U })

    ([T], [U; N], { T: Clone + PartialEq<U>, U, const N: usize })

    // Cow<CStr>

    #[cfg(feature = "std")]
    (CStr, CStr)

    #[cfg(feature = "std")]
    (CStr, CString)

    // Cow<OsStr>

    #[cfg(feature = "unix")]
    (OsStr, OsStr)

    #[cfg(feature = "unix")]
    (OsStr, OsString)

    #[cfg(feature = "unix")]
    (OsStr, Path)

    #[cfg(feature = "unix")]
    (OsStr, PathBuf)

    // Cow<Path>

    #[cfg(feature = "unix")]
    (Path, Path)

    #[cfg(feature = "unix")]
    (Path, PathBuf)

    #[cfg(feature = "unix")]
    (Path, OsStr)

    #[cfg(feature = "unix")]
    (Path, OsString)
}
