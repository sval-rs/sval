use crate::flattener::{Flatten, Flattener};
use crate::label::{LabelBuf, LabelStream};
use sval::{Index, Label, Stream, Tag, Value};

pub fn flatten_to_map<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let stream = PassThru::new(stream);

    let mut stream = Flattener::begin(MapFlatten { stream }, offset);

    value.stream(&mut stream)?;

    Ok(stream.end())
}

struct MapFlatten<S> {
    stream: PassThru<S>,
}

impl<'sval, S: Stream<'sval>> Flatten<'sval> for MapFlatten<S> {
    type Stream = PassThru<S>;
    type LabelStream = PassThru<S>;

    fn stream(&mut self) -> &mut Self::Stream {
        &mut self.stream
    }

    fn label_stream(&mut self) -> &mut Self::LabelStream {
        &mut self.stream
    }

    fn flattened_value_begin(&mut self, _: Option<&Tag>, label: &Label, _: &Index) -> sval::Result {
        // If the source is a record then we won't see a distinct key
        if !self.stream.seen_key {
            self.stream.map_key_begin()?;
            if let Some(label) = label.as_static_str() {
                self.stream.value(label)?;
            } else {
                self.stream.value_computed(label.as_str())?;
            }
            self.stream.map_key_end()?;
        }

        self.stream.map_value_begin()
    }

    fn flattened_value_end(&mut self, _: Option<&Tag>, _: &Label, _: &Index) -> sval::Result {
        self.stream.map_value_end()
    }
}

struct PassThru<S> {
    stream: S,
    seen_key: bool,
}

impl<S> PassThru<S> {
    pub(crate) fn new(stream: S) -> Self {
        PassThru {
            stream,
            seen_key: false,
        }
    }
}

impl<'sval, S: Stream<'sval>> LabelStream<'sval> for PassThru<S> {
    fn take(&mut self) -> LabelBuf<'sval> {
        LabelBuf::Empty
    }
}

