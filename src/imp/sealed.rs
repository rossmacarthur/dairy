//! Restrict [`Dairy`](crate::Dairy) implementations to this crate.

pub trait Sealed {}

impl Sealed for str {}

impl<T: Clone> Sealed for [T] {}

#[cfg(feature = "std")]
impl Sealed for std::ffi::CStr {}

#[cfg(feature = "std")]
impl Sealed for std::ffi::OsStr {}

#[cfg(feature = "std")]
impl Sealed for std::path::Path {}
