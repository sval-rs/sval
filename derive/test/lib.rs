#![cfg(test)]

use sval_derive::Value;
use sval_test::assert_tokens;

fn assert_tokens_ref<'sval>(
    value: impl sval_ref::ValueRef<'sval>,
    tokens: &[sval_test::Token<'sval>],
) {
    let mut actual = sval_test::TokenBuf::new();
    value.stream_ref(&mut actual).unwrap();

    assert_eq!(tokens, actual.as_tokens());
}

mod derive_struct {
    use super::*;

    #[allow(unused_imports)]
    use crate::shadow::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct RecordTuple {
            a: i32,
        }

        assert_tokens(&RecordTuple { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn basic_ref() {
        #[derive(Value)]
        #[sval(ref)]
        struct RecordTuple<'a> {
            a: &'a i32,
        }

        let value = RecordTuple { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_explicit_lifetime_and_generics() {
        #[derive(Value)]
        #[sval(ref = "'b")]
        struct RecordTuple<'a, 'b, T> {
            a: &'a i32,
            b: &'b i32,
            t: T,
        }

        let value = RecordTuple {
            a: &42,
            b: &43,
            t: 100,
        };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(3),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("b"),
                    sval::Index::new(1),
                ),
                I32(43),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("b"),
                    sval::Index::new(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("t"),
                    sval::Index::new(2),
                ),
                I32(100),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("t"),
                    sval::Index::new(2),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_inner_ref_generic_field() {
        #[derive(Value)]
        #[sval(ref)]
        struct RecordTuple<'a, T> {
            #[sval(inner_ref)]
            field: T,
            #[sval(skip)]
            _marker: ::std::marker::PhantomData<&'a ()>,
        }

        #[derive(Value)]
        #[sval(transparent, ref)]
        struct Inner<'a>(#[sval(outer_ref)] &'a i32);

        let value = RecordTuple {
            field: Inner(&42),
            _marker: ::std::marker::PhantomData,
        };

        assert_tokens(&value, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        });
        // Note: assert_tokens_ref is not applicable here because `#[sval(inner_ref)]`
        // delegates to `T: ValueRef<'sval>`, but `&i32` doesn't implement `ValueRef`
        // since `i32` is a primitive type without a `ValueRef` implementation.
    }

    #[test]
    fn ref_with_inner_ref_concrete_field() {
        #[derive(Value)]
        #[sval(ref)]
        struct Outer<'a> {
            #[sval(inner_ref)]
            field: Inner<'a>,
        }