impl<'sval, S: Stream<'sval>> Stream<'sval> for PassThru<S> {
    fn value<V: Value + ?Sized>(&mut self, v: &'sval V) -> sval::Result {
        self.stream.value(v)
    }

    fn value_computed<V: Value + ?Sized>(&mut self, v: &V) -> sval::Result {
        self.stream.value_computed(v)
    }

    fn null(&mut self) -> sval::Result {
        self.stream.null()
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.stream.bool(value)
    }

    fn text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.stream.text_begin(num_bytes)
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.stream.text_fragment(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.stream.text_fragment_computed(fragment)
    }

    fn text_end(&mut self) -> sval::Result {
        self.stream.text_end()
    }

    fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.stream.binary_begin(num_bytes)
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.stream.binary_fragment(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.stream.binary_fragment_computed(fragment)
    }

    fn binary_end(&mut self) -> sval::Result {
        self.stream.binary_end()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.stream.u8(value)
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.stream.u16(value)
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.stream.u32(value)
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.stream.u64(value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.stream.u128(value)
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.stream.i8(value)
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.stream.i16(value)
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.stream.i32(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.stream.i64(value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.stream.i128(value)
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.stream.f32(value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.stream.f64(value)
    }

    fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.stream.map_begin(num_entries)
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.stream.map_key_begin()
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.seen_key = true;
        self.stream.map_key_end()
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.seen_key = false;
        self.stream.map_value_begin()
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.stream.map_value_end()
    }

    fn map_end(&mut self) -> sval::Result {
        self.stream.map_end()
    }

    fn seq_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.stream.seq_begin(num_entries)
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.stream.seq_value_begin()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.stream.seq_value_end()
    }

    fn seq_end(&mut self) -> sval::Result {
        self.stream.seq_end()
    }

    fn enum_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.enum_begin(tag, label, index)
    }

    fn enum_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.enum_end(tag, label, index)
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.tagged_begin(tag, label, index)
    }

    fn tagged_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.tagged_end(tag, label, index)
    }

    fn tag(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.tag(tag, label, index)
    }

    fn record_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.stream.record_begin(tag, label, index, num_entries)
    }

    fn record_value_begin(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        self.stream.record_value_begin(tag, label)
    }

    fn record_value_end(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        self.stream.record_value_end(tag, label)
    }

    fn record_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.record_end(tag, label, index)
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.stream.tuple_begin(tag, label, index, num_entries)
    }

    fn tuple_value_begin(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        self.stream.tuple_value_begin(tag, index)
    }

    fn tuple_value_end(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        self.stream.tuple_value_end(tag, index)
    }

    fn tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.tuple_end(tag, label, index)
    }

    fn record_tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.stream
            .record_tuple_begin(tag, label, index, num_entries)
    }

    fn record_tuple_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.stream.record_tuple_value_begin(tag, label, index)
    }

    fn record_tuple_value_end(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.stream.record_tuple_value_end(tag, label, index)
    }

    fn record_tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.stream.record_tuple_end(tag, label, index)
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::ToOwned;
    use sval_derive_macros::*;

    use super::*;

    struct Outer<I> {
        a: i32,
        // #[sval(flatten)]
        i: I,
        d: i32,
    }

    impl<I: sval::Value> sval::Value for Outer<I> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
            let mut offset = 0;

            stream.map_begin(None)?;

            stream.map_key_begin()?;
            "a".stream(&mut *stream)?;
            stream.map_key_end()?;

            stream.map_value_begin()?;
            stream.i32(self.a)?;
            stream.map_value_end()?;
            offset += 1;

            offset = flatten_to_map(&mut *stream, &self.i, offset)?;

            stream.map_key_begin()?;
            "d".stream(&mut *stream)?;
            stream.map_key_end()?;

            stream.map_value_begin()?;
            stream.i32(self.d)?;
            stream.map_value_end()?;
            offset += 1;

            let _ = offset;
            stream.map_end()
        }
    }

    #[test]
    fn flatten_record() {
        #[derive(Value)]
        #[sval(unindexed_fields)]
        struct Inner {
            b: i32,
            c: i32,
        }

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner { b: 2, c: 3 },
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("b"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("c"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );
    }

    #[test]
    fn flatten_tuple() {
        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: (2, 3),
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("1".to_owned()),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("2".to_owned()),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );
    }

    #[test]
    fn flatten_seq() {
        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: [2, 3],
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("1".to_owned()),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("2".to_owned()),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );
    }

    #[test]
    fn flatten_map() {
        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: sval::MapSlice::new(&[("b", 2), ("c", 3)]),
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("b"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("c"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );
    }

    #[test]
    fn flatten_map_complex() {
        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: sval::MapSlice::new(&[(["b1", "b2"], 2), (["c1", "c2"], 3)]),
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TaggedBegin(Some(sval::tags::CONSTANT_SIZE), None, None),
                    SeqBegin(Some(2)),
                    SeqValueBegin,
                    TextBegin(Some(2)),
                    TextFragment("b1"),
                    TextEnd,
                    SeqValueEnd,
                    SeqValueBegin,
                    TextBegin(Some(2)),
                    TextFragment("b2"),
                    TextEnd,
                    SeqValueEnd,
                    SeqEnd,
                    TaggedEnd(Some(sval::tags::CONSTANT_SIZE), None, None),
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TaggedBegin(Some(sval::tags::CONSTANT_SIZE), None, None),
                    SeqBegin(Some(2)),
                    SeqValueBegin,
                    TextBegin(Some(2)),
                    TextFragment("c1"),
                    TextEnd,
                    SeqValueEnd,
                    SeqValueBegin,
                    TextBegin(Some(2)),
                    TextFragment("c2"),
                    TextEnd,
                    SeqValueEnd,
                    SeqEnd,
                    TaggedEnd(Some(sval::tags::CONSTANT_SIZE), None, None),
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );
    }

    #[test]
    fn flatten_record_tuple() {
        #[derive(Value)]
        struct Inner {
            b: i32,
            c: i32,
        }

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner { b: 2, c: 3 },
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("b"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("c"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );
    }

    #[test]
    fn flatten_enum() {
        #[derive(Value)]
        enum Inner {
            #[sval(label = "b")]
            A(i32),
            B {
                b: i32,
                c: i32,
            },
            C(i32, i32),
        }

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner::A(2),
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("b"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner::B { b: 2, c: 3 },
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("b"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("c"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner::C(2, 3),
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    MapBegin(None),
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(1),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("1".to_owned()),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(2),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("2".to_owned()),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(3),
                    MapValueEnd,
                    MapKeyBegin,
                    TextBegin(Some(1)),
                    TextFragment("d"),
                    TextEnd,
                    MapKeyEnd,
                    MapValueBegin,
                    I32(4),
                    MapValueEnd,
                    MapEnd,
                ]
            },
        );
    }
}
