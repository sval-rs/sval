use crate::{
    flattener::{Flatten, Flattener},
    label::Empty,
};
use sval::{Index, Label, Stream, Tag};

/**
Flatten the fields of a value onto a tuple.

The `offset` is the current length of the tuple. A new offset will be returned
with the length of the tuple after flattening the value.
 */
pub fn flatten_to_tuple<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: isize,
) -> sval::Result<isize> {
    let mut stream = FlattenToTuple::new(stream, offset);

    value.stream(&mut stream)?;

    Ok(stream.end().0)
}

/**
A stream that flattens the fields of a value onto a tuple.
*/
pub struct FlattenToTuple<'sval, S> {
    inner: Flattener<'sval, TupleFlatten<S>>,
}

impl<'sval, S: sval::Stream<'sval>> FlattenToTuple<'sval, S> {
    /**
    Wrap the given `stream`.

    The `offset` is the current length of the tuple being flattened onto.
    Call [`FlattenToTuple::end`] after streaming a value to get the new length of the tuple.
    */
    pub fn new(stream: S, offset: isize) -> Self {
        FlattenToTuple {
            inner: Flattener::begin(
                TupleFlatten {
                    stream,
                    label_stream: Empty,
                },
                offset,
            ),
        }
    }

    /**
    Finish flattening a value.

    This method returns the length of the map after flattening that can be used to reconstruct a `FlattenToTuple` for a future value.
    */
    pub fn end(self) -> (isize, S) {
        let (offset, TupleFlatten { stream, .. }) = self.inner.end();

        (offset, stream)
    }
}

impl_stream_forward!({ impl<'sval, S: Stream<'sval>> Stream<'sval> for FlattenToTuple<'sval, S> } => x => { x.inner });

struct TupleFlatten<S> {
    stream: S,
    label_stream: Empty,
}

impl<'sval, S: Stream<'sval>> Flatten<'sval> for TupleFlatten<S> {
    type Stream = S;
    type LabelStream = Empty;

    fn stream(&mut self) -> &mut Self::Stream {
        &mut self.stream
    }

    fn label_stream(&mut self) -> &mut Self::LabelStream {
        &mut self.label_stream
    }

    fn flattened_value_begin(
        &mut self,
        tag: Option<&Tag>,
        _: &Label,
        index: &Index,
    ) -> sval::Result {
        self.stream.tuple_value_begin(tag, index)
    }

    fn flattened_value_end(&mut self, tag: Option<&Tag>, _: &Label, index: &Index) -> sval::Result {
        self.stream.tuple_value_end(tag, index)
    }
}

#[cfg(test)]
mod tests {
    use sval_derive_macros::*;

    use super::*;

    struct Outer<I>(i32, I, i32);

    impl<I: sval::Value> sval::Value for Outer<I> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
            let mut offset = 0;

            stream.tuple_begin(None, Some(&Label::new("Outer")), None, None)?;

