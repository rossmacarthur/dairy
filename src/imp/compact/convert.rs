//! Convert a [`ToOwned`] type to and from parts.

use core::mem::ManuallyDrop;
use core::ptr;
use core::ptr::NonNull;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(all(feature = "std", unix))]
use std::os::unix::ffi::{OsStrExt, OsStringExt};
#[cfg(all(feature = "std", target_os = "wasi"))]
use std::os::wasi::ffi::{OsStrExt, OsStringExt};

use super::extent::Extent;

/// Whether or not this extra data describes an owned type.
pub trait IsOwned {
    fn is_owned(&self) -> bool;
}

/// Convert a [`ToOwned`] type to and from parts.
pub unsafe trait Convert: ToOwned {
    /// The pointer type that will be used in the better `Cow`.
    type Ptr;

    /// Any extra data that is required to reconstruct an owned or borrowed
    /// variant of this type. For example: length and capacity.
    type Extent: Copy + Default + IsOwned;

    /// Convert a borrowed version of self into parts.
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extent);

    /// Convert an owned version of self into parts.
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extent);

    /// Returns a pointer to self constructed from parts.
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> *const Self;

    /// Returns an owned version of self constructed from parts.
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> Self::Owned;
}

impl IsOwned for bool {
    #[inline]
    fn is_owned(&self) -> bool {
        *self
    }
}

impl IsOwned for Extent {
    #[inline]
    fn is_owned(&self) -> bool {
        self.capacity() != 0
    }
}

unsafe impl Convert for str {
    type Ptr = u8;
    type Extent = Extent;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extent) {
        unsafe { Extent::borrowed(b.as_ptr(), b.len()) }
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extent) {
        let mut o = ManuallyDrop::new(o);
        unsafe { Extent::owned(o.as_mut_ptr(), o.len(), o.capacity()) }
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> *const Self {
        ptr::slice_from_raw_parts(ptr.as_ptr(), extra.len()) as *const Self
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> Self::Owned {
        unsafe { String::from_raw_parts(ptr.as_ptr(), extra.len(), extra.capacity()) }
    }
}

unsafe impl<T: Clone> Convert for [T] {
    type Ptr = T;
    type Extent = Extent;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extent) {
        unsafe { Extent::borrowed(b.as_ptr(), b.len()) }
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extent) {
        let mut o = ManuallyDrop::new(o);
        unsafe { Extent::owned(o.as_mut_ptr(), o.len(), o.capacity()) }
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> *const Self {
        ptr::slice_from_raw_parts(ptr.as_ptr(), extra.len())
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> Self::Owned {
        unsafe { Vec::from_raw_parts(ptr.as_ptr(), extra.len(), extra.capacity()) }
    }
}

#[cfg(feature = "std")]
unsafe impl Convert for std::ffi::CStr {
    type Ptr = std::os::raw::c_char;
    type Extent = bool;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extent) {
        let ptr = unsafe { NonNull::new_unchecked(b.as_ptr() as *mut Self::Ptr) };
        (ptr, false)
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extent) {
        let ptr = unsafe { NonNull::new_unchecked(o.into_raw()) };
        (ptr, true)
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, _: Self::Extent) -> *const Self {
        unsafe { Self::from_ptr(ptr.as_ptr()) as *const Self }
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, _: Self::Extent) -> Self::Owned {
        unsafe { std::ffi::CString::from_raw(ptr.as_ptr()) }
    }
}

#[cfg(all(feature = "std", os_str_ext))]
unsafe impl Convert for std::ffi::OsStr {
    type Ptr = u8;
    type Extent = Extent;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extent) {
        <[u8]>::unmake_borrowed(b.as_bytes())
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extent) {
        <[u8]>::unmake_owned(o.into_vec())
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> *const Self {
        unsafe { <[u8]>::make_ptr(ptr, extra) as *const Self }
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> Self::Owned {
        unsafe { std::ffi::OsString::from_vec(<[u8]>::make_owned(ptr, extra)) }
    }
}

#[cfg(all(feature = "std", os_str_ext))]
unsafe impl Convert for std::path::Path {
    type Ptr = u8;
    type Extent = Extent;

    #[inline]
    fn unmake_borrowed(b: &Self) -> (NonNull<Self::Ptr>, Self::Extent) {
        std::ffi::OsStr::unmake_borrowed(b.as_os_str())
    }

    #[inline]
    fn unmake_owned(o: Self::Owned) -> (NonNull<Self::Ptr>, Self::Extent) {
        std::ffi::OsStr::unmake_owned(o.into_os_string())
    }

    #[inline]
    unsafe fn make_ptr(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> *const Self {
        unsafe { std::ffi::OsStr::make_ptr(ptr, extra) as *const Self }
    }

    #[inline]
    unsafe fn make_owned(ptr: NonNull<Self::Ptr>, extra: Self::Extent) -> Self::Owned {
        unsafe { std::path::PathBuf::from(std::ffi::OsStr::make_owned(ptr, extra)) }
    }
}
