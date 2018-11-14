/*!
Convert between `val` and `serde`.

A type that implements [`val::value::Value`] can be converted into
a type that implements [`serde::Serialize`]:

```
# #[derive(Debug)] struct MyValue;
# impl val::value::Value for MyValue {
#     fn visit(&self, visit: val::value::Visit) -> Result<(), val::value::Error> {
#         visit.none()
#     }
# }
# let my_value = MyValue;
let my_serialize = val_serde::to_serialize(my_value);
```

A type that implements [`std::fmt::Debug`] and [`serde::Serialize`] can be converted into
a type that implements [`val::value::Value`]:

```
# #[derive(Debug)] struct MySerialize;
# impl serde::Serialize for MySerialize {
#     fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
#         s.serialize_none()
#     }
# }
# let my_serialize = MySerialize;
let my_value = val_serde::to_value(my_serialize);
```
*/

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod error;
mod debug;
mod to_serialize;
mod to_value;

use crate::{
    std::fmt::{
        self,
        Debug,
    },
    error::err,
};

use val::value::Value;

use serde::ser::{
    Error as SerError,
    Serialize,
    Serializer,
};

/**
Convert a `T: Value` into an `impl Serialize + Debug`.
*/
pub fn to_serialize(value: impl Value) -> impl Serialize + Debug {
    struct ToSerialize<V>(V);

    impl<V> Debug for ToSerialize<V>
    where
        V: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<V> Serialize for ToSerialize<V>
    where
        V: Value,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut visit = to_serialize::Visit::begin(serializer);
            val::visit(&self.0, &mut visit).map_err(S::Error::custom)?;

            Ok(visit.expect_ok())
        }
    }

    ToSerialize(value)
}

/**
Convert a `T: Serialize + Debug` into an `impl Value`.
*/
pub fn to_value(serialize: impl Serialize + Debug) -> impl Value {
    struct ToValue<S>(S);

    impl<S> Debug for ToValue<S>
    where
        S: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<S> Value for ToValue<S>
    where
        S: Serialize + Debug,
    {
        fn visit(&self, visit: val::value::Visit) -> Result<(), val::value::Error> {
            self.0
                .serialize(to_value::Serializer::begin(visit))
                .map_err(err("error visiting serde"))?;

            Ok(())
        }
    }

    ToValue(serialize)
}
