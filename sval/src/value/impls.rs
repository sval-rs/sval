use crate::{
    std::fmt,
    value::{
        Error,
        Stream,
        Value,
    },
};

impl Value for () {
    #[inline]
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
            stream.seq_elem(v)?;
        }

        stream.seq_end()
    }
}

impl Value for u8 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(u64::from(*self))
    }
}

impl Value for u16 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(u64::from(*self))
    }
}

impl Value for u32 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(u64::from(*self))
    }
}

impl Value for u64 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u64(*self)
    }
}

impl Value for i8 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(i64::from(*self))
    }
}

impl Value for i16 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(i64::from(*self))
    }
}

impl Value for i32 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(i64::from(*self))
    }
}

impl Value for i64 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i64(*self)
    }
}

impl Value for u128 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.u128(*self)
    }
}

impl Value for i128 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.i128(*self)
    }
}

impl Value for f32 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.f64(f64::from(*self))
    }
}

impl Value for f64 {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.f64(*self)
    }
}

impl Value for bool {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.bool(*self)
    }
}

impl Value for str {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.str(self)
    }
}

impl<'a> Value for fmt::Arguments<'a> {
    #[inline]
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
        hash::{
            BuildHasher,
            Hash,
        },
        string::String,
        vec::Vec,
    };

    impl Value for String {
        #[inline]
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
                stream.map_key(k)?;
                stream.map_value(v)?;
            }

            stream.map_end()
        }
    }

    impl<K, V, H> Value for HashMap<K, V, H>
    where
        K: Hash + Eq + Value,
        V: Value,
        H: BuildHasher,
    {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key(k)?;
                stream.map_value(v)?;
            }

            stream.map_end()
        }
    }
}
