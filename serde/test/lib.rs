#![cfg(test)]

#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

use serde_test::assert_ser_tokens;
use sval_test::assert_tokens;

use std::collections::BTreeMap;

type Map = BTreeMap<&'static str, i32>;

type Seq = Vec<i32>;

#[derive(Value, Serialize)]
struct MapStruct {
    field_0: i32,
    field_1: bool,
    field_2: &'static str,
}

#[derive(Value, Serialize)]
struct SeqStruct(i32, bool, &'static str);

#[derive(Value, Serialize)]
struct Tagged(i32);

#[derive(Value, Serialize)]
enum Enum {
    Constant,
    Tagged(i32),
    MapStruct {
        field_0: i32,
        field_1: bool,
        field_2: &'static str,
    },
    SeqStruct(i32, bool, &'static str),
}

fn test_case(
    v: (impl sval::Value + serde::Serialize),
    serde: &[serde_test::Token],
    sval: &[sval_test::Token],
) {
    assert_ser_tokens(&sval_serde::to_serialize(&v), serde);
    assert_ser_tokens(
        &sval_serde::to_serialize(sval_buffer::stream_to_value(&v).unwrap()),
        serde,
    );
    assert_ser_tokens(
        &sval_serde::to_serialize(&v as &dyn sval_dynamic::Value),
        serde,
    );
    assert_ser_tokens(&v, serde);

    assert_tokens(&sval_serde::to_value(&v), sval);

    assert_ser_tokens(&sval_serde::to_serialize(sval_serde::to_value(&v)), serde);
    assert_tokens(&sval_serde::to_value(sval_serde::to_serialize(&v)), sval);
}

#[test]
fn unit_to_serialize() {
    test_case(
        (),
        {
            use serde_test::Token::*;

            &[Unit]
        },
        {
            use sval_test::Token::*;

            &[Tag(Some(sval::tags::RUST_UNIT), None, None)]
        },
    )
}

#[test]
fn option_some_to_serialize() {
    test_case(
        Some(1i32),
        {
            use serde_test::Token::*;

            &[Some, I32(1)]
        },
        {
            use sval_test::Token::*;

            &[
                TaggedBegin(
                    Some(sval::tags::RUST_OPTION_SOME),
                    Some(sval::Label::new("Some")),
                    Some(sval::Index::new(1)),
                ),
                I32(1),
                TaggedEnd(
                    Some(sval::tags::RUST_OPTION_SOME),
                    Some(sval::Label::new("Some")),
                    Some(sval::Index::new(1)),
                ),
            ]
        },
    )
}

#[test]
fn option_none_to_serialize() {
    test_case(
        None::<i32>,
        {
            use serde_test::Token::*;

            &[None]
        },
        {
            use sval_test::Token::*;

            &[Tag(
                Some(sval::tags::RUST_OPTION_NONE),
                Some(sval::Label::new("None")),
                Some(sval::Index::new(0)),
            )]
        },
    )
}

#[test]
fn map_to_serialize() {
    test_case(
        {
            let mut map = Map::new();

            map.insert("a", 1);
            map.insert("b", 2);

            map
        },
        {
            use serde_test::Token::*;

            &[
                Map {
                    len: Option::Some(2),
                },
                Str("a"),
                I32(1),
                Str("b"),
                I32(2),
                MapEnd,
            ]
        },
        {
            use sval_test::Token::*;

            &[
                MapBegin(Some(2)),
                MapKeyBegin,
                TextBegin(Some(1)),
                TextFragmentComputed("a".into()),
                TextEnd,
                MapKeyEnd,
                MapValueBegin,
                I32(1),
                MapValueEnd,
                MapKeyBegin,
                TextBegin(Some(1)),
                TextFragmentComputed("b".into()),
                TextEnd,
                MapKeyEnd,
                MapValueBegin,
                I32(2),
                MapValueEnd,
                MapEnd,
            ]
        },
    );
}

#[test]
fn seq_to_serialize() {
    test_case(
        {
            let mut seq = Seq::new();

            seq.push(1);
            seq.push(2);

            seq
        },
        {
            use serde_test::Token::*;

            &[
                Seq {
                    len: Option::Some(2),
                },
                I32(1),
                I32(2),
                SeqEnd,
            ]
        },
        {
            use sval_test::Token::*;

            &[
                SeqBegin(Some(2)),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqEnd,
            ]
        },
    );
}

#[test]
fn map_struct_to_serialize() {
    test_case(
        MapStruct {
            field_0: 1,
            field_1: true,
            field_2: "a",
        },
        {
            use serde_test::Token::*;

            &[
                Struct {
                    name: "MapStruct",
                    len: 3,
                },
                Str("field_0"),
                I32(1),
                Str("field_1"),
                Bool(true),
                Str("field_2"),
                Str("a"),
                StructEnd,
            ]
        },
        {
            use sval_test::Token::*;

            &[
                RecordBegin(None, Some(sval::Label::new("MapStruct")), None, Some(3)),
                RecordValueBegin(None, sval::Label::new("field_0")),
                I32(1),
                RecordValueEnd(None, sval::Label::new("field_0")),
                RecordValueBegin(None, sval::Label::new("field_1")),
                Bool(true),
                RecordValueEnd(None, sval::Label::new("field_1")),
                RecordValueBegin(None, sval::Label::new("field_2")),
                TextBegin(Some(1)),
                TextFragmentComputed("a".into()),
                TextEnd,
                RecordValueEnd(None, sval::Label::new("field_2")),
                RecordEnd(None, Some(sval::Label::new("MapStruct")), None),
            ]
        },
    );
}

#[test]
fn seq_struct_named_to_serialize() {
    test_case(
        SeqStruct(1, true, "a"),
        {
            use serde_test::Token::*;

            &[
                TupleStruct {
                    name: "SeqStruct",
                    len: 3,
                },
                I32(1),
                Bool(true),
                Str("a"),
                TupleStructEnd,
            ]
        },
        {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(sval::Label::new("SeqStruct")), None, Some(3)),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(1),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleValueBegin(None, sval::Index::new(1)),
                Bool(true),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleValueBegin(None, sval::Index::new(2)),
                TextBegin(Some(1)),
                TextFragmentComputed("a".into()),
                TextEnd,
                TupleValueEnd(None, sval::Index::new(2)),
                TupleEnd(None, Some(sval::Label::new("SeqStruct")), None),
            ]
        },
    );
}

