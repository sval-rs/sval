#![cfg(test)]

#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

fn assert_json(v: impl sval::Value + serde::Serialize) {
    let expected = format!("{}", serde_json::to_string(&v).unwrap());
    let actual = format!("{}", sval_json::stream_to_string(&v).unwrap());

    assert_eq!(expected, actual);
}

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

#[test]
fn stream_primitive() {
    assert_json(42i32);
}

#[test]
fn stream_option() {
    assert_json(Some(42i32));
    assert_json(None::<i32>);
}

#[test]
fn stream_map_struct() {
    assert_json(MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });
}

#[test]
fn stream_seq_struct() {
    assert_json(SeqStruct(42, true, "Hello"));
    assert_json((42, true, "Hello"));
}

#[test]
fn stream_tagged() {
    assert_json(Tagged(42));
}

#[test]
fn stream_enum() {
    assert_json(Enum::Constant);

    assert_json(Enum::MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });

    assert_json(Enum::SeqStruct(42, true, "Hello"));

    assert_json(Enum::Tagged(42));
}

#[test]
fn stream_unit() {
    assert_json(());
}

#[test]
fn stream_exotic_record() {
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
        "{\"field_0\":42,\"field_1\":true,\"field_2\":\"Hello\"}",
        format!(
            "{}",
            sval_json::stream_to_string(&UnnamedRecord {
                field_0: 42,
                field_1: true,
                field_2: "Hello",
            })
            .unwrap()
        )
    );
}

#[test]
fn stream_exotic_nested_enum() {
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

    assert_eq!(
        "\"Variant\"",
        format!("{}", sval_json::stream_to_string(NestedEnum).unwrap())
    );
}

#[test]
fn stream_exotic_unnamed_enum() {
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
        format!(
            "{}",
            sval_json::stream_to_string(UntaggedEnum::I32(42)).unwrap()
        )
    );
}

#[test]
fn stream_to_io() {
    let mut buf = Vec::new();

    sval_json::stream_to_writer(
        &mut buf,
        MapStruct {
            field_0: 42,
            field_1: true,
            field_2: "a",
        },
    )
    .unwrap();

    assert_eq!(
        "{\"field_0\":42,\"field_1\":true,\"field_2\":\"a\"}",
        String::from_utf8(buf).unwrap()
    );
}
