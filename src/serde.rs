#![cfg(feature = "serde")]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Convert, Cow};

impl<'de, 'a, T: ?Sized> Deserialize<'de> for Cow<'a, T>
where
    T: Convert,
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

impl<'a, T: ?Sized> Serialize for Cow<'a, T>
where
    T: Serialize + Convert,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}
