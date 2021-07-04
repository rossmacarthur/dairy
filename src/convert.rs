use core::mem::ManuallyDrop;
use core::ptr;
use core::ptr::NonNull;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};
#[cfg(feature = "unix")]
use std::{
    ffi::{OsStr, OsString},
    os::unix::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};

use crate::convert::extra::Extra;

mod private {
    pub trait Sealed {}

    impl Sealed for str {}

    impl<T: Clone> Sealed for [T] {}

    #[cfg(feature = "std")]
    impl Sealed for std::ffi::CStr {}

    #[cfg(feature = "unix")]
    impl Sealed for std::ffi::OsStr {}

    #[cfg(feature = "unix")]
    impl Sealed for std::path::Path {}
}

#[cfg(target_pointer_width = "64")]
mod extra {
    use super::*;

    const SHIFT: u32 = usize::BITS / 2;
    const LOWER: usize = usize::MAX >> SHIFT;
    const UPPER: usize = !LOWER;

    #[derive(Clone, Copy)]
    pub struct Extra(usize);

    impl Extra {
        #[inline]
        pub unsafe fn borrowed<T>(ptr: *const T, len: usize) -> (NonNull<T>, Self) {
            let ptr = unsafe { NonNull::new_unchecked(ptr as *mut T) };
            (ptr, Extra(len))
        }

        #[inline]
        pub unsafe fn owned<T>(ptr: *mut T, len: usize, cap: usize) -> (NonNull<T>, Self) {
            assert!((cap & LOWER) == cap, "capacity out of bounds");
            let extra = (cap << SHIFT) | len;
            let ptr = unsafe { NonNull::new_unchecked(ptr) };
            (ptr, Extra(extra))
        }

        #[inline]
        pub const fn len(&self) -> usize {
            self.0 & LOWER
        }

        #[inline]
        pub const fn capacity(&self) -> usize {
            (self.0 & UPPER) >> SHIFT
        }
    }
}

#[cfg(not(target_pointer_width = "64"))]
mod extra {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct Extra {
        len: usize,
        cap: usize,
    }

    impl Extra {
        #[inline]
        pub unsafe fn borrowed<T>(ptr: *const T, len: usize) -> (NonNull<T>, Self) {
            let ptr = unsafe { NonNull::new_unchecked(ptr as *mut T) };
            (ptr, Extra { len, cap: 0 })
        }

        #[inline]
        pub unsafe fn owned<T>(ptr: *mut T, len: usize, cap: usize) -> (NonNull<T>, Self) {
            let ptr = unsafe { NonNull::new_unchecked(ptr) };
            (ptr, Extra { len, cap })
        }

        #[inline]
        pub const fn len(&self) -> usize {
            self.len
        }

        #[inline]
        pub const fn capacity(&self) -> usize {
            self.cap
        }
    }
}

/// Whether or not this extra data describes and owned type.
pub trait IsOwned {
    fn is_owned(&self) -> bool;
}

/// Convert a [`ToOwned`] type to and from parts.
pub unsafe trait Convert: ToOwned + private::Sealed {
    /// The pointer type that will be used in [`Cow`][crate::Cow].
    type Ptr;

    /// Any extra data that is required to reconstruct an owned or borrowed
    /// variant of this type. For example: length and capacity.
    type Extra: IsOwned + Copy;

    /// Convert a borrowed version of self into parts.
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extra);

    /// Convert an owned version of self into parts.
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extra);

    /// Returns a pointer to self constructed from parts.
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> *const Self;

    /// Returns an owned version of self constructed from parts.
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> Self::Owned;
}

impl IsOwned for bool {
    #[inline]
    fn is_owned(&self) -> bool {
        *self
    }
}

impl IsOwned for Extra {
    #[inline]
    fn is_owned(&self) -> bool {
        self.capacity() != 0
    }
}

unsafe impl Convert for str {
    type Ptr = u8;
    type Extra = Extra;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extra) {
        unsafe { Extra::borrowed(b.as_ptr(), b.len()) }
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extra) {
        let mut o = ManuallyDrop::new(o);
        unsafe { Extra::owned(o.as_mut_ptr(), o.len(), o.capacity()) }
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> *const Self {
        ptr::slice_from_raw_parts(ptr.as_ptr(), extra.len()) as *const Self
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> Self::Owned {
        unsafe { String::from_raw_parts(ptr.as_ptr(), extra.len(), extra.capacity()) }
    }
}

unsafe impl<T: Clone> Convert for [T] {
    type Ptr = T;
    type Extra = Extra;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extra) {
        unsafe { Extra::borrowed(b.as_ptr(), b.len()) }
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extra) {
        let mut o = ManuallyDrop::new(o);
        unsafe { Extra::owned(o.as_mut_ptr(), o.len(), o.capacity()) }
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> *const Self {
        ptr::slice_from_raw_parts(ptr.as_ptr(), extra.len())
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> Self::Owned {
        unsafe { Vec::from_raw_parts(ptr.as_ptr(), extra.len(), extra.capacity()) }
    }
}

#[cfg(feature = "std")]
unsafe impl Convert for CStr {
    type Ptr = c_char;
    type Extra = bool;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extra) {
        let ptr = unsafe { NonNull::new_unchecked(b.as_ptr() as *mut Self::Ptr) };
        (ptr, false)
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extra) {
        let ptr = unsafe { NonNull::new_unchecked(o.into_raw()) };
        (ptr, true)
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, _: Self::Extra) -> *const Self {
        unsafe { Self::from_ptr(ptr.as_ptr()) as *const Self }
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, _: Self::Extra) -> Self::Owned {
        unsafe { CString::from_raw(ptr.as_ptr()) }
    }
}

#[cfg(feature = "unix")]
unsafe impl Convert for OsStr {
    type Ptr = u8;
    type Extra = Extra;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extra) {
        <[u8]>::unmake_borrowed(b.as_bytes())
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extra) {
        <[u8]>::unmake_owned(o.into_vec())
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> *const Self {
        unsafe { <[u8]>::make_ptr(ptr, extra) as *const Self }
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> Self::Owned {
        unsafe { OsString::from_vec(<[u8]>::make_owned(ptr, extra)) }
    }
}

#[cfg(feature = "unix")]
unsafe impl Convert for Path {
    type Ptr = u8;
    type Extra = Extra;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extra) {
        OsStr::unmake_borrowed(b.as_os_str())
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extra) {
        OsStr::unmake_owned(o.into_os_string())
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> *const Self {
        unsafe { OsStr::make_ptr(ptr, extra) as *const Self }
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extra) -> Self::Owned {
        unsafe { PathBuf::from(OsStr::make_owned(ptr, extra)) }
    }
}
