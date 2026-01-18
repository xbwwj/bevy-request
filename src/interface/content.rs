use bevy::prelude::*;

/// How to cope with response body.
#[derive(Component, Clone, Copy, Debug)]
pub enum GetContent
// <T: DeserializeOwned = ()>
{
    Bytes,
    Text,
    // Json(PhantomData<T>),
}

// In case we will add json and generic in future
// impl GetContent<()> {
//     pub const BYTES: Self = Self::Bytes;
//     pub const TEXT: Self = Self::Text;
// }
