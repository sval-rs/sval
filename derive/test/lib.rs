#![cfg(test)]

use sval_derive::Value;
use sval_test::assert_tokens;

mod derive_struct {
    use super::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct RecordTuple {
            a: i32,
        }

        assert_tokens(&RecordTuple { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(None, Some(sval::Label::new("RecordTuple")), None, Some(1)),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                RecordTupleEnd(None, Some(sval::Label::new("RecordTuple")), None),
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
                RecordTupleBegin(None, Some(sval::Label::new("RecordTuple")), None, Some(1)),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                RecordTupleEnd(None, Some(sval::Label::new("RecordTuple")), None),
            ]
        })
    }

    #[test]
    fn indexed() {
        #[derive(Value)]
        struct RecordTuple {
            #[sval(index = 1)]
            a: i32,
            b: i32,
        }

        assert_tokens(&RecordTuple { a: 42, b: 57 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(None, Some(sval::Label::new("RecordTuple")), None, Some(2)),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(1)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(1)),
                RecordTupleValueBegin(None, sval::Label::new("b"), sval::Index::new(2)),
                I32(57),
                RecordTupleValueEnd(None, sval::Label::new("b"), sval::Index::new(2)),
                RecordTupleEnd(None, Some(sval::Label::new("RecordTuple")), None),
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
                TupleBegin(None, Some(sval::Label::new("Tuple")), None, Some(1)),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleEnd(None, Some(sval::Label::new("Tuple")), None),
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
                RecordBegin(None, Some(sval::Label::new("Record")), None, Some(1)),
                RecordValueBegin(None, sval::Label::new("a")),
                I32(42),
                RecordValueEnd(None, sval::Label::new("a")),
                RecordEnd(None, Some(sval::Label::new("Record")), None),
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
                TaggedBegin(None, Some(sval::Label::new("Seq")), None),
                SeqBegin(Some(1)),
                SeqValueBegin,
                I32(42),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(None, Some(sval::Label::new("Seq")), None),
            ]
        })
    }

    #[test]
    fn data_tagged() {
        #[derive(Value)]
        struct RecordTuple {
            #[sval(data_tag = "sval::tags::NUMBER")]
            a: i32,
        }

        assert_tokens(&RecordTuple { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(None, Some(sval::Label::new("RecordTuple")), None, Some(1)),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                TaggedBegin(Some(sval::tags::NUMBER), None, None),
                I32(42),
                TaggedEnd(Some(sval::tags::NUMBER), None, None),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                RecordTupleEnd(None, Some(sval::Label::new("RecordTuple")), None),
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
                TaggedBegin(None, Some(sval::Label::new("Seq")), None),
                SeqBegin(Some(1)),
                SeqValueBegin,
                TaggedBegin(Some(sval::tags::NUMBER), None, None),
                I32(42),
                TaggedEnd(Some(sval::tags::NUMBER), None, None),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(None, Some(sval::Label::new("Seq")), None),
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
                    RecordTupleBegin(None, Some(sval::Label::new("RecordTuple")), None, None),
                    RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                    RecordTupleValueBegin(None, sval::Label::new("b"), sval::Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, sval::Label::new("b"), sval::Index::new(1)),
                    RecordTupleValueBegin(None, sval::Label::new("c"), sval::Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, sval::Label::new("c"), sval::Index::new(2)),
                    RecordTupleValueBegin(None, sval::Label::new("d"), sval::Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, sval::Label::new("d"), sval::Index::new(3)),
                    RecordTupleEnd(None, Some(sval::Label::new("RecordTuple")), None),
                ]
            },
        )
    }

    #[test]
    fn tagged() {
        const CONTAINER: sval::Tag = sval::Tag::new("container");
        const FIELD: sval::Tag = sval::Tag::new("field");

        #[derive(Value)]
        #[sval(tag = "CONTAINER", label = "record", index = 0)]
        struct Record {
            #[sval(tag = "FIELD", label = "field0")]
            a: i32,
        }

        assert_tokens(&Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(
                    Some(CONTAINER),
                    Some(sval::Label::new("record")),
                    Some(sval::Index::new(0)),
                    Some(1),
                ),
                RecordTupleValueBegin(Some(FIELD), sval::Label::new("field0"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(Some(FIELD), sval::Label::new("field0"), sval::Index::new(0)),
                RecordTupleEnd(
                    Some(CONTAINER),
                    Some(sval::Label::new("record")),
                    Some(sval::Index::new(0)),
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
                RecordTupleBegin(None, Some(sval::Label::new("Record")), None, Some(0)),
                RecordTupleEnd(None, Some(sval::Label::new("Record")), None),
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
                RecordTupleBegin(None, Some(sval::Label::new("Record")), None, Some(1)),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                RecordTupleEnd(None, Some(sval::Label::new("Record")), None),
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
    fn labeled() {
        #[derive(Value)]
        struct RecordTuple(#[sval(label = "A")] i32, #[sval(label = "B")] i32);

        assert_tokens(&RecordTuple(42, 43), {
            use sval_test::Token::*;

            &[
                RecordTupleBegin(None, Some(sval::Label::new("RecordTuple")), None, Some(2)),
                RecordTupleValueBegin(None, sval::Label::new("A"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("A"), sval::Index::new(0)),
                RecordTupleValueBegin(None, sval::Label::new("B"), sval::Index::new(1)),
                I32(43),
                RecordTupleValueEnd(None, sval::Label::new("B"), sval::Index::new(1)),
                RecordTupleEnd(None, Some(sval::Label::new("RecordTuple")), None),
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
                TupleBegin(None, Some(sval::Label::new("Tuple")), None, Some(2)),
                TupleValueBegin(None, sval::Index::new(1)),
                I32(42),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleValueBegin(None, sval::Index::new(2)),
                I32(43),
                TupleValueEnd(None, sval::Index::new(2)),
                TupleEnd(None, Some(sval::Label::new("Tuple")), None),
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
                TaggedBegin(None, Some(sval::Label::new("Seq")), None),
                SeqBegin(Some(2)),
                SeqValueBegin,
                I32(42),
                SeqValueEnd,
                SeqValueBegin,
                I32(43),
                SeqValueEnd,
                SeqEnd,
                TaggedEnd(None, Some(sval::Label::new("Seq")), None),
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
                TupleBegin(None, Some(sval::Label::new("Tuple")), None, None),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(1),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleValueBegin(None, sval::Index::new(1)),
                I32(2),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleValueBegin(None, sval::Index::new(2)),
                I32(3),
                TupleValueEnd(None, sval::Index::new(2)),
                TupleValueBegin(None, sval::Index::new(3)),
                I32(4),
                TupleValueEnd(None, sval::Index::new(3)),
                TupleEnd(None, Some(sval::Label::new("Tuple")), None),
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
                TaggedBegin(None, Some(sval::Label::new("Seq")), None),
                SeqBegin(None),
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
                TaggedEnd(None, Some(sval::Label::new("Seq")), None),
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
                    Some(CONTAINER),
                    Some(sval::Label::new("tuple")),
                    Some(sval::Index::new(0)),
                    Some(2),
                ),
                TupleValueBegin(Some(FIELD), sval::Index::new(1)),
                I32(42),
                TupleValueEnd(Some(FIELD), sval::Index::new(1)),
                TupleValueBegin(Some(FIELD), sval::Index::new(2)),
                I32(43),
                TupleValueEnd(Some(FIELD), sval::Index::new(2)),
                TupleEnd(
                    Some(CONTAINER),
                    Some(sval::Label::new("tuple")),
                    Some(sval::Index::new(0)),
                ),
            ]
        })
    }

    #[test]
    fn skip() {
        #[derive(Value)]
        struct Tuple(#[sval(skip)] i32, i32);

        assert_tokens(&Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(sval::Label::new("Tuple")), None, Some(1)),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(43),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleEnd(None, Some(sval::Label::new("Tuple")), None),
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
                    Some(CONTAINER),
                    Some(sval::Label::new("tagged")),
                    Some(sval::Index::new(0)),
                ),
                I32(42),
                TaggedEnd(
                    Some(CONTAINER),
                    Some(sval::Label::new("tagged")),
                    Some(sval::Index::new(0)),
                ),
            ]
        })
    }
}

mod derive_unit_struct {
    use super::*;

    #[test]
    fn basic() {
        #[derive(Value)]
        struct Tag;

        assert_tokens(&Tag, {
            use sval_test::Token::*;

            &[Tag(None, Some(sval::Label::new("Tag")), None)]
        })
    }
}

mod derive_enum {
    use super::*;

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
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                Tag(
                    None,
                    Some(sval::Label::new("Tag")),
                    Some(sval::Index::new(0)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        });

        assert_tokens(&Enum::Tagged(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                TaggedBegin(
                    None,
                    Some(sval::Label::new("Tagged")),
                    Some(sval::Index::new(1)),
                ),
                I32(42),
                TaggedEnd(
                    None,
                    Some(sval::Label::new("Tagged")),
                    Some(sval::Index::new(1)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        });

        assert_tokens(&Enum::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                RecordTupleBegin(
                    None,
                    Some(sval::Label::new("Record")),
                    Some(sval::Index::new(2)),
                    Some(1),
                ),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                RecordTupleEnd(
                    None,
                    Some(sval::Label::new("Record")),
                    Some(sval::Index::new(2)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                TupleBegin(
                    None,
                    Some(sval::Label::new("Tuple")),
                    Some(sval::Index::new(3)),
                    Some(2),
                ),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleValueBegin(None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleEnd(
                    None,
                    Some(sval::Label::new("Tuple")),
                    Some(sval::Index::new(3)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
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
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
                Tag(
                    Some(VARIANT),
                    Some(sval::Label::new("tag")),
                    Some(sval::Index::new_isize(-1)),
                ),
                EnumEnd(
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
            ]
        });

        assert_tokens(&Enum::Tagged(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
                TaggedBegin(
                    Some(VARIANT),
                    Some(sval::Label::new("tagged")),
                    Some(sval::Index::new_isize(-2)),
                ),
                I32(42),
                TaggedEnd(
                    Some(VARIANT),
                    Some(sval::Label::new("tagged")),
                    Some(sval::Index::new_isize(-2)),
                ),
                EnumEnd(
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
            ]
        });

        assert_tokens(&Enum::Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
                RecordTupleBegin(
                    Some(VARIANT),
                    Some(sval::Label::new("record")),
                    Some(sval::Index::new_isize(-3)),
                    Some(1),
                ),
                RecordTupleValueBegin(Some(FIELD), sval::Label::new("field"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(Some(FIELD), sval::Label::new("field"), sval::Index::new(0)),
                RecordTupleEnd(
                    Some(VARIANT),
                    Some(sval::Label::new("record")),
                    Some(sval::Index::new_isize(-3)),
                ),
                EnumEnd(
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
                TupleBegin(
                    Some(VARIANT),
                    Some(sval::Label::new("tuple")),
                    Some(sval::Index::new_isize(-4)),
                    Some(2),
                ),
                TupleValueBegin(Some(FIELD), sval::Index::new(1)),
                I32(42),
                TupleValueEnd(Some(FIELD), sval::Index::new(1)),
                TupleValueBegin(Some(FIELD), sval::Index::new(2)),
                I32(43),
                TupleValueEnd(Some(FIELD), sval::Index::new(2)),
                TupleEnd(
                    Some(VARIANT),
                    Some(sval::Label::new("tuple")),
                    Some(sval::Index::new_isize(-4)),
                ),
                EnumEnd(
                    Some(CONTAINER),
                    Some(sval::Label::new("enum")),
                    Some(sval::Index::new(0)),
                ),
            ]
        });
    }

    #[test]
    fn skip() {
        #[derive(Value)]
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
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                RecordTupleBegin(
                    None,
                    Some(sval::Label::new("Record")),
                    Some(sval::Index::new(0)),
                    Some(1),
                ),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                RecordTupleEnd(
                    None,
                    Some(sval::Label::new("Record")),
                    Some(sval::Index::new(0)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        });

        assert_tokens(&Enum::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                TupleBegin(
                    None,
                    Some(sval::Label::new("Tuple")),
                    Some(sval::Index::new(1)),
                    Some(1),
                ),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(43),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleEnd(
                    None,
                    Some(sval::Label::new("Tuple")),
                    Some(sval::Index::new(1)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
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
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                Tag(
                    None,
                    Some(sval::Label::new("A")),
                    Some(sval::Index::new_i32(-3)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        });

        assert_tokens(&Enum::B, {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                Tag(
                    None,
                    Some(sval::Label::new("B")),
                    Some(sval::Index::new_i32(-2)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        });

        assert_tokens(&Enum::C(42), {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                TaggedBegin(
                    None,
                    Some(sval::Label::new("C")),
                    Some(sval::Index::new_i32(-1)),
                ),
                I32(42),
                TaggedEnd(
                    None,
                    Some(sval::Label::new("C")),
                    Some(sval::Index::new_i32(-1)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
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

            &[Tag(None, Some(sval::Label::new("Tag")), None)]
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
                RecordTupleBegin(None, None, None, Some(1)),
                RecordTupleValueBegin(None, sval::Label::new("a"), sval::Index::new(0)),
                I32(42),
                RecordTupleValueEnd(None, sval::Label::new("a"), sval::Index::new(0)),
                RecordTupleEnd(None, None, None),
            ]
        });

        assert_tokens(&Dynamic::Tuple(42, 43), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, None, None, Some(2)),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(42),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleValueBegin(None, sval::Index::new(1)),
                I32(43),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleEnd(None, None, None),
            ]
        });
    }
}

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("compile_fail/*.rs");
}
