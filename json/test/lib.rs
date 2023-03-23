#![cfg(test)]

#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

fn assert_json(v: impl sval::Value + serde::Serialize) {
    let expected = serde_json::to_string(&v).unwrap();

    assert_stream(&expected, v);
}

fn assert_stream(expected: &str, v: impl sval::Value) {
    let actual_string = sval_json::stream_to_string(&v).unwrap();
    let actual_bytes = String::from_utf8(sval_json::stream_to_vec(&v).unwrap()).unwrap();

    assert_eq!(expected, actual_string);
    assert_eq!(expected, actual_bytes);
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

#[derive(Clone, Value, Serialize)]
enum Enum {
    Constant,
    Tagged(i32),
    MapStruct {
        field_0: i32,
        field_1: bool,
        field_2: &'static str,
    },
    SeqStruct(i32, bool, &'static str),
    Nested(Box<Enum>),
}

#[derive(Value)]
#[sval(tag = "sval::tags::NUMBER")]
struct Number<T>(T);

#[derive(Value)]
#[sval(tag = "sval_json::tags::JSON_NUMBER")]
struct JsonNumber<T>(T);

#[derive(Value)]
#[sval(tag = "sval_json::tags::JSON_TEXT")]
struct JsonText<T>(T);

#[test]
fn stream_primitive() {
    assert_json(42i32);
    assert_json(true);
    assert_json(false);
    assert_json("abc");
    assert_json("a\nb");
}

#[test]
fn stream_native_number() {
    for (num, expected) in [
        ("0", "0"),
        ("000", "0"),
        ("001", "1"),
        ("-1", "-1"),
        ("+1", "1"),
        ("1.234", "1.234"),
        ("1.200", "1.200"),
        ("123.456e789", "123.456e789"),
        ("-123.456e789", "-123.456e789"),
        ("-123.456e-789", "-123.456e-789"),
        ("+123.456e789", "123.456e789"),
        ("+123.456e+789", "123.456e+789"),
        ("+00123.456e+789", "123.456e+789"),
        (
            "84621599231797873982892348534.48343235787975583989258932",
            "84621599231797873982892348534.48343235787975583989258932",
        ),
    ] {
        assert_stream(expected, Number(num));
        assert_stream(num, JsonNumber(Number(num)));
        assert_stream(num, Number(JsonNumber(num)));
        assert_stream(num, JsonNumber(num));
    }
}

#[test]
fn stream_native_text() {
    for str in ["abc", "a\nb"] {
        let expected = format!("\"{}\"", str);

        assert_stream(&expected, JsonText(str));
    }
}

#[test]
fn stream_option() {
    assert_json(Some(42i32));
    assert_json(None::<i32>);
}

#[test]
fn stream_unit() {
    assert_json(());
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
    for variant in [
        Enum::Constant,
        Enum::MapStruct {
            field_0: 42,
            field_1: true,
            field_2: "Hello",
        },
        Enum::SeqStruct(42, true, "Hello"),
        Enum::Tagged(42),
    ] {
        assert_json(&variant);

        assert_json(Enum::Nested(Box::new(variant.clone())));

        assert_json(Enum::Nested(Box::new(Enum::Nested(Box::new(
            variant.clone(),
        )))));
    }
}

#[test]
fn stream_exotic_record() {
    // { field_0: 42, field_1: true, field_2: "Hello" }
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
fn stream_exotic_nested_enum_tag() {
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
        "{\"Inner\":\"Variant\"}",
        format!("{}", sval_json::stream_to_string(NestedEnum).unwrap())
    );
}

#[test]
fn stream_exotic_nested_enum_record() {
    // Outer::Inner::Variant { a: 42 }
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

            stream.record_begin(
                None,
                Some(&sval::Label::new("Variant")),
                Some(&sval::Index::new(0)),
                None,
            )?;

            stream.record_value_begin(None, &sval::Label::new("a"))?;
            stream.i32(42)?;
            stream.record_value_end(None, &sval::Label::new("a"))?;

            stream.record_end(
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
        "{\"Inner\":{\"Variant\":{\"a\":42}}}",
        format!("{}", sval_json::stream_to_string(NestedEnum).unwrap())
    );
}

#[test]
fn stream_exotic_unnamed_enum() {
    // (i32 | bool)
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
fn stream_exotic_unnamed_nested_enum_record() {
    // { a: 42 }
    struct NestedEnum;

    impl sval::Value for NestedEnum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(None, None, None)?;

            stream.enum_begin(None, None, None)?;

            stream.record_begin(None, None, None, None)?;

            stream.record_value_begin(None, &sval::Label::new("a"))?;
            stream.i32(42)?;
            stream.record_value_end(None, &sval::Label::new("a"))?;

            stream.record_end(None, None, None)?;

            stream.enum_end(None, None, None)?;

            stream.enum_end(None, None, None)
        }
    }

    assert_eq!(
        "{\"a\":42}",
        format!("{}", sval_json::stream_to_string(NestedEnum).unwrap())
    );
}

#[test]
fn stream_to_io() {
    let mut buf = Vec::new();

    sval_json::stream_to_io_write(
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
