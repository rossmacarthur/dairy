use core::ptr::NonNull;

#[cfg(not(target_pointer_width = "64"))]
pub use medium::Extra;
#[cfg(target_pointer_width = "64")]
pub use small::Extra;

#[cfg(target_pointer_width = "64")]
mod small {
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
mod medium {
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
