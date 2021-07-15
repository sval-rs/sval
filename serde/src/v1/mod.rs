mod error;

mod to_serialize;
mod to_value;

use crate::{
    Error,
    Stream,
    Value,
};

use serde1_lib::ser::{
    Serialize,
    Serializer,
};

pub use self::{
    to_serialize::ToSerialize,
    to_value::ToValue,
};

pub fn to_serialize<V>(value: V) -> ToSerialize<V>
where
    V: Value,
{
    ToSerialize(value)
}

pub fn serialize<S>(serializer: S, value: impl Value) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    to_serialize(value).serialize(serializer)
}

pub fn to_value<S>(value: S) -> ToValue<S>
where
    S: Serialize,
{
    ToValue(value)
}

pub fn stream_owned<'a, S>(stream: impl Stream<'a>, value: impl Serialize) -> Result<(), Error> {
    crate::stream_owned(stream, to_value(value))
}

#[doc(hidden)]
#[cfg(feature = "std")]
pub const IS_NO_STD: bool = false;

#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub const IS_NO_STD: bool = true;