            stream.tuple_value_begin(
                None,
                &Index::from(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            stream.i32(self.0)?;
            stream.tuple_value_end(
                None,
                &Index::from(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            offset += 1;

            offset = flatten_to_tuple(&mut *stream, &self.1, offset)?;

            stream.tuple_value_begin(
                None,
                &Index::from(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            stream.i32(self.2)?;
            stream.tuple_value_end(
                None,
                &Index::from(offset).with_tag(&sval::tags::VALUE_OFFSET),
            )?;
            offset += 1;

            let _ = offset;
            stream.tuple_end(None, Some(&Label::new("Outer")), None)
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

        sval_test::assert_tokens(&Outer(1, Inner { b: 2, c: 3 }, 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(Label::new("Outer")), None, None),
                TupleValueBegin(None, Index::new(0)),
                I32(1),
                TupleValueEnd(None, Index::new(0)),
                TupleValueBegin(None, Index::new(1)),
                I32(2),
                TupleValueEnd(None, Index::new(1)),
                TupleValueBegin(None, Index::new(2)),
                I32(3),
                TupleValueEnd(None, Index::new(2)),
                TupleValueBegin(None, Index::new(3)),
                I32(4),
                TupleValueEnd(None, Index::new(3)),
                TupleEnd(None, Some(Label::new("Outer")), None),
            ]
        });
    }

    #[test]
    fn flatten_tuple() {
        sval_test::assert_tokens(&Outer(1, (2, 3), 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(Label::new("Outer")), None, None),
                TupleValueBegin(None, Index::new(0)),
                I32(1),
                TupleValueEnd(None, Index::new(0)),
                TupleValueBegin(None, Index::new(1)),
                I32(2),
                TupleValueEnd(None, Index::new(1)),
                TupleValueBegin(None, Index::new(2)),
                I32(3),
                TupleValueEnd(None, Index::new(2)),
                TupleValueBegin(None, Index::new(3)),
                I32(4),
                TupleValueEnd(None, Index::new(3)),
                TupleEnd(None, Some(Label::new("Outer")), None),
            ]
        });
    }

    #[test]
    fn flatten_seq() {
        sval_test::assert_tokens(&Outer(1, [2, 3], 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(Label::new("Outer")), None, None),
                TupleValueBegin(None, Index::new(0)),
                I32(1),
                TupleValueEnd(None, Index::new(0)),
                TupleValueBegin(None, Index::new(1)),
                I32(2),
                TupleValueEnd(None, Index::new(1)),
                TupleValueBegin(None, Index::new(2)),
                I32(3),
                TupleValueEnd(None, Index::new(2)),
                TupleValueBegin(None, Index::new(3)),
                I32(4),
                TupleValueEnd(None, Index::new(3)),
                TupleEnd(None, Some(Label::new("Outer")), None),
            ]
        });
    }

    #[test]
    fn flatten_map() {
        sval_test::assert_tokens(
            &Outer(
                1,
                sval::MapSlice::new(&[(["b1", "b2"], 2), (["c1", "c2"], 3)]),
                4,
            ),
            {
                use sval_test::Token::*;

                &[
                    TupleBegin(None, Some(Label::new("Outer")), None, None),
                    TupleValueBegin(None, Index::new(0)),
                    I32(1),
                    TupleValueEnd(None, Index::new(0)),
                    TupleValueBegin(None, Index::new(1)),
                    I32(2),
                    TupleValueEnd(None, Index::new(1)),
                    TupleValueBegin(None, Index::new(2)),
                    I32(3),
                    TupleValueEnd(None, Index::new(2)),
                    TupleValueBegin(None, Index::new(3)),
                    I32(4),
                    TupleValueEnd(None, Index::new(3)),
                    TupleEnd(None, Some(Label::new("Outer")), None),
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

        sval_test::assert_tokens(&Outer(1, Inner { b: 2, c: 3 }, 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(Label::new("Outer")), None, None),
                TupleValueBegin(None, Index::new(0)),
                I32(1),
                TupleValueEnd(None, Index::new(0)),
                TupleValueBegin(None, Index::new(1)),
                I32(2),
                TupleValueEnd(None, Index::new(1)),
                TupleValueBegin(None, Index::new(2)),
                I32(3),
                TupleValueEnd(None, Index::new(2)),
                TupleValueBegin(None, Index::new(3)),
                I32(4),
                TupleValueEnd(None, Index::new(3)),
                TupleEnd(None, Some(Label::new("Outer")), None),
            ]
        });
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

        sval_test::assert_tokens(&Outer(1, Inner::A(2), 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(Label::new("Outer")), None, None),
                TupleValueBegin(None, Index::new(0)),
                I32(1),
                TupleValueEnd(None, Index::new(0)),
                TupleValueBegin(None, Index::new(1)),
                I32(2),
                TupleValueEnd(None, Index::new(1)),
                TupleValueBegin(None, Index::new(2)),
                I32(4),
                TupleValueEnd(None, Index::new(2)),
                TupleEnd(None, Some(Label::new("Outer")), None),
            ]
        });

        sval_test::assert_tokens(&Outer(1, Inner::B { b: 2, c: 3 }, 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(Label::new("Outer")), None, None),
                TupleValueBegin(None, Index::new(0)),
                I32(1),
                TupleValueEnd(None, Index::new(0)),
                TupleValueBegin(None, Index::new(1)),
                I32(2),
                TupleValueEnd(None, Index::new(1)),
                TupleValueBegin(None, Index::new(2)),
                I32(3),
                TupleValueEnd(None, Index::new(2)),
                TupleValueBegin(None, Index::new(3)),
                I32(4),
                TupleValueEnd(None, Index::new(3)),
                TupleEnd(None, Some(Label::new("Outer")), None),
            ]
        });

        sval_test::assert_tokens(&Outer(1, Inner::C(2, 3), 4), {
            use sval_test::Token::*;

            &[
                TupleBegin(None, Some(Label::new("Outer")), None, None),
                TupleValueBegin(None, Index::new(0)),
                I32(1),
                TupleValueEnd(None, Index::new(0)),
                TupleValueBegin(None, Index::new(1)),
                I32(2),
                TupleValueEnd(None, Index::new(1)),
                TupleValueBegin(None, Index::new(2)),
                I32(3),
                TupleValueEnd(None, Index::new(2)),
                TupleValueBegin(None, Index::new(3)),
                I32(4),
                TupleValueEnd(None, Index::new(3)),
                TupleEnd(None, Some(Label::new("Outer")), None),
            ]
        });
    }
}
