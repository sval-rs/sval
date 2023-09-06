use crate::flattener::{Flatten, Flattener};
use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_record_tuple<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let mut stream = Flattener::begin(RecordTupleFlatten(stream), offset);

    value.stream(&mut stream)?;

    Ok(stream.end())
}

struct RecordTupleFlatten<S>(S);

impl<'sval, S: Stream<'sval>> Flatten<'sval> for RecordTupleFlatten<S> {
    type Stream = S;

    fn as_stream(&mut self) -> &mut Self::Stream {
        &mut self.0
    }

    fn flattened_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.0.record_tuple_value_begin(tag, label, index)
    }

    fn flattened_value_end(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.0.record_tuple_value_end(tag, label, index)
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
}
