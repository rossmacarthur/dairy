#![cfg(feature = "serde")]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Cow, Dairy};

impl<'de, 'a, T> Deserialize<'de> for Cow<'a, T>
where
    T: ?Sized + Dairy<'a>,
    T::Owned: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::Owned::deserialize(deserializer).map(Cow::owned)
    }
}

impl<'a, T> Serialize for Cow<'a, T>
where
    T: ?Sized + Dairy<'a> + Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}
