/*!
Convert between `val` and `serde`.

A type that implements [`value::Value`] can be converted into
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
a type that implements [`value::Value`]:

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

use crate::std::fmt::{
    self,
    Debug,
};

mod debug;
mod to_serialize;
mod to_value;

use serde::ser::{
    self,
    Error as SerError,
    Serialize,
    Serializer,
};

use val::value::{
    self,
    Value,
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
        V: value::Value,
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
        fn visit(&self, visit: value::Visit) -> Result<(), value::Error> {
            self.0
                .serialize(to_value::Serializer::begin(visit))
                .map_err(err("error visiting serde"))?;

            Ok(())
        }
    }

    ToValue(serialize)
}

/**
An error encountered during serialization.
*/
struct Error(value::Error);

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Error(value::Error::msg("error during formatting"))
    }
}

impl From<value::Error> for Error {
    fn from(err: value::Error) -> Self {
        Error(err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

fn err<E>(msg: &'static str) -> impl FnOnce(E) -> val::Error
where
    E: ser::Error,
{
    #[cfg(feature = "std")]
    {
        let _ = msg;
        move |err| val::Error::from(err)
    }

    #[cfg(not(feature = "std"))]
    {
        move |_| val::Error::msg(msg)
    }
}

#[cfg(not(feature = "std"))]
mod core_support {
    use super::*;

    impl ser::Error for Error {
        fn custom<E>(_: E) -> Self
        where
            E: fmt::Display,
        {
            Error(value::Error::msg("serialization error"))
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error;

    impl error::Error for Error {
        fn cause(&self) -> Option<&dyn error::Error> {
            None
        }

        fn description(&self) -> &str {
            "serialization error"
        }
    }

    impl ser::Error for Error {
        fn custom<E>(e: E) -> Self
        where
            E: fmt::Display,
        {
            Error(value::Error::custom(e))
        }
    }
}
