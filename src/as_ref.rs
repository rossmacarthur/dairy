#[cfg(feature = "unix")]
use std::{ffi::OsStr, path::Path};

use crate::{Convert, Cow};

impl<T> AsRef<T> for Cow<'_, T>
where
    T: ?Sized + Convert,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.make_borrowed()
    }
}

macro_rules! impl_basic {
    ($(
        $(#[$attrs:meta])*
        ($Ty:ty as $As:ty)
    )+) => {
        $(
            $(#[$attrs])*
            impl<'a> AsRef<$As> for Cow<'a, $Ty> {
                #[inline]
                fn as_ref(&self) -> &$As {
                    self.make_borrowed().as_ref()
                }
            }
        )+
    };
}

impl_basic! {
    (str as [u8])

    #[cfg(feature = "unix")]
    (str as OsStr)

    #[cfg(feature = "unix")]
    (str as Path)

    #[cfg(feature = "unix")]
    (OsStr as Path)

    #[cfg(feature = "unix")]
    (Path as OsStr)
}
