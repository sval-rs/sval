/*!
Integration between `sval` and `serde`.

A type that implements [`sval::value::Value`] can be converted into
a type that implements [`serde::Serialize`]:

```
# struct MyValue;
# impl sval::value::Value for MyValue {
#     fn visit(&self, visit: sval::value::Visit) -> Result<(), sval::value::Error> {
#         visit.none()
#     }
# }
# let my_value = MyValue;
let my_serialize = sval::serde::to_serialize(my_value);
```

A type that implements [`serde::Serialize`] can be converted into
a type that implements [`sval::value::Value`]:

```
# struct MySerialize;
# impl serde::Serialize for MySerialize {
#     fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
#         s.serialize_none()
#     }
# }
# let my_serialize = MySerialize;
let my_value = sval::serde::to_value(my_serialize);
```
*/

mod to_serialize;
mod to_value;

use crate::{
    Error,
    Stream,
    Value,
};

use serde::ser::{
    self,
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

fn err<E>(msg: &'static str) -> impl FnOnce(E) -> crate::Error
where
    E: ser::Error,
{
    #[cfg(feature = "std")]
    {
        let _ = msg;
        move |err| crate::Error::custom(err)
    }

    #[cfg(not(feature = "std"))]
    {
        move |_| crate::Error::msg(msg)
    }
}
