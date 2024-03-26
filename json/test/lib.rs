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

    let actual_json_str = sval_json::stream_to_json_str(&v).unwrap();
    let rountrip_json_str = sval_json::stream_to_string(&actual_json_str).unwrap();

    assert_eq!(expected, actual_string);
    assert_eq!(expected, actual_bytes);
    assert_eq!(expected, actual_json_str.as_str());
    assert_eq!(expected, rountrip_json_str);
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
struct NestedMap {
    field_0: i32,
    field_1: bool,
}

#[derive(Value, Serialize)]
struct EmptyMap {}

#[derive(Value, Serialize)]
struct Tagged<T>(T);

#[derive(Value, Serialize)]
struct UnitStruct;

#[derive(Clone, Value, Serialize)]
enum Enum<F0, F1> {
    Constant,
    Tagged(F0),
    MapStruct { field_0: F0, field_1: F1 },
    EmptyMapStruct,
    SeqStruct(F0, F1),
    EmptySeq(&'static [i32]),
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
fn stream_native_text_nested() {
    #[derive(Value)]
    struct MapStruct<'a> {
        #[sval(data_tag = "sval_json::tags::JSON_TEXT")]
        text: &'a str,
    }

    assert_stream("{\"text\":\"a\nb\"}", MapStruct { text: "a\nb" });
}

#[test]
fn stream_native_number_nested() {
    #[derive(Value)]
    struct MapStruct<'a> {
        #[sval(data_tag = "sval_json::tags::JSON_NUMBER")]
        number: &'a str,
    }

    assert_stream("{\"number\":123.456}", MapStruct { number: "123.456" });
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
fn stream_unit_struct() {
    // NOTE: This is an incompatibility with `serde_json`
    // In `serde_json` this would produce `null`
    assert_stream("\"UnitStruct\"", UnitStruct);
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

    assert_json(MapStruct {
        field_0: EmptyMap {},
        field_1: EmptyMap {},
    });

    assert_json(MapStruct {
        field_0: &[] as &[i32],
        field_1: &[] as &[i32],
    });

    assert_json(MapStruct {
        field_0: NestedMap {
            field_0: 42,
            field_1: true,
        },
        field_1: NestedMap {
            field_0: 43,
            field_1: false,
        },
    });
}

#[test]
fn stream_seq_struct() {
    assert_json(SeqStruct(42, true));
    assert_json(SeqStruct("Hello", 1.3));
    assert_json((42, true));

    #[derive(Value, Serialize)]
    struct NestedMap {
        field_0: i32,
        field_1: bool,
    }

    assert_json((
        NestedMap {
            field_0: 42,
            field_1: true,
        },
        NestedMap {
            field_0: 43,
            field_1: false,
        },
    ));

    assert_json((EmptyMap {}, EmptyMap {}));

    assert_json((&[] as &[i32], &[] as &[i32]));
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
        Enum::EmptyMapStruct,
        Enum::SeqStruct(42, true),
        Enum::EmptySeq(&[]),
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
    #[sval(dynamic)]
    enum Dynamic<'a> {
        Constant,
        Null(sval::Null),
        Text(&'a str),
        Number(f64),
        Boolean(bool),
        Array(&'a [Dynamic<'a>]),
    }

    assert_eq!(
        "\"Constant\"",
        sval_json::stream_to_string(Dynamic::Constant).unwrap()
    );
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
        sval_json::stream_to_string(Dynamic::Null(sval::Null)).unwrap()
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
fn stream_empty_enum() {
    struct Enum;

    impl sval::Value for Enum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;
            stream.enum_end(None, Some(&sval::Label::new("Enum")), None)
        }
    }

    assert_eq!("\"Enum\"", sval_json::stream_to_string(Enum).unwrap());
}

#[test]
fn stream_unlabeled_tag() {
    struct Tag;

    impl sval::Value for Tag {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.tag(None, None, Some(&sval::Index::new(1)))
        }
    }

    assert_eq!("1", sval_json::stream_to_string(Tag).unwrap());
}

#[test]
fn stream_unlabeled_tag_variant() {
    struct Enum;

    impl sval::Value for Enum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;
            stream.tag(None, None, Some(&sval::Index::new(1)))?;
            stream.enum_end(None, Some(&sval::Label::new("Enum")), None)
        }
    }

    assert_eq!("1", sval_json::stream_to_string(Enum).unwrap());
}

#[test]
fn stream_unlabeled_unindexed_tag() {
    struct Tag;

    impl sval::Value for Tag {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.tag(None, None, None)
        }
    }

    assert_eq!("null", sval_json::stream_to_string(Tag).unwrap());
}

#[test]
fn stream_unlabeled_empty_enum() {
    struct Enum;

    impl sval::Value for Enum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(None, None, Some(&sval::Index::new(1)))?;
            stream.enum_end(None, None, Some(&sval::Index::new(1)))
        }
    }

    assert_eq!("1", sval_json::stream_to_string(Enum).unwrap());
}

#[test]
fn stream_unlabeled_unindexed_empty_enum() {
    struct Enum;

    impl sval::Value for Enum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(None, None, None)?;
            stream.enum_end(None, None, None)
        }
    }

    assert_eq!("null", sval_json::stream_to_string(Enum).unwrap());
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
fn stream_exotic_nested_enum_empty() {
    // Outer::Inner
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

            stream.enum_end(
                None,
                Some(&sval::Label::new("Inner")),
                Some(&sval::Index::new(1)),
            )?;

            stream.enum_end(None, Some(&sval::Label::new("Outer")), None)
        }
    }

    assert_eq!(
        "{\"Inner\":\"Inner\"}",
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
