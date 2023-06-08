#![cfg(test)]

#[macro_use]
extern crate sval_derive;

use std::fmt::{self, Display};
use sval::{Stream, Tag};
use sval_fmt::TokenWrite;

fn assert_fmt(v: impl sval::Value + fmt::Debug) {
    let expected = format!("{:?}", v);
    let actual_debug = format!("{:?}", sval_fmt::ToFmt::new(&v));
    let actual_display = format!("{}", sval_fmt::ToFmt::new(&v));

    assert_eq!(expected, actual_debug);
    assert_eq!(expected, actual_display);

    let to_debug = sval_fmt::DebugToValue::new(&v);
    let buffered = sval_buffer::TextBuf::collect(&to_debug).unwrap();

    assert_eq!(expected, buffered.as_str());
}

#[derive(Value, Debug)]
#[sval(tag = "sval::tags::NUMBER")]
struct Number(&'static str);

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
    assert_eq!("0042", format!("{:>04?}", sval_fmt::ToFmt::new(42i64)));
}

#[test]
fn debug_primitive() {
    assert_fmt(42i32);
    assert_fmt(true);
    assert_fmt("a string");
}

#[test]
fn debug_option() {
    assert_fmt(Some(42i32));
    assert_fmt(None::<i32>);
}

#[test]
fn debug_map_struct() {
    assert_fmt(MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });
}

#[test]
fn debug_seq_struct() {
    assert_fmt(SeqStruct(42, true, "Hello"));
    assert_fmt((42, true, "Hello"));
}

#[test]
fn debug_tagged() {
    assert_fmt(Tagged(42));
}

#[test]
fn debug_enum() {
    assert_fmt(Enum::Constant);

    assert_fmt(Enum::MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });

    assert_fmt(Enum::SeqStruct(42, true, "Hello"));

    assert_fmt(Enum::Tagged(42));
}

