#![cfg(test)]

use sval_derive::Value;
use sval_test::assert_tokens;

mod derive_record {
    use super::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct Record {
            a: i32,
        }

        assert_tokens(&Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordBegin(None, Some(sval::Label::new("Record")), None, Some(1)),
                RecordValueBegin(None, sval::Label::new("a")),
                I32(42),
                RecordValueEnd(None, sval::Label::new("a")),
                RecordEnd(None, Some(sval::Label::new("Record")), None),
            ]
        })
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const FIELD: sval::Tag = sval::Tag::new("field");

        #[derive(Value)]
        #[sval(tag = "CONTAINER")]
        struct Record {
            #[sval(tag = "FIELD")]
            a: i32,
        }

        assert_tokens(&Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordBegin(
                    Some(CONTAINER),
                    Some(sval::Label::new("Record")),
                    None,
                    Some(1),
                ),
                RecordValueBegin(Some(FIELD), sval::Label::new("a")),
                I32(42),
                RecordValueEnd(Some(FIELD), sval::Label::new("a")),
                RecordEnd(Some(CONTAINER), Some(sval::Label::new("Record")), None),
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
                RecordBegin(None, Some(sval::Label::new("Record")), None, Some(0)),
                RecordEnd(None, Some(sval::Label::new("Record")), None),
            ]
        })
    }
}

mod derive_tuple {
    use super::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct Tuple(i32, i32);

        assert_tokens(&Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(sval::Label::new("Tuple")), None, Some(2)),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleValueBegin(None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleEnd(None, Some(sval::Label::new("Tuple")), None),
            ]
        })
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const FIELD: sval::Tag = sval::Tag::new("field");

        #[derive(Value)]
        #[sval(tag = "CONTAINER")]
        struct Tuple(#[sval(tag = "FIELD")] i32, #[sval(tag = "FIELD")] i32);

        assert_tokens(&Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(
                    Some(CONTAINER),
                    Some(sval::Label::new("Tuple")),
                    None,
                    Some(2),
                ),
                TupleValueBegin(Some(FIELD), sval::Index::new(0)),
                I32(42),
                TupleValueEnd(Some(FIELD), sval::Index::new(0)),
                TupleValueBegin(Some(FIELD), sval::Index::new(1)),
                I32(43),
                TupleValueEnd(Some(FIELD), sval::Index::new(1)),
                TupleEnd(Some(CONTAINER), Some(sval::Label::new("Tuple")), None),
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
                TupleBegin(None, Some(sval::Label::new("Tuple")), None, Some(0)),
                TupleEnd(None, Some(sval::Label::new("Tuple")), None),
            ]
        })
    }
}

mod derive_newtype {
    use super::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct Tagged(i32);

        assert_tokens(&Tagged(42), {
            use sval_test::Token::*;

            &[
                TaggedBegin(None, Some(sval::Label::new("Tagged")), None),
                I32(42),
                TaggedEnd(None, Some(sval::Label::new("Tagged")), None),
            ]
        })
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");

        #[derive(Value)]
        #[sval(tag = "CONTAINER")]
        struct Tagged(i32);

        assert_tokens(&Tagged(42), {
            use sval_test::Token::*;

            &[
                TaggedBegin(Some(CONTAINER), Some(sval::Label::new("Tagged")), None),
                I32(42),
                TaggedEnd(Some(CONTAINER), Some(sval::Label::new("Tagged")), None),
            ]
        })
    }
}
