/*!
Integration between `sval` and `serde`.

A type that implements [`sval::value::Value`] can be converted into
a type that implements [`serde::Serialize`]:

```
# struct MyValue;
# impl sval::value::Value for MyValue {
#     fn stream(&self, stream: &mut sval::value::Stream) -> Result<(), sval::value::Error> {
#         unimplemented!()
#     }
# }
# let my_value = MyValue;
let my_serialize = sval::serde::to_serialize(my_value);
```

A type that implements [`serde::Serialize`] can be converted into
a type that implements [`sval::value::Value`]:

```
# struct MySerialize;
# impl serde_lib::Serialize for MySerialize {
#     fn serialize<S: serde_lib::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
#         unimplemented!()
#     }
# }
# let my_serialize = MySerialize;
let my_value = sval::serde::to_value(my_serialize);
```
*/

mod error;

mod to_serialize;
mod to_value;

use crate::{
    Stream,
    Value,
    Error,
};

use serde_lib::ser::{
    Serialize,
    Serializer,
};

/**
Convert a [`Value`] into a [`Serialize`].
*/
pub fn to_serialize(value: impl Value) -> impl Serialize {
    to_serialize::ToSerialize(value)
}

/**
Serialize a [`Value`] using the given [`Serializer`].
*/
pub fn serialize<S>(value: impl Value, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    to_serialize(value).serialize(serializer)
}

/**
Convert a [`Serialize`] into a [`Value`].
*/
pub fn to_value(value: impl Serialize) -> impl Value {
    to_value::ToValue(value)
}

/**
Stream a [`Serialize`] using the given [`Stream`].
*/
pub fn stream(value: impl Serialize, stream: impl Stream) -> Result<(), Error> {
    crate::stream(to_value(value), stream)
}
