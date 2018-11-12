use std::fmt::{self, Debug};

use serde::ser::{Serialize, Serializer, SerializeMap, SerializeSeq};

use crate::{Value, Visit, Error};

pub fn to_serialize<V>(value: V) -> impl Serialize
where
    V: Value,
{
    struct ToSerialize<V>(V);

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

pub fn to_value<S>(serialize: S) -> impl Value
where
    S: Serialize + Debug,
{
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
