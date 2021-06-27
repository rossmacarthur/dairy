use std::mem;

#[test]
fn size() {
    const WORD: usize = mem::size_of::<usize>();
    assert_eq!(mem::size_of::<std::borrow::Cow<str>>(), 4 * WORD);
    assert_eq!(mem::size_of::<dairy::Cow<str>>(), 2 * WORD);
}