#[test]
fn debug_unit() {
    assert_fmt(());
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
            sval_fmt::ToFmt::new(&UnnamedRecord {
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

    assert_eq!("Variant", format!("{:?}", sval_fmt::ToFmt::new(NestedEnum)));
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
        format!("{:?}", sval_fmt::ToFmt::new(UntaggedEnum::I32(42)))
    );
}

#[test]
fn token_write() {
    #[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
    struct Writer {
        null: bool,
        bool: bool,
        text: bool,
        number: bool,
    }

    impl fmt::Write for Writer {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    impl TokenWrite for Writer {
        fn write_null(&mut self) -> fmt::Result {
            self.null = true;
            Ok(())
        }

        fn write_text(&mut self, _: &str) -> fmt::Result {
            self.text = true;
            Ok(())
        }

        fn write_number<N: Display>(&mut self, _: N) -> fmt::Result {
            self.number = true;
            Ok(())
        }

        fn write_bool(&mut self, _: bool) -> fmt::Result {
            self.bool = true;
            Ok(())
        }
    }

    let mut writer = Writer::default();
    sval_fmt::stream_to_token_write(&mut writer, 42).unwrap();
    assert!(writer.number);

    let mut writer = Writer::default();
    sval_fmt::stream_to_token_write(&mut writer, Number("436543.457656765")).unwrap();
    assert!(writer.number);

    let mut writer = Writer::default();
    sval_fmt::stream_to_token_write(&mut writer, true).unwrap();
    assert!(writer.bool);

    let mut writer = Writer::default();
    sval_fmt::stream_to_token_write(&mut writer, "a string").unwrap();
    assert!(writer.text);

    let mut writer = Writer::default();
    sval_fmt::stream_to_token_write(&mut writer, ()).unwrap();
    assert!(writer.null);
}

struct NoEscapeWriter<W>(W);

impl<W: fmt::Write> fmt::Write for NoEscapeWriter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<W: fmt::Write> TokenWrite for NoEscapeWriter<W> {
    fn write_text_quote(&mut self) -> fmt::Result {
        Ok(())
    }

    fn write_tagged_text(&mut self, tag: &Tag, text: &str) -> fmt::Result {
        self.write_token_fragment(tag, text)
    }
}

struct CompactWriter<W>(W);

impl<W: fmt::Write> fmt::Write for CompactWriter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<W: fmt::Write> TokenWrite for CompactWriter<W> {
    fn write_ws(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }
}

#[test]
fn stream_token_write_in_value() {
    struct DefaultWriter<S>(S);

    impl<'sval, S: Stream<'sval>> fmt::Write for DefaultWriter<S> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.text_fragment_computed(s).map_err(|_| fmt::Error)
        }
    }

    impl<'sval, S: Stream<'sval>> TokenWrite for DefaultWriter<S> {
        fn write_token_fragment<T: Display>(&mut self, tag: &Tag, token: T) -> fmt::Result {
            sval::stream_tagged_display_fragments(&mut self.0, tag, token).map_err(|_| fmt::Error)
        }
    }

    struct Template<V> {
        pre: &'static str,
        value: V,
        post: &'static str,
    }

    impl<V: sval::Value> sval::Value for Template<V> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
            stream.text_begin(None)?;
            stream.text_fragment(self.pre)?;
            sval_fmt::stream_to_token_write(DefaultWriter(&mut *stream), &self.value)
                .map_err(|_| sval::Error::new())?;
            stream.text_fragment(self.post)?;
            stream.text_end()
        }
    }

    let template = Template {
        pre: "before ",
        value: MapStruct {
            field_0: 42,
            field_1: true,
            field_2: "text \"in quotes\"",
        },
        post: " after",
    };

    sval_test::assert_tokens(&template, {
        use sval_test::Token::*;

        &[
            TextBegin(None),
            TextFragment("before "),
            TaggedTextFragmentComputed(sval_fmt::tags::IDENT, "MapStruct".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, "{".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::IDENT, "field_0".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, ":".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::NUMBER, "42".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, ",".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::IDENT, "field_1".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, ":".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::ATOM, "true".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, ",".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::IDENT, "field_2".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, ":".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "\"".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "text ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "\\".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "\"".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "in quotes".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "\\".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "\"".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "\"".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
            TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, "}".to_owned()),
            TextFragment(" after"),
            TextEnd,
        ]
    });
}

#[test]
fn stream_token_write_compact() {
    struct Writer<W>(W);

    impl<W: fmt::Write> fmt::Write for Writer<W> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.write_str(s)
        }
    }

    impl<W: fmt::Write> TokenWrite for Writer<W> {
        fn write_ws(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut buf = String::new();
    sval_fmt::stream_to_token_write(
        Writer(&mut buf),
        MapStruct {
            field_0: 42,
            field_1: true,
            field_2: "text \"in quotes\"",
        },
    )
    .unwrap();

    assert_eq!(
        r#"MapStruct{field_0:42,field_1:true,field_2:"text \"in quotes\""}"#,
        buf
    );
}

#[test]
fn stream_token_write_no_escaping() {
    struct Writer<W>(W);

    impl<W: fmt::Write> fmt::Write for Writer<W> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.write_str(s)
        }
    }

    impl<W: fmt::Write> TokenWrite for Writer<W> {
        fn write_tagged_text(&mut self, tag: &Tag, text: &str) -> fmt::Result {
            self.write_token_fragment(tag, text)
        }

        fn write_text_quote(&mut self) -> fmt::Result {
            Ok(())
        }
    }

    let mut buf = String::new();
    sval_fmt::stream_to_token_write(
        Writer(&mut buf),
        MapStruct {
            field_0: 42,
            field_1: true,
            field_2: "text \"in quotes\"",
        },
    )
    .unwrap();

    assert_eq!(
        r#"MapStruct { field_0: 42, field_1: true, field_2: text "in quotes" }"#,
        buf
    );
}

