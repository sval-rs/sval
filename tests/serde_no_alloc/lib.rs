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
            test::Token,
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
            fn stream(&self, stream: &mut value::Stream) -> value::Result {
                stream.map_begin(None)?;

                stream.map_key_begin()?.i64(1)?;

                stream.map_value_begin()?.map_begin(None)?;

                stream.map_key(2)?;

                stream.map_value_begin()?.seq_begin(None)?;

                stream.seq_elem_begin()?.i64(3)?;

                stream.seq_end()?;

                stream.map_end()?;

                stream.map_key(11)?;

                stream.map_value(111)?;

                stream.map_end()
            }
        }

        #[test]
        fn sval_derive() {
            let ser = sval::serde::to_serialize(Struct {
                a: 1,
                b: 2,
                c: Nested { a: 3, b: "Hello!" },
            });

            let v = sval::test::tokens(sval::serde::to_value(ser));
            assert_eq!(
                vec![
                    Token::MapBegin(Some(3)),
                    Token::Str(String::from("a")),
                    Token::Signed(1),
                    Token::Str(String::from("b")),
                    Token::Signed(2),
                    Token::Str(String::from("renamed")),
                    Token::MapBegin(Some(2)),
                    Token::Str(String::from("a")),
                    Token::Signed(3),
                    Token::Str(String::from("b")),
                    Token::Str(String::from("Hello!")),
                    Token::MapEnd,
                    Token::MapEnd,
                ],
                v
            );
        }

        #[test]
        #[should_panic]
        fn sval_to_serde_anonymous() {
            let ser = sval::serde::to_serialize(Anonymous);

            // The anonymous map isn't supported in no-std
            sval::test::tokens(sval::serde::to_value(ser));
        }
    }
}
