#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc as std;
#[cfg(feature = "std")]
extern crate std;

mod from;

use std::borrow::ToOwned;

pub enum Cow<'a, T: ?Sized + 'a>
where
    T: ToOwned,
{
    /// Borrowed data.
    Borrowed(&'a T),

    /// Owned data.
    Owned(<T as ToOwned>::Owned),
}
