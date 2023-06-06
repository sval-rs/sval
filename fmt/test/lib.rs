#![cfg(test)]

#[macro_use]
extern crate sval_derive;

use std::fmt;
use std::fmt::Display;
use sval::{Stream, Tag};

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

    impl sval_fmt::TokenWrite for Writer {
        fn write_null(&mut self) -> core::fmt::Result {
            self.null = true;
            Ok(())
        }

        fn write_text(&mut self, _: &str) -> core::fmt::Result {
            self.text = true;
            Ok(())
        }

        fn write_number<N: fmt::Display>(&mut self, _: N) -> fmt::Result {
            self.number = true;
            Ok(())
        }

        fn write_bool(&mut self, _: bool) -> core::fmt::Result {
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

struct DefaultWriter<S>(S);

impl<'sval, S: sval::Stream<'sval>> fmt::Write for DefaultWriter<S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sval_fmt::stream_display_to_text_fragments(&mut self.0, s).map_err(|_| fmt::Error)
    }
}

impl<'sval, S: sval::Stream<'sval>> sval_fmt::TokenWrite for DefaultWriter<S> {
    fn write_token_fragment<T: Display>(&mut self, tag: &Tag, token: T) -> fmt::Result {
        sval_fmt::stream_display_to_tagged_text_fragments(&mut self.0, tag, token)
            .map_err(|_| fmt::Error)
    }
}

struct TemplateWrite<S>(S);

impl<'sval, S: Stream<'sval>> sval_fmt::TokenWrite for TemplateWrite<S> {
    fn write_token_fragment<T: Display>(&mut self, tag: &Tag, token: T) -> fmt::Result {
        sval_fmt::stream_display_to_tagged_text_fragments(&mut self.0, tag, token)
            .map_err(|_| fmt::Error)
    }

    fn write_text_quote(&mut self) -> fmt::Result {
        Ok(())
    }

    fn text_escaper(&self) -> sval_fmt::TextEscaper {
        sval_fmt::TextEscaper::no_escaping()
    }
}

impl<'sval, S: Stream<'sval>> fmt::Write for TemplateWrite<S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sval_fmt::stream_display_to_text_fragments(&mut self.0, s).map_err(|_| fmt::Error)
    }
}

#[test]
fn stream_fragments_to_tokens_default() {
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

    sval_test::assert_tokens(
        &Template {
            pre: "before ",
            value: MapStruct {
                field_0: 42,
                field_1: true,
                field_2: "text \"in quotes\"",
            },
            post: " after",
        },
        {
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
        },
    );
}

#[test]
fn stream_fragments_to_tokens_custom() {
    struct Template<V> {
        pre: &'static str,
        value: V,
        post: &'static str,
    }

    impl<V: sval::Value> sval::Value for Template<V> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
            stream.text_begin(None)?;
            stream.text_fragment(self.pre)?;
            sval_fmt::stream_to_token_write(TemplateWrite(&mut *stream), &self.value)
                .map_err(|_| sval::Error::new())?;
            stream.text_fragment(self.post)?;
            stream.text_end()
        }
    }

    sval_test::assert_tokens(
        &Template {
            pre: "before ",
            value: MapStruct {
                field_0: 42,
                field_1: true,
                field_2: "text \"in quotes\"",
            },
            post: " after",
        },
        {
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
                TaggedTextFragmentComputed(sval_fmt::tags::TEXT, "text \"in quotes\"".to_owned()),
                TaggedTextFragmentComputed(sval_fmt::tags::WS, " ".to_owned()),
                TaggedTextFragmentComputed(sval_fmt::tags::PUNCT, "}".to_owned()),
                TextFragment(" after"),
                TextEnd,
            ]
        },
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
