#[cfg(feature = "std")]
use std::{ffi::OsStr, path::Path};

use crate::{Cow, Dairy};

impl<'a, T> AsRef<T> for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.make_ref()
    }
}

macro_rules! impl_basic {
    ($(
        $(#[$attrs:meta])*
        { $Ty:ty => $As:ty }
    )+) => {
        $(
            $(#[$attrs])*
            impl<'a> AsRef<$As> for Cow<'a, $Ty> {
                #[inline]
                fn as_ref(&self) -> &$As {
                    self.make_ref().as_ref()
                }
            }
        )+
    };
}

impl_basic! {
    { str => [u8] }

    #[cfg(feature = "std")]
    { str => OsStr }

    #[cfg(feature = "std")]
    { str => Path }

    #[cfg(feature = "std")]
    { OsStr => Path }

    #[cfg(feature = "std")]
    { Path => OsStr }
}