#[test]
fn seq_struct_unnamed_to_serialize() {
    test_case(
        (1, true, "a"),
        {
            use serde_test::Token::*;

            &[Tuple { len: 3 }, I32(1), Bool(true), Str("a"), TupleEnd]
        },
        {
            use sval_test::Token::*;

            &[
                TupleBegin(None, None, None, Some(3)),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(1),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleValueBegin(None, sval::Index::new(1)),
                Bool(true),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleValueBegin(None, sval::Index::new(2)),
                TextBegin(Some(1)),
                TextFragmentComputed("a".into()),
                TextEnd,
                TupleValueEnd(None, sval::Index::new(2)),
                TupleEnd(None, None, None),
            ]
        },
    );
}

#[test]
fn tagged_struct_to_serialize() {
    test_case(
        Tagged(1),
        {
            use serde_test::Token::*;

            &[NewtypeStruct { name: "Tagged" }, I32(1)]
        },
        {
            use sval_test::Token::*;

            &[
                TaggedBegin(None, Some(sval::Label::new("Tagged")), None),
                I32(1),
                TaggedEnd(None, Some(sval::Label::new("Tagged")), None),
            ]
        },
    )
}

#[test]
fn enum_tag_to_serialize() {
    test_case(
        Enum::Constant,
        {
            use serde_test::Token::*;

            &[UnitVariant {
                name: "Enum",
                variant: "Constant",
            }]
        },
        {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                Tag(
                    None,
                    Some(sval::Label::new("Constant")),
                    Some(sval::Index::new(0)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        },
    );
}

#[test]
fn enum_tagged_to_serialize() {
    test_case(
        Enum::Tagged(1),
        {
            use serde_test::Token::*;

            &[
                NewtypeVariant {
                    name: "Enum",
                    variant: "Tagged",
                },
                I32(1),
            ]
        },
        {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                TaggedBegin(
                    None,
                    Some(sval::Label::new("Tagged")),
                    Some(sval::Index::new(1)),
                ),
                I32(1),
                TaggedEnd(
                    None,
                    Some(sval::Label::new("Tagged")),
                    Some(sval::Index::new(1)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        },
    );
}

#[test]
fn enum_record_to_serialize() {
    test_case(
        Enum::MapStruct {
            field_0: 1,
            field_1: true,
            field_2: "a",
        },
        {
            use serde_test::Token::*;

            &[
                StructVariant {
                    name: "Enum",
                    variant: "MapStruct",
                    len: 3,
                },
                Str("field_0"),
                I32(1),
                Str("field_1"),
                Bool(true),
                Str("field_2"),
                Str("a"),
                StructVariantEnd,
            ]
        },
        {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                RecordBegin(
                    None,
                    Some(sval::Label::new("MapStruct")),
                    Some(sval::Index::new(2)),
                    Some(3),
                ),
                RecordValueBegin(None, sval::Label::new("field_0")),
                I32(1),
                RecordValueEnd(None, sval::Label::new("field_0")),
                RecordValueBegin(None, sval::Label::new("field_1")),
                Bool(true),
                RecordValueEnd(None, sval::Label::new("field_1")),
                RecordValueBegin(None, sval::Label::new("field_2")),
                TextBegin(Some(1)),
                TextFragmentComputed("a".into()),
                TextEnd,
                RecordValueEnd(None, sval::Label::new("field_2")),
                RecordEnd(
                    None,
                    Some(sval::Label::new("MapStruct")),
                    Some(sval::Index::new(2)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        },
    );
}

#[test]
fn enum_tuple_to_serialize() {
    test_case(
        Enum::SeqStruct(1, true, "a"),
        {
            use serde_test::Token::*;

            &[
                TupleVariant {
                    name: "Enum",
                    variant: "SeqStruct",
                    len: 3,
                },
                I32(1),
                Bool(true),
                Str("a"),
                TupleVariantEnd,
            ]
        },
        {
            use sval_test::Token::*;

            &[
                EnumBegin(None, Some(sval::Label::new("Enum")), None),
                TupleBegin(
                    None,
                    Some(sval::Label::new("SeqStruct")),
                    Some(sval::Index::new(3)),
                    Some(3),
                ),
                TupleValueBegin(None, sval::Index::new(0)),
                I32(1),
                TupleValueEnd(None, sval::Index::new(0)),
                TupleValueBegin(None, sval::Index::new(1)),
                Bool(true),
                TupleValueEnd(None, sval::Index::new(1)),
                TupleValueBegin(None, sval::Index::new(2)),
                TextBegin(Some(1)),
                TextFragmentComputed("a".into()),
                TextEnd,
                TupleValueEnd(None, sval::Index::new(2)),
                TupleEnd(
                    None,
                    Some(sval::Label::new("SeqStruct")),
                    Some(sval::Index::new(3)),
                ),
                EnumEnd(None, Some(sval::Label::new("Enum")), None),
            ]
        },
    );
}
