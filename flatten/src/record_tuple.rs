use crate::{
    flattener::{Flatten, Flattener},
    label::LabelBuf,
};
use sval::{Index, Label, Stream, Tag};

/**
Flatten the fields of a value onto a record or tuple.

The `offset` is the current length of the record or tuple. A new offset will be returned
with the length of the record or tuple after flattening the value.
 */
pub fn flatten_to_record_tuple<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let label_stream = LabelBuf::default();

    let mut stream = Flattener::begin(
        RecordTupleFlatten {
            stream,
            label_stream,
        },
        offset,
    );

    value.stream(&mut stream)?;

    Ok(stream.end())
}

struct RecordTupleFlatten<'sval, S> {
    stream: S,
    label_stream: LabelBuf<'sval>,
}

impl<'sval, S: Stream<'sval>> Flatten<'sval> for RecordTupleFlatten<'sval, S> {
    type Stream = S;
    type LabelStream = LabelBuf<'sval>;

    fn stream(&mut self) -> &mut Self::Stream {
        &mut self.stream
    }

    fn label_stream(&mut self) -> &mut Self::LabelStream {
        &mut self.label_stream
    }

    fn flattened_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.stream.record_tuple_value_begin(tag, label, index)
    }

    fn flattened_value_end(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.stream.record_tuple_value_end(tag, label, index)
    }
}

#[cfg(test)]
mod tests {
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

            stream.record_tuple_begin(None, Some(&Label::new("Outer")), None, None)?;

            stream.record_tuple_value_begin(
                None,
                &Label::new("a"),
                &Index::new(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            stream.i32(self.a)?;
            stream.record_tuple_value_end(
                None,
                &Label::new("a"),
                &Index::new(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            offset += 1;

            offset = flatten_to_record_tuple(&mut *stream, &self.i, offset)?;

            stream.record_tuple_value_begin(
                None,
                &Label::new("d"),
                &Index::new(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            stream.i32(self.d)?;
            stream.record_tuple_value_end(
                None,
                &Label::new("d"),
                &Index::new(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            offset += 1;

            let _ = offset;
            stream.record_tuple_end(None, Some(&Label::new("Outer")), None)
        }
    }

    #[test]
    fn flatten_record() {
        #[derive(Value)]
        #[sval(unindexed_values)]
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("b"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("c"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("c"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("1"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("1"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("2"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("2"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("1"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("1"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("2"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("2"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("b"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("c"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("c"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("b"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("c"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("c"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("b"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(2)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(2)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("b"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("c"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("c"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("1"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("1"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("2"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("2"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );
    }

    #[test]
    fn flatten_nested() {
        #[derive(Value)]
        struct Inner {
            b: ReallyInner,
            c: ReallyInner,
        }

        #[derive(Value)]
        struct ReallyInner {
            b1: i32,
            b2: i32,
        }

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner {
                    b: ReallyInner { b1: 21, b2: 22 },
                    c: ReallyInner { b1: 31, b2: 32 },
                },
                d: 4,
            },
            {
                use sval_test::Token::*;

                /*
                RecordTupleBegin(None, Some(Label::new("ReallyInner")), None, None),
                RecordTupleValueBegin(None, Label::new("b1"), Index::new(0)),
                I32(21),
                RecordTupleValueEnd(None, Label::new("b1"), Index::new(0)),
                RecordTupleValueBegin(None, Label::new("b2"), Index::new(1)),
                I32(22),
                RecordTupleValueEnd(None, Label::new("b2"), Index::new(1)),
                RecordTupleEnd(None, Some(Label::new("ReallyInner")), None),
                */

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b"), Index::new(1)),
                    RecordTupleBegin(None, Some(Label::new("ReallyInner")), None, Some(2)),
                    RecordTupleValueBegin(None, Label::new("b1"), Index::new(0)),
                    I32(21),
                    RecordTupleValueEnd(None, Label::new("b1"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b2"), Index::new(1)),
                    I32(22),
                    RecordTupleValueEnd(None, Label::new("b2"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("ReallyInner")), None),
                    RecordTupleValueEnd(None, Label::new("b"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("c"), Index::new(2)),
                    RecordTupleBegin(None, Some(Label::new("ReallyInner")), None, Some(2)),
                    RecordTupleValueBegin(None, Label::new("b1"), Index::new(0)),
                    I32(31),
                    RecordTupleValueEnd(None, Label::new("b1"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b2"), Index::new(1)),
                    I32(32),
                    RecordTupleValueEnd(None, Label::new("b2"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("ReallyInner")), None),
                    RecordTupleValueEnd(None, Label::new("c"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
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
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("b1b2"), Index::new(1)),
                    I32(2),
                    RecordTupleValueEnd(None, Label::new("b1b2"), Index::new(1)),
                    RecordTupleValueBegin(None, Label::new("c1c2"), Index::new(2)),
                    I32(3),
                    RecordTupleValueEnd(None, Label::new("c1c2"), Index::new(2)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(3)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(3)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );
    }

    #[test]
    fn flatten_enum_tag() {
        #[derive(Value)]
        enum Inner {
            A,
        }

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner::A,
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(1)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );
    }

    #[test]
    fn flatten_primitive() {
        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: 1u128,
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(1)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: -1i128,
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(1)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: 3.14f64,
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(1)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: true,
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(1)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: "Text",
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(1)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: sval::BinarySlice::new(b"Binary"),
                d: 4,
            },
            {
                use sval_test::Token::*;

                &[
                    RecordTupleBegin(None, Some(Label::new("Outer")), None, None),
                    RecordTupleValueBegin(None, Label::new("a"), Index::new(0)),
                    I32(1),
                    RecordTupleValueEnd(None, Label::new("a"), Index::new(0)),
                    RecordTupleValueBegin(None, Label::new("d"), Index::new(1)),
                    I32(4),
                    RecordTupleValueEnd(None, Label::new("d"), Index::new(1)),
                    RecordTupleEnd(None, Some(Label::new("Outer")), None),
                ]
            },
        );
    }
}
