//! A better `Cow` implementation.

mod convert;
mod extent;

use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;

use crate::imp;
use crate::imp::Cow as _;

use self::convert::{Convert, IsOwned};

/// A compact copy-on-write pointer.
pub struct Cow<'a, T>
where
    T: ?Sized + Convert,
{
    /// Pointer to the data.
    ptr: NonNull<T::Ptr>,

    /// Any extra data that is required to reconstruct an owned or borrowed
    /// variant of this type. For example: length and capacity.
    extent: T::Extent,

    /// For the lifetime.
    marker: PhantomData<&'a T>,
}

impl<'a, T> imp::Cow<'a, T> for Cow<'a, T>
where
    T: ?Sized + Convert,
{
    #[inline]
    fn borrowed(b: &'a T) -> Self {
        let (ptr, extent) = T::unmake_borrowed(b);
        Self {
            ptr,
            extent,
            marker: PhantomData,
        }
    }

    #[inline]
    fn owned(o: T::Owned) -> Self {
        let (ptr, extent) = T::unmake_owned(o);
        Self {
            ptr,
            extent,
            marker: PhantomData,
        }
    }

    #[inline]
    fn is_borrowed(&self) -> bool {
        !self.extent.is_owned()
    }

    #[inline]
    fn is_owned(&self) -> bool {
        self.extent.is_owned()
    }

    #[inline]
    fn make_ref(&self) -> &T {
        // SAFETY: This is valid for both owned and borrowed variants.
        unsafe { &*T::make_ptr(self.ptr, self.extent) }
    }

    #[inline]
    fn into_owned(self) -> T::Owned {
        if self.is_owned() {
            let cow = ManuallyDrop::new(self);
            unsafe { T::make_owned(cow.ptr, cow.extent) }
        } else {
            self.make_ref().to_owned()
        }
    }

    #[inline]
    fn apply<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T::Owned),
    {
        let mut o = if self.is_owned() {
            // SAFETY:  This is safe because we temporarily set the `extent` to
            // its default value which should encode a "borrowed" version.
            // Therefore, if `f` had to panic, no double drop would occur.
            let o = unsafe { T::make_owned(self.ptr, self.extent) };
            self.extent = T::Extent::default();
            o
        } else {
            self.make_ref().to_owned()
        };
        f(&mut o);
        let (ptr, extent) = T::unmake_owned(o);
        self.ptr = ptr;
        self.extent = extent;
    }
}

impl<T> Clone for Cow<'_, T>
where
    T: ?Sized + Convert,
{
    #[inline]
    fn clone(&self) -> Self {
        if self.is_owned() {
            Self::owned(self.make_ref().to_owned())
        } else {
            Self { ..*self }
        }
    }
}

impl<T> Drop for Cow<'_, T>
where
    T: ?Sized + Convert,
{
    #[inline]
    fn drop(&mut self) {
        if self.is_owned() {
            unsafe { T::make_owned(self.ptr, self.extent) };
        }
    }
}
