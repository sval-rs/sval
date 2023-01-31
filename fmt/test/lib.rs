#![cfg(test)]

#[macro_use]
extern crate sval_derive;

use std::fmt;

fn assert_debug(v: impl sval::Value + fmt::Debug) {
    let expected = format!("{:?}", v);
    let actual = format!("{:?}", sval_fmt::to_debug(v));

    assert_eq!(expected, actual);
}

#[derive(Value, Debug)]
struct MapStruct {
    field_0: i32,
    field_1: bool,
    field_2: &'static str,
}

#[derive(Value, Debug)]
struct SeqStruct(i32, bool, &'static str);

#[derive(Value, Debug)]
struct Tagged(i32);

#[derive(Value, Debug)]
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

#[test]
fn debug_retains_flags() {
    assert_eq!("0042", format!("{:>04?}", sval_fmt::to_debug(42i64)));
}

#[test]
fn debug_primitive() {
    assert_debug(42i32);
}

#[test]
fn debug_option() {
    assert_debug(Some(42i32));
    assert_debug(None::<i32>);
}

#[test]
fn debug_map_struct() {
    assert_debug(MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });
}

#[test]
fn debug_seq_struct() {
    assert_debug(SeqStruct(42, true, "Hello"));
    assert_debug((42, true, "Hello"));
}

#[test]
fn debug_tagged() {
    assert_debug(Tagged(42));
}

#[test]
fn debug_enum() {
    assert_debug(Enum::Constant);

    assert_debug(Enum::MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });

    assert_debug(Enum::SeqStruct(42, true, "Hello"));

    assert_debug(Enum::Tagged(42));
}

#[test]
fn debug_unit() {
    assert_debug(());
}

#[test]
fn debug_exotic_record() {
    struct UnnamedRecord {
        field_0: i32,
        field_1: bool,
        field_2: &'static str,
    }

    impl sval::Value for UnnamedRecord {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.record_begin(None, None, None, Some(3))?;

            stream.record_value_begin(None, &sval::Label::new("field_0"))?;
            stream.value(&self.field_0)?;
            stream.record_value_end(None, &sval::Label::new("field_0"))?;

            stream.record_value_begin(None, &sval::Label::new("field_1"))?;
            stream.value(&self.field_1)?;
            stream.record_value_end(None, &sval::Label::new("field_1"))?;

            stream.record_value_begin(None, &sval::Label::new("field_2"))?;
            stream.value(&self.field_2)?;
            stream.record_value_end(None, &sval::Label::new("field_2"))?;

            stream.record_end(None, None, None)
        }
    }

    assert_eq!(
        "{ field_0: 42, field_1: true, field_2: \"Hello\" }",
        format!(
            "{:?}",
            sval_fmt::to_debug(&UnnamedRecord {
                field_0: 42,
                field_1: true,
                field_2: "Hello",
            })
        )
    );
}

#[test]
fn debug_exotic_nested_enum() {
    // Outer::Inner::Variant
    struct NestedEnum;

    impl sval::Value for NestedEnum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(None, Some(&sval::Label::new("Outer")), None)?;

            stream.enum_begin(
                None,
                Some(&sval::Label::new("Inner")),
                Some(&sval::Index::new(1)),
            )?;

            stream.tag(
                None,
                Some(&sval::Label::new("Variant")),
                Some(&sval::Index::new(0)),
            )?;

            stream.enum_end(
                None,
                Some(&sval::Label::new("Inner")),
                Some(&sval::Index::new(1)),
            )?;

            stream.enum_end(None, Some(&sval::Label::new("Outer")), None)
        }
    }

    assert_eq!("Variant", format!("{:?}", sval_fmt::to_debug(NestedEnum)));
}

#[test]
fn debug_exotic_unnamed_enum() {
    // i32 | bool
    enum UntaggedEnum {
        I32(i32),
        #[allow(dead_code)]
        Bool(bool),
    }

    impl sval::Value for UntaggedEnum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(None, None, None)?;

            match self {
                UntaggedEnum::I32(v) => {
                    stream.tagged_begin(None, None, None)?;
                    stream.value(v)?;
                    stream.tagged_end(None, None, None)?;
                }
                UntaggedEnum::Bool(v) => {
                    stream.tagged_begin(None, None, None)?;
                    stream.value(v)?;
                    stream.tagged_end(None, None, None)?;
                }
            }

            stream.enum_end(None, None, None)
        }
    }

    assert_eq!(
        "42",
        format!("{:?}", sval_fmt::to_debug(UntaggedEnum::I32(42)))
    );
}
