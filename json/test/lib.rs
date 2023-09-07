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

fn assert_valid(v: impl sval::Value) {
    let json = sval_json::stream_to_string(v).unwrap();

    let _: serde_json::Value = serde_json::from_str(&json).unwrap();
}

#[derive(Value, Serialize)]
struct MapStruct<F0, F1> {
    field_0: F0,
    field_1: F1,
}

#[derive(Value, Serialize)]
struct SeqStruct<F0, F1>(F0, F1);

#[derive(Value, Serialize)]
struct Tagged<T>(T);

#[derive(Clone, Value, Serialize)]
enum Enum<F0, F1> {
    Constant,
    Tagged(F0),
    MapStruct { field_0: F0, field_1: F1 },
    SeqStruct(F0, F1),
    Nested(Box<Enum<F0, F1>>),
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
    });

    assert_json(MapStruct {
        field_0: "Hello",
        field_1: 1.3,
    });
}

#[test]
fn stream_seq_struct() {
    assert_json(SeqStruct(42, true));
    assert_json(SeqStruct("Hello", 1.3));
    assert_json((42, true));
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
        },
        Enum::SeqStruct(42, true),
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
fn stream_untagged_enum() {
    #[derive(Value)]
    enum Null {}

    #[derive(Value)]
    #[sval(dynamic)]
    enum Dynamic<'a> {
        Null(Option<Null>),
        Text(&'a str),
        Number(f64),
        Boolean(bool),
        Array(&'a [Dynamic<'a>]),
    }

    assert_eq!(
        "\"Some text\"",
        sval_json::stream_to_string(Dynamic::Text("Some text")).unwrap()
    );
    assert_eq!(
        "3.14",
        sval_json::stream_to_string(Dynamic::Number(3.14)).unwrap()
    );
    assert_eq!(
        "true",
        sval_json::stream_to_string(Dynamic::Boolean(true)).unwrap()
    );
    assert_eq!(
        "null",
        sval_json::stream_to_string(Dynamic::Null(None)).unwrap()
    );
    assert_eq!(
        "[true,false]",
        sval_json::stream_to_string(Dynamic::Array(&[
            Dynamic::Boolean(true),
            Dynamic::Boolean(false),
        ]))
        .unwrap()
    );
}

#[test]
fn stream_externally_tagged_enum() {
    #[derive(Value)]
    struct Container {
        internally_tagged: Enum<bool, bool>,
        #[sval(flatten)]
        externally_tagged: Enum<bool, bool>,
    }

    assert_eq!(
        "{\"internally_tagged\":{\"Tagged\":true},\"Tagged\":true}",
        sval_json::stream_to_string(Container {
            internally_tagged: Enum::Tagged(true),
            externally_tagged: Enum::Tagged(true)
        })
        .unwrap()
    );
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
        sval_json::stream_to_string(UnnamedRecord {
            field_0: 42,
            field_1: true,
            field_2: "Hello",
        })
        .unwrap()
    );

    assert_eq!(
        "{\"field_0\":{\"field_0\":42,\"field_1\":true,\"field_2\":\"Hello\"},\"field_1\":{\"field_0\":42,\"field_1\":true,\"field_2\":\"Hello\"}}",
        sval_json::stream_to_string(MapStruct {
            field_0: UnnamedRecord {
                field_0: 42,
                field_1: true,
                field_2: "Hello",
            },
            field_1: UnnamedRecord {
                field_0: 42,
                field_1: true,
                field_2: "Hello",
            },
        }).unwrap()
    );

    assert_valid(MapStruct {
        field_0: UnnamedRecord {
            field_0: 42,
            field_1: true,
            field_2: "Hello",
        },
        field_1: UnnamedRecord {
            field_0: 42,
            field_1: true,
            field_2: "Hello",
        },
    });

    assert_valid(SeqStruct(
        UnnamedRecord {
            field_0: 42,
            field_1: true,
            field_2: "Hello",
        },
        UnnamedRecord {
            field_0: 42,
            field_1: true,
            field_2: "Hello",
        },
    ));
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
        sval_json::stream_to_string(NestedEnum).unwrap(),
    );

    assert_valid(MapStruct {
        field_0: NestedEnum,
        field_1: NestedEnum,
    });

    assert_valid(SeqStruct(NestedEnum, NestedEnum));
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
        sval_json::stream_to_string(NestedEnum).unwrap(),
    );

    assert_valid(MapStruct {
        field_0: NestedEnum,
        field_1: NestedEnum,
    });

    assert_valid(SeqStruct(NestedEnum, NestedEnum));
}

#[test]
fn stream_exotic_unnamed_enum() {
    // (i32 | bool)
    enum UntaggedEnum {
        I32(i32),
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
        sval_json::stream_to_string(UntaggedEnum::I32(42)).unwrap(),
    );

    assert_valid(MapStruct {
        field_0: UntaggedEnum::I32(42),
        field_1: UntaggedEnum::Bool(true),
    });

    assert_valid(SeqStruct(UntaggedEnum::I32(42), UntaggedEnum::Bool(true)));
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
        sval_json::stream_to_string(NestedEnum).unwrap(),
    );

    assert_valid(MapStruct {
        field_0: NestedEnum,
        field_1: NestedEnum,
    });

    assert_valid(SeqStruct(NestedEnum, NestedEnum));
}

#[test]
fn stream_to_io() {
    let mut buf = Vec::new();

    sval_json::stream_to_io_write(
        &mut buf,
        MapStruct {
            field_0: 42,
            field_1: true,
        },
    )
    .unwrap();

    assert_eq!(
        "{\"field_0\":42,\"field_1\":true}",
        String::from_utf8(buf).unwrap()
    );
}
