#![cfg(test)]

#[macro_use]
extern crate sval;

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

        struct Anonymous;

        impl Value for Anonymous {
            fn stream<'s, 'v>(&'v self, mut stream: value::Stream<'s, 'v>) -> value::Result {
                stream.map_begin(None)?;

                stream.map_key_begin()?.i64(1)?;

                stream.map_value_begin()?.map_begin(None)?;

                stream.map_key(&2)?;

                stream.map_value_begin()?.seq_begin(None)?;

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
    }
}
