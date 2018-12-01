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

impl<T, U> Value for (T, U)
where
    T: Value,
    U: Value,
{
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.seq_begin(Some(2))?;

        stream.seq_elem(&self.0)?;
        stream.seq_elem(&self.1)?;

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

impl Value for char {
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        stream.char(*self)
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
        boxed::Box,
        collections::{
            BTreeMap,
            HashMap,
        },
        hash::{
            BuildHasher,
            Hash,
        },
        rc::Rc,
        string::String,
        sync::Arc,
        vec::Vec,
    };

    impl<T: ?Sized> Value for Box<T>
    where
        T: Value,
    {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            (**self).stream(stream)
        }
    }

    // FIXME: It'd be a shame not to optimize `Value::to_owned` for `Arc`
    impl<T: ?Sized> Value for Arc<T>
    where
        T: Value,
    {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            (**self).stream(stream)
        }
    }

    impl<T: ?Sized> Value for Rc<T>
    where
        T: Value,
    {
        fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
            (**self).stream(stream)
        }
    }

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

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    mod std_support {
        use crate::{
            std::{
                boxed::Box,
                collections::{
                    BTreeMap,
                    HashMap,
                },
                rc::Rc,
                string::String,
                sync::Arc,
                vec::Vec,
            },
            test::{
                self,
                Kind,
            },
        };

        #[test]
        fn stream_unsigned() {
            assert_eq!(vec![Kind::Unsigned(1)], test::tokens(1u8));

            assert_eq!(vec![Kind::Unsigned(1)], test::tokens(1u16));

            assert_eq!(vec![Kind::Unsigned(1)], test::tokens(1u32));

            assert_eq!(vec![Kind::Unsigned(1)], test::tokens(1u64));

            assert_eq!(vec![Kind::BigUnsigned(1)], test::tokens(1u128));
        }

        #[test]
        fn stream_signed() {
            assert_eq!(vec![Kind::Signed(1)], test::tokens(1i8));

            assert_eq!(vec![Kind::Signed(1)], test::tokens(1i16));

            assert_eq!(vec![Kind::Signed(1)], test::tokens(1i32));

            assert_eq!(vec![Kind::Signed(1)], test::tokens(1i64));

            assert_eq!(vec![Kind::BigSigned(1)], test::tokens(1i128));
        }

        #[test]
        fn stream_float() {
            assert_eq!(vec![Kind::Float(1.0)], test::tokens(1f32));

            assert_eq!(vec![Kind::Float(1.0)], test::tokens(1f64));
        }

        #[test]
        fn stream_bool() {
            assert_eq!(vec![Kind::Bool(false)], test::tokens(false));
        }

        #[test]
        fn stream_str() {
            assert_eq!(vec![Kind::Str("a string".into())], test::tokens("a string"));

            assert_eq!(
                vec![Kind::Str("a string".into())],
                test::tokens(String::from("a string"))
            );

            assert_eq!(
                vec![Kind::Str("a format 1".into())],
                test::tokens(format_args!("a format {}", 1))
            );

            assert_eq!(vec![Kind::Char('a')], test::tokens('a'));
        }

        #[test]
        fn stream_option() {
            assert_eq!(vec![Kind::None], test::tokens(Option::None::<i32>));

            assert_eq!(vec![Kind::Signed(1)], test::tokens(Some(1)));
        }

        #[test]
        fn stream_vec() {
            let v = test::tokens(&[] as &[i32]);
            assert_eq!(vec![Kind::SeqBegin(Some(0)), Kind::SeqEnd], v);

            let v = test::tokens(&[1, 2, 3] as &[i32]);
            assert_eq!(
                vec![
                    Kind::SeqBegin(Some(3)),
                    Kind::SeqElem,
                    Kind::Signed(1),
                    Kind::SeqElem,
                    Kind::Signed(2),
                    Kind::SeqElem,
                    Kind::Signed(3),
                    Kind::SeqEnd,
                ],
                v
            );

            let v = test::tokens(Vec::<i32>::new());
            assert_eq!(vec![Kind::SeqBegin(Some(0)), Kind::SeqEnd], v);

            let v = test::tokens(vec![1, 2, 3]);
            assert_eq!(
                vec![
                    Kind::SeqBegin(Some(3)),
                    Kind::SeqElem,
                    Kind::Signed(1),
                    Kind::SeqElem,
                    Kind::Signed(2),
                    Kind::SeqElem,
                    Kind::Signed(3),
                    Kind::SeqEnd,
                ],
                v
            );
        }

        #[test]
        fn stream_map() {
            let v = test::tokens(HashMap::<i32, i32>::new());
            assert_eq!(vec![Kind::MapBegin(Some(0)), Kind::MapEnd], v);

            let v = test::tokens(BTreeMap::<i32, i32>::new());
            assert_eq!(vec![Kind::MapBegin(Some(0)), Kind::MapEnd], v);

            let v = test::tokens({
                let mut map = BTreeMap::new();
                map.insert(1, 11);
                map.insert(2, 22);
                map
            });
            assert_eq!(
                vec![
                    Kind::MapBegin(Some(2)),
                    Kind::MapKey,
                    Kind::Signed(1),
                    Kind::MapValue,
                    Kind::Signed(11),
                    Kind::MapKey,
                    Kind::Signed(2),
                    Kind::MapValue,
                    Kind::Signed(22),
                    Kind::MapEnd,
                ],
                v
            );
        }

        #[test]
        fn stream_box() {
            assert_eq!(vec![Kind::Signed(1)], test::tokens(Box::new(1i8)));
        }

        #[test]
        fn stream_rc() {
            assert_eq!(vec![Kind::Signed(1)], test::tokens(Rc::new(1i8)));

            assert_eq!(vec![Kind::Signed(1)], test::tokens(Arc::new(1i8)));
        }
    }
}