#[test]
fn stream_token_write_indented() {
    struct Writer<W> {
        inner: W,
        indent: usize,
    }

    impl<W: fmt::Write> Writer<W> {
        fn write_indent(&mut self) -> fmt::Result {
            for _ in 0..self.indent {
                self.write_ws(" ")?;
            }

            Ok(())
        }
    }

    impl<W: fmt::Write> fmt::Write for Writer<W> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.inner.write_str(s)
        }
    }

    impl<W: fmt::Write> TokenWrite for Writer<W> {
        fn write_map_begin(&mut self) -> fmt::Result {
            self.indent += 4;

            self.write_punct("{")
        }

        fn write_map_key_begin(&mut self, is_first: bool) -> fmt::Result {
            if !is_first {
                self.write_punct(",")?;
            }

            self.write_ws("\n")?;
            self.write_indent()
        }

        fn write_map_end(&mut self, is_empty: bool) -> fmt::Result {
            self.indent -= 4;

            if !is_empty {
                self.write_punct(",")?;
                self.write_ws("\n")?;
                self.write_indent()?;
            }

            self.write_punct("}")
        }

        fn write_seq_begin(&mut self) -> fmt::Result {
            self.write_punct("[")?;

            self.indent += 4;

            Ok(())
        }

        fn write_seq_value_begin(&mut self, is_first: bool) -> fmt::Result {
            if !is_first {
                self.write_punct(",")?;
            }

            self.write_ws("\n")?;
            self.write_indent()
        }

        fn write_seq_end(&mut self, is_empty: bool) -> fmt::Result {
            self.indent -= 4;

            if !is_empty {
                self.write_punct(",")?;
                self.write_ws("\n")?;
                self.write_indent()?;
            }

            self.write_punct("]")
        }

        fn write_record_begin(&mut self) -> fmt::Result {
            self.indent += 4;

            self.write_punct("{")
        }

        fn write_record_value_begin(&mut self, field: &str, is_first: bool) -> fmt::Result {
            if !is_first {
                self.write_punct(",")?;
            }

            self.write_ws("\n")?;
            self.write_indent()?;

            self.write_ident(field)?;
            self.write_punct(":")?;
            self.write_ws(" ")
        }

        fn write_record_end(&mut self, is_empty: bool) -> fmt::Result {
            self.indent -= 4;

            if !is_empty {
                self.write_punct(",")?;
                self.write_ws("\n")?;
                self.write_indent()?;
            }

            self.write_punct("}")
        }

        fn write_tuple_begin(&mut self) -> fmt::Result {
            self.indent += 4;

            self.write_punct("(")
        }

        fn write_tuple_value_begin(&mut self, is_first: bool) -> fmt::Result {
            if !is_first {
                self.write_punct(",")?;
            }

            self.write_ws("\n")?;
            self.write_indent()
        }

        fn write_tuple_end(&mut self, is_empty: bool) -> fmt::Result {
            self.indent -= 4;

            if !is_empty {
                self.write_punct(",")?;
                self.write_ws("\n")?;
                self.write_indent()?;
            }

            self.write_punct(")")
        }
    }

    let mut buf = String::new();
    sval_fmt::stream_to_token_write(
        Writer {
            inner: &mut buf,
            indent: 0,
        },
        MapStruct {
            field_0: 42,
            field_1: true,
            field_2: "text \"in quotes\"",
        },
    )
    .unwrap();

    assert_eq!(
        r#"MapStruct {
    field_0: 42,
    field_1: true,
    field_2: "text \"in quotes\"",
}"#,
        buf
    );
}

#[test]
fn failing_value_does_not_panic_to_string() {
    struct Kaboom;

    impl sval::Value for Kaboom {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, _: &mut S) -> sval::Result {
            Err(sval::Error::new())
        }
    }

    #[derive(Value)]
    struct NestedKaboom {
        a: i32,
        b: Kaboom,
        c: i32,
    }

    assert_eq!(
        "<an error occurred when formatting an argument>",
        sval_fmt::ToFmt::new(Kaboom).to_string()
    );
    assert_eq!(
        "NestedKaboom { a: 1, b: <an error occurred when formatting an argument>",
        sval_fmt::ToFmt::new(NestedKaboom {
            a: 1,
            b: Kaboom,
            c: 2
        })
        .to_string()
    );
}
