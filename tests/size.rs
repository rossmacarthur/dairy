use std::ffi::OsStr;
use std::mem;

#[test]
fn size_str() {
    const WORD: usize = mem::size_of::<usize>();

    assert_eq!(mem::size_of::<std::borrow::Cow<str>>(), 4 * WORD);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(mem::size_of::<dairy::Cow<str>>(), 2 * WORD);

    #[cfg(not(target_pointer_width = "64"))]
    assert_eq!(mem::size_of::<dairy::Cow<str>>(), 3 * WORD);
}

#[test]
fn size_os_str() {
    const WORD: usize = mem::size_of::<usize>();

    assert_eq!(mem::size_of::<std::borrow::Cow<OsStr>>(), 4 * WORD);

    #[cfg(all(target_pointer_width = "64", os_str_ext))]
    assert_eq!(mem::size_of::<dairy::Cow<OsStr>>(), 2 * WORD);

    #[cfg(all(not(target_pointer_width = "64"), os_str_ext))]
    assert_eq!(mem::size_of::<dairy::Cow<OsStr>>(), 3 * WORD);

    #[cfg(not(os_str_ext))]
    assert_eq!(mem::size_of::<dairy::Cow<OsStr>>(), 4 * WORD);
}
