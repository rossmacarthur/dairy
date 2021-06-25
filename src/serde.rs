#![cfg(feature = "serde")]

use alloc::borrow::ToOwned;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Cow;

impl<'de, 'a, T: ?Sized> Deserialize<'de> for Cow<'a, T>
where
    T: ToOwned,
    T::Owned: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::Owned::deserialize(deserializer).map(Cow::Owned)
    }
}

impl<'a, T: ?Sized> Serialize for Cow<'a, T>
where
    T: Serialize + ToOwned,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}
