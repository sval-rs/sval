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
        #[sval(tag = "CONTAINER", label = "record", index = 0)]
        struct Record {
            #[sval(tag = "FIELD", label = "field0")]
            a: i32,
        }

        assert_tokens(&Record { a: 42 }, {
            use sval_test::Token::*;

            &[
                RecordBegin(
                    Some(CONTAINER),
                    Some(sval::Label::new("record")),
                    Some(sval::Index::new(0)),
                    Some(1),
                ),
                RecordValueBegin(Some(FIELD), sval::Label::new("field0")),
                I32(42),
                RecordValueEnd(Some(FIELD), sval::Label::new("field0")),
                RecordEnd(
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
                RecordBegin(None, Some(sval::Label::new("Record")), None, Some(0)),
                RecordEnd(None, Some(sval::Label::new("Record")), None),
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
                RecordBegin(None, Some(sval::Label::new("Record")), None, Some(1)),
                RecordValueBegin(None, sval::Label::new("a")),
                I32(42),
                RecordValueEnd(None, sval::Label::new("a")),
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
                RecordBegin(
                    None,
                    Some(sval::Label::new("Record")),
                    Some(sval::Index::new(2)),
                    Some(1),
                ),
                RecordValueBegin(None, sval::Label::new("a")),
                I32(42),
                RecordValueEnd(None, sval::Label::new("a")),
                RecordEnd(
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
            #[sval(tag = "VARIANT", label = "tag", index = 1)]
            Tag,
            #[sval(tag = "VARIANT", label = "tagged", index = 2)]
            Tagged(i32),
            #[sval(tag = "VARIANT", label = "record", index = 3)]
            Record {
                #[sval(tag = "FIELD", label = "field")]
                a: i32,
            },
            #[sval(tag = "VARIANT", label = "tuple", index = 4)]
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
                    Some(sval::Index::new(1)),
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
                    Some(sval::Index::new(2)),
                ),
                I32(42),
                TaggedEnd(
                    Some(VARIANT),
                    Some(sval::Label::new("tagged")),
                    Some(sval::Index::new(2)),
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
                RecordBegin(
                    Some(VARIANT),
                    Some(sval::Label::new("record")),
                    Some(sval::Index::new(3)),
                    Some(1),
                ),
                RecordValueBegin(Some(FIELD), sval::Label::new("field")),
                I32(42),
                RecordValueEnd(Some(FIELD), sval::Label::new("field")),
                RecordEnd(
                    Some(VARIANT),
                    Some(sval::Label::new("record")),
                    Some(sval::Index::new(3)),
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
                    Some(sval::Index::new(4)),
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
                    Some(sval::Index::new(4)),
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
                RecordBegin(
                    None,
                    Some(sval::Label::new("Record")),
                    Some(sval::Index::new(0)),
                    Some(1),
                ),
                RecordValueBegin(None, sval::Label::new("a")),
                I32(42),
                RecordValueEnd(None, sval::Label::new("a")),
                RecordEnd(
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
    fn empty() {
        #![allow(dead_code)]

        // Just ensure `derive` works on empty enums
        #[derive(Value)]
        enum Enum {}
    }
}

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("compile_fail/*.rs");
}
