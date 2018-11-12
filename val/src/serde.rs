use std::fmt::{self, Debug};

use serde::ser::{Serialize, Serializer, SerializeMap, SerializeSeq};

use crate::{value, visit};

/**
Convert a `T: Value` into an `impl Serialize + Debug`.
*/
pub fn to_serialize(value: impl value::Value) -> impl Serialize + Debug {
    use self::value::Value;

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
            unimplemented!()
        }
    }

    ToSerialize(value)
}

/**
Convert a `T: Serialize + Debug` into an `impl Value`.
*/
pub fn to_value(serialize: impl Serialize + Debug) -> impl value::Value {
    use self::value::{Value, Visit, Error};

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
        fn visit(&self, visit: Visit) -> Result<(), Error> {
            unimplemented!()
        }
    }

    ToValue(serialize)
}
