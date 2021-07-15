#![cfg(test)]

#[macro_use]
extern crate sval;

#[macro_use]
extern crate serde_derive;

sval_if_alloc! {
    if #[cfg(feature = "alloc")]
    {
        // NOTE: We can't run tests when `alloc` is enabled
    }
    else
    {
        use sval::{
            value::{
                self,
                Value,
            },
        };

        #[derive(Value)]
        struct Struct<'a> {
            a: i32,
            b: i32,
            #[sval(rename = "renamed")]
            c: Nested<'a>,
        }

        #[derive(Value)]
        struct Nested<'a> {
            a: i32,
            b: &'a str,
        }

        #[derive(Serialize, Value)]
        #[sval(derive_from = "serde")]
        enum Tagged {
            Unit,
            NewType(i32),
            Tuple(i32, i32),
            Struct { a: i32, b: i32 },
        }

        #[derive(Serialize, Value)]
        #[sval(derive_from = "serde")]
        struct NewType(i32);

        struct Anonymous;

        impl Value for Anonymous {
            fn stream<'s, 'v>(&'v self, mut stream: value::Stream<'s, 'v>) -> value::Result {
                stream.map_begin(Default::default())?;

                stream.map_key_begin()?.i64(1)?;

                stream.map_value_begin()?.map_begin(Default::default())?;

                stream.map_key(&2)?;

                stream.map_value_begin()?.seq_begin(Default::default())?;

                stream.seq_elem_begin()?.i64(3)?;

                stream.seq_end()?;

                stream.map_end()?;

                stream.map_key(&11)?;

                stream.map_value(&111)?;

                stream.map_end()
            }
        }

        #[test]
        fn sval_derive() {
            let ser = sval::serde::v1::to_serialize(Struct {
                a: 1,
                b: 2,
                c: Nested { a: 3, b: "Hello!" },
            });

            let mut buf = String::new();
            sval_json::to_fmt(&mut buf, &sval::serde::v1::to_value(ser)).unwrap();

            let expected = "{\"a\":1,\"b\":2,\"renamed\":{\"a\":3,\"b\":\"Hello!\"}}";

            assert_eq!(expected, buf);
        }

        #[test]
        #[should_panic]
        fn sval_to_serde_anonymous() {
            let ser = sval::serde::v1::to_serialize(Anonymous);

            // The anonymous map isn't supported in no-std
            let mut buf = String::new();
            sval_json::to_fmt(&mut buf, &sval::serde::v1::to_value(ser)).unwrap();
        }

        #[test]
        fn serde_to_sval_newtype() {
            let ser = sval::serde::v1::to_serialize(NewType(42));

            let mut buf = String::new();
            sval_json::to_fmt(&mut buf, &sval::serde::v1::to_value(ser)).unwrap();

            let expected = "42";

            assert_eq!(expected, buf);
        }

        #[test]
        fn serde_to_sval_tagged_unit() {
            let ser = sval::serde::v1::to_serialize(Tagged::Unit);

            let mut buf = String::new();
            sval_json::to_fmt(&mut buf, &sval::serde::v1::to_value(ser)).unwrap();

            let expected = "\"Unit\"";

            assert_eq!(expected, buf);
        }

        #[test]
        fn serde_to_sval_tagged_newtype() {
            let ser = sval::serde::v1::to_serialize(Tagged::NewType(42));

            let mut buf = String::new();
            sval_json::to_fmt(&mut buf, &sval::serde::v1::to_value(ser)).unwrap();

            let expected = "{\"NewType\":42}";

            assert_eq!(expected, buf);
        }

        #[test]
        fn serde_to_sval_tagged_tuple() {
            let ser = sval::serde::v1::to_serialize(Tagged::Tuple(42, 43));

            let mut buf = String::new();
            sval_json::to_fmt(&mut buf, &sval::serde::v1::to_value(ser)).unwrap();

            let expected = "{\"Tuple\":[42,43]}";

            assert_eq!(expected, buf);
        }

        #[test]
        fn serde_to_sval_tagged_struct() {
            let ser = sval::serde::v1::to_serialize(Tagged::Struct {
                a: 1,
                b: 2,
            });

            let mut buf = String::new();
            sval_json::to_fmt(&mut buf, &sval::serde::v1::to_value(ser)).unwrap();

            let expected = "{\"Struct\":{\"a\":1,\"b\":2}}";

            assert_eq!(expected, buf);
        }
    }
}
