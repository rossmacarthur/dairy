pub enum Cow<'a, T: ?Sized + 'a>
where
    T: ToOwned,
{
    /// Borrowed data.
    Borrowed(&'a T),

    /// Owned data.
    Owned(<T as ToOwned>::Owned),
}
