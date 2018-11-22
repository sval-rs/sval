use crate::{
    std::fmt,
    value::{
        Error,
        Value,
        Stream,
    },
};

impl Value for () {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.none()
    }
}

impl<T> Value for Option<T>
where
    T: Value,
{
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        match self {
            Some(v) => v.stream(stream),
            None => stream.none(),
        }
    }
}

impl<T> Value for [T]
where
    T: Value,
{
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.seq_begin(Some(self.len()))?;

        for v in self {
            stream.seq_elem()?.any(v)?;
        }

        stream.seq_end()
    }
}

impl Value for u8 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(*self as u64)
    }
}

impl Value for u16 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(*self as u64)
    }
}

impl Value for u32 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(*self as u64)
    }
}

impl Value for u64 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(*self)
    }
}

impl Value for i8 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(*self as i64)
    }
}

impl Value for i16 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(*self as i64)
    }
}

impl Value for i32 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(*self as i64)
    }
}

impl Value for i64 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(*self)
    }
}

impl Value for u128 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u128(*self)
    }
}

impl Value for i128 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i128(*self)
    }
}

impl Value for f32 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.f64(*self as f64)
    }
}

impl Value for f64 {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.f64(*self)
    }
}

impl Value for bool {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.bool(*self)
    }
}

impl Value for str {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.str(self)
    }
}

impl<'a> Value for fmt::Arguments<'a> {
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.fmt(*self)
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::{
        collections::{
            BTreeMap,
            HashMap,
        },
        hash::Hash,
        string::String,
        vec::Vec,
    };

    impl Value for String {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            stream.str(&*self)
        }
    }

    impl<T> Value for Vec<T>
    where
        T: Value,
    {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            self.as_slice().stream(stream)
        }
    }

    impl<K, V> Value for BTreeMap<K, V>
    where
        K: Eq + Value,
        V: Value,
    {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key()?.any(k)?;
                stream.map_value()?.any(v)?;
            }

            stream.map_end()
        }
    }

    impl<K, V> Value for HashMap<K, V>
    where
        K: Hash + Eq + Value,
        V: Value,
    {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key()?.any(k)?;
                stream.map_value()?.any(v)?;
            }

            stream.map_end()
        }
    }
}
