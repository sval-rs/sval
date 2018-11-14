use crate::{
    std::fmt,
    value::{Error, Value, Visit},
    visit,
};

impl Value for () {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.none()
    }
}

impl<T> Value for Option<T>
where
    T: Value,
{
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        match self {
            Some(v) => v.visit(visit),
            None => visit.none(),
        }
    }
}

impl<T> Value for [T]
where
    T: Value,
{
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        let mut seq = visit.seq(Some(self.len()))?;

        for v in self {
            seq.elem(v)?;
        }

        seq.end()
    }
}

impl Value for u8 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.u64(*self as u64)
    }
}

impl Value for u16 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.u64(*self as u64)
    }
}

impl Value for u32 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.u64(*self as u64)
    }
}

impl Value for u64 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.u64(*self)
    }
}

impl Value for i8 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.i64(*self as i64)
    }
}

impl Value for i16 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.i64(*self as i64)
    }
}

impl Value for i32 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.i64(*self as i64)
    }
}

impl Value for i64 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.i64(*self)
    }
}

impl Value for u128 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.u128(*self)
    }
}

impl Value for i128 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.i128(*self)
    }
}

impl Value for f32 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.f64(*self as f64)
    }
}

impl Value for f64 {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.f64(*self)
    }
}

impl Value for bool {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.bool(*self)
    }
}

impl Value for str {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.str(self)
    }
}

impl<'a> Value for fmt::Arguments<'a> {
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        visit.fmt(self)
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::{
        boxed::Box,
        collections::{BTreeMap, HashMap},
        hash::Hash,
        string::{String, ToString},
        vec::Vec,
    };

    impl Value for String {
        fn visit(&self, visit: Visit) -> Result<(), Error> {
            visit.str(&*self)
        }
    }

    impl<T> Value for Vec<T>
    where
        T: Value,
    {
        fn visit(&self, visit: Visit) -> Result<(), Error> {
            self.as_slice().visit(visit)
        }
    }

    impl<K, V> Value for BTreeMap<K, V>
    where
        K: Eq + Value,
        V: Value,
    {
        fn visit(&self, visit: Visit) -> Result<(), Error> {
            let mut map = visit.map(Some(self.len()))?;

            for (k, v) in self {
                map.entry(k, v)?;
            }

            map.end()
        }
    }

    impl<K, V> Value for HashMap<K, V>
    where
        K: Hash + Eq + Value,
        V: Value,
    {
        fn visit(&self, visit: Visit) -> Result<(), Error> {
            let mut map = visit.map(Some(self.len()))?;

            for (k, v) in self {
                map.entry(k, v)?;
            }

            map.end()
        }
    }
}