        #[derive(Value)]
        #[sval(ref)]
        struct Inner<'a> {
            #[sval(outer_ref)]
            field: &'a i32,
        }

        let value = Outer {
            field: Inner { field: &42 },
        };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Outer")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Inner")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Inner")),
                    ::std::option::Option::None,
                ),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Outer")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const FIELD: sval::Tag = sval::Tag::new("field");

        #[derive(Value)]
        #[sval(ref, tag = CONTAINER, label = "record")]
        struct Record<'a> {
            #[sval(tag = FIELD)]
            a: &'a i32,
        }

        let value = Record { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::Some(FIELD),
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::Some(FIELD),
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("record")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_unlabeled_fields() {
        #[derive(Value)]
        #[sval(ref, unlabeled_fields)]
        struct Tuple<'a> {
            a: &'a i32,
        }

        let value = Tuple { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_unindexed_fields() {
        #[derive(Value)]
        #[sval(ref, unindexed_fields)]
        struct Record<'a> {
            a: &'a i32,
        }

        let value = Record { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordValueBegin(::std::option::Option::None, sval::Label::new("a")),
                I32(42),
                RecordValueEnd(::std::option::Option::None, sval::Label::new("a")),
                RecordEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_unlabeled_unindexed_fields() {
        #[derive(Value)]
        #[sval(ref, unlabeled_fields, unindexed_fields)]
        struct Seq<'a> {
            a: &'a i32,
        }

        let value = Seq { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
                SeqBegin(::std::option::Option::Some(1)),
                SeqValueBegin,
                I32(42),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_data_tag() {
        #[derive(Value)]
        #[sval(ref)]
        struct RecordTuple<'a> {
            #[sval(data_tag = sval::tags::NUMBER)]
            a: &'a i32,
        }

        let value = RecordTuple { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                TaggedBegin(
                    ::std::option::Option::Some(sval::tags::NUMBER),
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::Some(sval::tags::NUMBER),
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_skip() {
        #[derive(Value)]
        #[sval(ref)]
        struct Record<'a> {
            #[sval(skip)]
            _skipped: &'a i32,
            a: &'a i32,
        }

        let value = Record {
            _skipped: &1,
            a: &42,
        };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_computed() {
        struct Double<'a> {
            inner: &'a i32,
        }

        impl<'a> sval::Value for Double<'a> {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                stream.i32(self.inner * 2)
            }
        }

        #[derive(Value)]
        #[sval(ref)]
        struct Record<'a> {
            #[sval(computed)]
            doubled: Double<'a>,
            a: &'a i32,
        }

        let value = Record {
            doubled: Double { inner: &21 },
            a: &42,
        };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("doubled"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("doubled"),
                    sval::Index::new(0),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(1),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(1),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn uncooked() {
        #[derive(Value)]
        struct RecordTuple {
            r#type: i32,
        }

        assert_tokens(&RecordTuple { r#type: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("type"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("type"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn generic() {
        #[derive(Value)]
        struct RecordTuple<S> {
            a: S,
        }

        assert_tokens(&RecordTuple { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn indexed() {
        const B_INDEX: sval::Index = sval::Index::new(3);

        #[derive(Value)]
        struct RecordTuple {
            #[sval(index = 1)]
            a: i32,
            #[sval(index = B_INDEX)]
            b: i32,
        }

        assert_tokens(&RecordTuple { a: 42, b: 57 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(1),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("b"),
                    sval::Index::new(3),
                ),
                I32(57),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("b"),
                    sval::Index::new(3),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn unlabeled() {
        #[derive(Value)]
        #[sval(unlabeled_fields)]
        struct Tuple {
            a: i32,
        }

        assert_tokens(&Tuple { a: 42 }, {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn unindexed() {
        #[derive(Value)]
        #[sval(unindexed_fields)]
        struct Record {
            a: i32,
        }

        assert_tokens(&Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordValueBegin(::std::option::Option::None, sval::Label::new("a")),
                I32(42),
                RecordValueEnd(::std::option::Option::None, sval::Label::new("a")),
                RecordEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn unlabeled_unindexed() {
        #[derive(Value)]
        #[sval(unlabeled_fields, unindexed_fields)]
        struct Seq {
            a: i32,
        }

        assert_tokens(&Seq { a: 42 }, {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
                SeqBegin(::std::option::Option::Some(1)),
                SeqValueBegin,
                I32(42),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn data_tagged() {
        #[derive(Value)]
        struct RecordTuple {
            #[sval(data_tag = sval::tags::NUMBER)]
            a: i32,
        }

        assert_tokens(&RecordTuple { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                TaggedBegin(
                    ::std::option::Option::Some(sval::tags::NUMBER),
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::Some(sval::tags::NUMBER),
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn unlabeled_unindexed_data_tagged() {
        #[derive(Value)]
        #[sval(unlabeled_fields, unindexed_fields)]
        struct Seq {
            #[sval(data_tag = "sval::tags::NUMBER")]
            a: i32,
        }

        assert_tokens(&Seq { a: 42 }, {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
                SeqBegin(::std::option::Option::Some(1)),
                SeqValueBegin,
                TaggedBegin(
                    ::std::option::Option::Some(sval::tags::NUMBER),
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::Some(sval::tags::NUMBER),
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn flattened() {
        #[derive(Value)]
        struct Inner {
            b: i32,
            c: i32,
        }

        #[derive(Value)]
        struct RecordTuple {
            a: i32,
            #[sval(flatten)]
            b: Inner,
            d: i32,
        }

        assert_tokens(
            &RecordTuple {
                a: 1,
                b: Inner { b: 2, c: 3 },
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(
                        ::std::option::Option::None,
                        ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                        ::std::option::Option::None,
                        ::std::option::Option::None,
                    ),
                    RecordTupleValueBegin(
                        ::std::option::Option::None,
                        sval::Label::new("a"),
                        sval::Index::new(0),
                    ),
                    I32(1),
                    RecordTupleValueEnd(
                        ::std::option::Option::None,
                        sval::Label::new("a"),
                        sval::Index::new(0),
                    ),
                    RecordTupleValueBegin(
                        ::std::option::Option::None,
                        sval::Label::new("b"),
                        sval::Index::new(1),
                    ),
                    I32(2),
                    RecordTupleValueEnd(
                        ::std::option::Option::None,
                        sval::Label::new("b"),
                        sval::Index::new(1),
                    ),
                    RecordTupleValueBegin(
                        ::std::option::Option::None,
                        sval::Label::new("c"),
                        sval::Index::new(2),
                    ),
                    I32(3),
                    RecordTupleValueEnd(
                        ::std::option::Option::None,
                        sval::Label::new("c"),
                        sval::Index::new(2),
                    ),
                    RecordTupleValueBegin(
                        ::std::option::Option::None,
                        sval::Label::new("d"),
                        sval::Index::new(3),
                    ),
                    I32(4),
                    RecordTupleValueEnd(
                        ::std::option::Option::None,
                        sval::Label::new("d"),
                        sval::Index::new(3),
                    ),
                    RecordTupleEnd(
                        ::std::option::Option::None,
                        ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                        ::std::option::Option::None,
                    ),
                ]
            },
        )
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const FIELD: sval::Tag = sval::Tag::new("field");

        #[derive(Value)]
        #[sval(tag = CONTAINER, label = "record", index = 0)]
        struct Record {
            #[sval(tag = "FIELD", label = "field0")]
            a: i32,
        }

        assert_tokens(&Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("record")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::Some(FIELD),
                    sval::Label::new("field0"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::Some(FIELD),
                    sval::Label::new("field0"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("record")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        })
    }

    #[test]
    fn empty() {
        #[derive(Value)]
        struct Record {}

        assert_tokens(&Record {}, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn skip() {
        #[derive(Value)]
        struct Record {
            #[sval(skip)]
            #[allow(dead_code)]
            skipped: i32,
            a: i32,
        }

        assert_tokens(&Record { skipped: 1, a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }
}

mod derive_tuple {
    use super::*;

    #[allow(unused_imports)]
    use crate::shadow::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct Tuple(i32, i32);

        assert_tokens(&Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn labeled() {
        const B_LABEL: sval::Label<'static> = sval::Label::new("B");

        #[derive(Value)]
        struct RecordTuple(#[sval(label = "A")] i32, #[sval(label = B_LABEL)] i32);

        assert_tokens(&RecordTuple(42, 43), {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("A"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("A"),
                    sval::Index::new(0),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("B"),
                    sval::Index::new(1),
                ),
                I32(43),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("B"),
                    sval::Index::new(1),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn indexed() {
        #[derive(Value)]
        struct Tuple(#[sval(index = 1)] i32, i32);

        assert_tokens(&Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(2)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(2)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn unindexed() {
        #[derive(Value)]
        #[sval(unindexed_fields)]
        struct Seq(i32, i32);

        assert_tokens(&Seq(42, 43), {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
                SeqBegin(::std::option::Option::Some(2)),
                SeqValueBegin,
                I32(42),
                SeqValueEnd,
                SeqValueBegin,
                I32(43),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn flattened() {
        #[derive(Value)]
        struct Inner(i32, i32);

        #[derive(Value)]
        struct Tuple(i32, #[sval(flatten)] Inner, i32);

        assert_tokens(&Tuple(1, Inner(2, 3), 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(1),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(2),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(2)),
                I32(3),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(2)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(3)),
                I32(4),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(3)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn unindexed_flattened() {
        #[derive(Value)]
        #[sval(unindexed_fields)]
        struct Inner(i32, i32);

        #[derive(Value)]
        #[sval(unindexed_fields)]
        struct Seq(i32, #[sval(flatten)] Inner, i32);

        assert_tokens(&Seq(1, Inner(2, 3), 4), {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
                SeqBegin(::std::option::Option::None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(3),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const FIELD: sval::Tag = sval::Tag::new("field");

        #[derive(Value)]
        #[sval(tag = "CONTAINER", label = "tuple", index = 0)]
        struct Tuple(
            #[sval(tag = "FIELD", index = 1)] i32,
            #[sval(tag = "FIELD", index = 2)] i32,
        );

        assert_tokens(&Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("tuple")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::Some(FIELD), sval::Index::new(1)),
                I32(42),
                TupleValueEnd(::std::option::Option::Some(FIELD), sval::Index::new(1)),
                TupleValueBegin(::std::option::Option::Some(FIELD), sval::Index::new(2)),
                I32(43),
                TupleValueEnd(::std::option::Option::Some(FIELD), sval::Index::new(2)),
                TupleEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("tuple")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        })
    }

    #[test]
    fn skip() {
        #[allow(dead_code)]
        #[derive(Value)]
        struct Tuple(#[sval(skip)] i32, i32);

        assert_tokens(&Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn empty() {
        #[derive(Value)]
        struct Tuple();

        assert_tokens(&Tuple(), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(0),
                ),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn ref_basic() {
        #[derive(Value)]
        #[sval(ref)]
        struct Tuple<'a>(&'a i32, &'a i32);

        let value = Tuple(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_labeled_fields() {
        #[derive(Value)]
        #[sval(ref)]
        struct RecordTuple<'a>(#[sval(label = "A")] &'a i32, #[sval(label = "B")] &'a i32);

        let value = RecordTuple(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("A"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("A"),
                    sval::Index::new(0),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("B"),
                    sval::Index::new(1),
                ),
                I32(43),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("B"),
                    sval::Index::new(1),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("RecordTuple")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_unindexed_fields() {
        #[derive(Value)]
        #[sval(ref, unindexed_fields)]
        struct Seq<'a>(&'a i32, &'a i32);

        let value = Seq(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
                SeqBegin(::std::option::Option::Some(2)),
                SeqValueBegin,
                I32(42),
                SeqValueEnd,
                SeqValueBegin,
                I32(43),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Seq")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_skip() {
        #[allow(dead_code)]
        #[derive(Value)]
        #[sval(ref)]
        struct Tuple<'a>(#[sval(skip)] &'a i32, &'a i32);

        let value = Tuple(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }
}

mod derive_newtype {
    use super::*;

    #[allow(unused_imports)]
    use crate::shadow::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct Tagged(i32);

        assert_tokens(&Tagged(42), {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
            ]
        })
    }

    #[test]
    fn transparent() {
        #[derive(Value)]
        #[sval(transparent)]
        struct Tagged(i32);

        assert_tokens(&Tagged(42), {
            use sval_test::Token::*;

            &[I32(42)]
        })
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");

        #[derive(Value)]
        #[sval(tag = "CONTAINER", label = "tagged", index = 0)]
        struct Tagged(i32);

        assert_tokens(&Tagged(42), {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        })
    }

    #[test]
    fn ref_basic() {
        #[derive(Value)]
        #[sval(ref)]
        struct Tagged<'a>(&'a i32);

        let value = Tagged(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");

        #[derive(Value)]
        #[sval(ref, tag = "CONTAINER", label = "tagged", index = 0)]
        struct Tagged<'a>(&'a i32);

        let value = Tagged(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_transparent() {
        #[derive(Value)]
        #[sval(ref, transparent)]
        struct Newtype<'a>(&'a i32);

        let value = Newtype(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[I32(42)]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_inner_ref() {
        #[derive(Value)]
        #[sval(ref)]
        struct Inner<'a> {
            #[sval(outer_ref)]
            value: &'a i32,
        }

        #[derive(Value)]
        #[sval(ref)]
        struct Newtype<'a>(#[sval(inner_ref)] Inner<'a>);

        let value = Newtype(Inner { value: &42 });
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Newtype")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Inner")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("value"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("value"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Inner")),
                    ::std::option::Option::None,
                ),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Newtype")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }
}

mod derive_unit_struct {
    use super::*;

    #[allow(unused_imports)]
    use crate::shadow::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct Tag;

        assert_tokens(&Tag, {
            use sval_test::Token::*;

            &[Tag(
                ::std::option::Option::None,
                ::std::option::Option::Some(sval::Label::new("Tag")),
                ::std::option::Option::None,
            )]
        })
    }

    #[test]
    #[allow(non_camel_case_types)]
    fn uncooked() {
        #[derive(Value)]
        struct r#type;

        assert_tokens(&r#type, {
            use sval_test::Token::*;

            &[Tag(
                ::std::option::Option::None,
                ::std::option::Option::Some(sval::Label::new("type")),
                ::std::option::Option::None,
            )]
        })
    }
}

mod derive_enum {
    use super::*;

    #[allow(unused_imports)]
    use crate::shadow::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        enum Enum {
            Tag,
            Tagged(i32),
            Record { a: i32 },
            Tuple(i32, i32),
        }

        assert_tokens(&Enum::Tag, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tag")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tagged(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::Some(sval::Index::new(2)),
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::Some(sval::Index::new(2)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::Some(sval::Index::new(3)),
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::Some(sval::Index::new(3)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    #[allow(non_camel_case_types)]
    fn uncooked() {
        #[derive(Value)]
        enum Enum {
            r#type,
        }

        assert_tokens(&Enum::r#type, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("type")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    fn unlabeled() {
        #[derive(Value)]
        #[sval(unlabeled_variants)]
        enum Enum {
            Tag,
            Tagged(i32),
            Record { a: i32 },
            Tuple(i32, i32),
        }

        assert_tokens(&Enum::Tag, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tagged(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(2)),
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(2)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(3)),
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(3)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    fn unindexed() {
        #[derive(Value)]
        #[sval(unindexed_variants)]
        enum Enum {
            Tag,
            Tagged(i32),
            Record { a: i32 },
            Tuple(i32, i32),
        }

        assert_tokens(&Enum::Tag, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tag")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tagged(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    fn unlabeled_unindexed() {
        #[derive(Value)]
        #[sval(unlabeled_variants, unindexed_variants)]
        enum Enum {
            Tag,
            Tagged(i32),
            Record { a: i32 },
            Tuple(i32, i32),
        }

        assert_tokens(&Enum::Tag, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tagged(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const VARIANT: sval::Tag = sval::Tag::new("variant");
        const FIELD: sval::Tag = sval::Tag::new("field");

        #[derive(Value)]
        #[sval(tag = "CONTAINER", label = "enum", index = 0)]
        enum Enum {
            #[sval(tag = "VARIANT", label = "tag", index = -1)]
            Tag,
            #[sval(tag = "VARIANT", label = "tagged", index = -2)]
            Tagged(i32),
            #[sval(tag = "VARIANT", label = "record", index = -3)]
            Record {
                #[sval(tag = "FIELD", label = "field")]
                a: i32,
            },
            #[sval(tag = "VARIANT", label = "tuple", index = -4)]
            Tuple(
                #[sval(tag = "FIELD", index = 1)] i32,
                #[sval(tag = "FIELD", index = 2)] i32,
            ),
        }

        assert_tokens(&Enum::Tag, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                Tag(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tag")),
                    ::std::option::Option::Some(sval::Index::new_isize(-1)),
                ),
                EnumEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        });

        assert_tokens(&Enum::Tagged(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                TaggedBegin(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new_isize(-2)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new_isize(-2)),
                ),
                EnumEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        });

        assert_tokens(&Enum::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                RecordTupleBegin(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("record")),
                    ::std::option::Option::Some(sval::Index::new_isize(-3)),
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::Some(FIELD),
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::Some(FIELD),
                    sval::Label::new("field"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("record")),
                    ::std::option::Option::Some(sval::Index::new_isize(-3)),
                ),
                EnumEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                TupleBegin(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tuple")),
                    ::std::option::Option::Some(sval::Index::new_isize(-4)),
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::Some(FIELD), sval::Index::new(1)),
                I32(42),
                TupleValueEnd(::std::option::Option::Some(FIELD), sval::Index::new(1)),
                TupleValueBegin(::std::option::Option::Some(FIELD), sval::Index::new(2)),
                I32(43),
                TupleValueEnd(::std::option::Option::Some(FIELD), sval::Index::new(2)),
                TupleEnd(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tuple")),
                    ::std::option::Option::Some(sval::Index::new_isize(-4)),
                ),
                EnumEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
            ]
        });
    }

    #[test]
    fn skip() {
        #[derive(Value)]
        #[allow(dead_code)]
        enum Enum {
            Record {
                #[sval(skip)]
                #[allow(dead_code)]
                skipped: i32,
                a: i32,
            },
            Tuple(#[sval(skip)] i32, i32),
        }

        assert_tokens(&Enum::Record { skipped: 1, a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                    ::std::option::Option::Some(1),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    fn discriminant() {
        #[derive(Value)]
        #[repr(i32)]
        enum Enum {
            A = -3,
            #[sval(index = -2)]
            B = 4,
            C(i32) = -1,
        }

        assert_tokens(&Enum::A, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("A")),
                    ::std::option::Option::Some(sval::Index::new_i32(-3)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::B, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("B")),
                    ::std::option::Option::Some(sval::Index::new_i32(-2)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Enum::C(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("C")),
                    ::std::option::Option::Some(sval::Index::new_i32(-1)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("C")),
                    ::std::option::Option::Some(sval::Index::new_i32(-1)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    fn empty() {
        #![allow(dead_code)]

        // Just ensure `derive` works on empty enums
        #[derive(Value)]
        enum Enum {}
    }

    #[test]
    fn dynamic() {
        #[derive(Value)]
        #[sval(dynamic)]
        enum Dynamic {
            Tag,
            I32(i32),
            Bool(bool),
            Record { a: i32 },
            Tuple(i32, i32),
        }

        assert_tokens(&Dynamic::Tag, {
            use sval_test::Token::*;

            &[Tag(
                ::std::option::Option::None,
                ::std::option::Option::Some(sval::Label::new("Tag")),
                ::std::option::Option::None,
            )]
        });

        assert_tokens(&Dynamic::Bool(true), {
            use sval_test::Token::*;

            &[Bool(true)]
        });

        assert_tokens(&Dynamic::I32(42), {
            use sval_test::Token::*;

            &[I32(42)]
        });

        assert_tokens(&Dynamic::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
            ]
        });

        assert_tokens(&Dynamic::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
            ]
        });
    }

    #[test]
    fn ref_basic() {
        #[derive(Value)]
        #[sval(ref)]
        enum Enum<'a> {
            Tag,
            Tagged(&'a i32),
            Record { a: &'a i32 },
            Tuple(&'a i32, &'a i32),
        }

        let value = Enum::Tag;
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tag")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Tagged(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Record { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::Some(sval::Index::new(2)),
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::Some(sval::Index::new(2)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Tuple(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::Some(sval::Index::new(3)),
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::Some(sval::Index::new(3)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_dynamic() {
        #[derive(Value)]
        #[sval(ref, dynamic)]
        enum Dynamic<'a> {
            Tag,
            I32(&'a i32),
            Bool(&'a bool),
            Record { a: &'a i32 },
            Tuple(&'a i32, &'a i32),
        }

        let value = Dynamic::Tag;
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[Tag(
                ::std::option::Option::None,
                ::std::option::Option::Some(sval::Label::new("Tag")),
                ::std::option::Option::None,
            )]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Dynamic::Bool(&true);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[Bool(true)]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Dynamic::I32(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[I32(42)]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Dynamic::Record { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Dynamic::Tuple(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_unlabeled_variants() {
        #[derive(Value)]
        #[sval(ref, unlabeled_variants)]
        enum Enum<'a> {
            Tag,
            Tagged(&'a i32),
            Record { a: &'a i32 },
            Tuple(&'a i32, &'a i32),
        }

        let value = Enum::Tag;
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Tagged(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Record { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(2)),
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(2)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Tuple(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(3)),
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Index::new(3)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_unindexed_variants() {
        #[derive(Value)]
        #[sval(ref, unindexed_variants)]
        enum Enum<'a> {
            Tag,
            Tagged(&'a i32),
            Record { a: &'a i32 },
            Tuple(&'a i32, &'a i32),
        }

        let value = Enum::Tag;
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tag")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Tagged(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tagged")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Record { a: &42 };
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                RecordTupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(1),
                ),
                RecordTupleValueBegin(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                I32(42),
                RecordTupleValueEnd(
                    ::std::option::Option::None,
                    sval::Label::new("a"),
                    sval::Index::new(0),
                ),
                RecordTupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Record")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Tuple(&42, &43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TupleBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                    ::std::option::Option::Some(2),
                ),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(0)),
                TupleValueBegin(::std::option::Option::None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(::std::option::Option::None, sval::Index::new(1)),
                TupleEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Tuple")),
                    ::std::option::Option::None,
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const VARIANT: sval::Tag = sval::Tag::new("variant");

        #[derive(Value)]
        #[sval(ref, tag = "CONTAINER", label = "enum")]
        enum Enum<'a> {
            #[sval(tag = "VARIANT", label = "tag")]
            Tag,
            #[sval(tag = "VARIANT", label = "tagged")]
            Tagged(&'a i32),
        }

        let value = Enum::Tag;
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::None,
                ),
                Tag(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tag")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::Tagged(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::Some(VARIANT),
                    ::std::option::Option::Some(sval::Label::new("tagged")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                EnumEnd(
                    ::std::option::Option::Some(CONTAINER),
                    ::std::option::Option::Some(sval::Label::new("enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }

    #[test]
    fn ref_with_explicit_lifetime() {
        #[derive(Value)]
        #[sval(ref = "'b")]
        enum Enum<'a, 'b> {
            A(&'a i32),
            B(&'b i32),
        }

        let value = Enum::A(&42);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("A")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                I32(42),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("A")),
                    ::std::option::Option::Some(sval::Index::new(0)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);

        let value = Enum::B(&43);
        let tokens: &[sval_test::Token] = {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
                TaggedBegin(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("B")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                I32(43),
                TaggedEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("B")),
                    ::std::option::Option::Some(sval::Index::new(1)),
                ),
                EnumEnd(
                    ::std::option::Option::None,
                    ::std::option::Option::Some(sval::Label::new("Enum")),
                    ::std::option::Option::None,
                ),
            ]
        };
        assert_tokens(&value, tokens);
        assert_tokens_ref(&value, tokens);
    }
}

mod shadow {
    // Shadow core imports
    #![allow(dead_code)]

    pub struct Result;
    pub struct Ok;
    pub struct Err;
    pub struct Some;
    pub struct None;
    pub struct String;
    pub struct Vec;
    pub mod core {}
    pub mod std {}
}

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("compile_fail/*.rs");
}
