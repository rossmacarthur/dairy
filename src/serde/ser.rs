use serde::{Serialize, Serializer};

use crate::{Cow, Dairy};

impl<'a, T> Serialize for Cow<'a, T>
where
    T: ?Sized + Dairy + Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}
