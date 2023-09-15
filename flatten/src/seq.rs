use crate::{
    flattener::{Flatten, Flattener},
    label::Empty,
};
use sval::{Index, Label, Stream, Tag};

/**
Flatten the fields of a value onto a sequence.
 */
pub fn flatten_to_seq<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
) -> sval::Result {
    let label_stream = Empty;

    let mut stream = Flattener::begin(
        SeqFlatten {
            stream,
            label_stream,
        },
        0,
    );

    value.stream(&mut stream)?;

    Ok(())
}

struct SeqFlatten<S> {
    stream: S,
    label_stream: Empty,
}

impl<'sval, S: Stream<'sval>> Flatten<'sval> for SeqFlatten<S> {
    type Stream = S;
    type LabelStream = Empty;

    fn stream(&mut self) -> &mut Self::Stream {
        &mut self.stream
    }

    fn label_stream(&mut self) -> &mut Self::LabelStream {
        &mut self.label_stream
    }

    fn flattened_value_begin(&mut self, _: Option<&Tag>, _: &Label, _: &Index) -> sval::Result {
        self.stream.seq_value_begin()
    }

    fn flattened_value_end(&mut self, _: Option<&Tag>, _: &Label, _: &Index) -> sval::Result {
        self.stream.seq_value_end()
    }
}

#[cfg(test)]
mod tests {
    use sval_derive_macros::*;

    use super::*;

    struct Outer<I>(i32, I, i32);

    impl<I: sval::Value> sval::Value for Outer<I> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
            stream.seq_begin(None)?;

            stream.seq_value_begin()?;
            stream.i32(self.0)?;
            stream.seq_value_end()?;

            flatten_to_seq(&mut *stream, &self.1)?;

            stream.seq_value_begin()?;
            stream.i32(self.2)?;
            stream.seq_value_end()?;

            stream.seq_end()
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
                SeqBegin(None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(3),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
            ]
        });
    }

    #[test]
    fn flatten_tuple() {
        sval_test::assert_tokens(&Outer(1, (2, 3), 4), {
            use sval_test::Token::*;

            &[
                SeqBegin(None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(3),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
            ]
        });
    }

    #[test]
    fn flatten_seq() {
        sval_test::assert_tokens(&Outer(1, [2, 3], 4), {
            use sval_test::Token::*;

            &[
                SeqBegin(None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(3),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
            ]
        });
    }

    #[test]
    fn flatten_map() {
        sval_test::assert_tokens(
            // Sequences ignore keys so can flatten even if they're complex
            &Outer(
                1,
                sval::MapSlice::new(&[(["b1", "b2"], 2), (["c1", "c2"], 3)]),
                4,
            ),
            {
                use sval_test::Token::*;

                &[
                    SeqBegin(None),
                    SeqValueBegin,
                    I32(1),
                    SeqValueEnd,
                    SeqValueBegin,
                    I32(2),
                    SeqValueEnd,
                    SeqValueBegin,
                    I32(3),
                    SeqValueEnd,
                    SeqValueBegin,
                    I32(4),
                    SeqValueEnd,
                    SeqEnd,
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
                SeqBegin(None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(3),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
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
                SeqBegin(None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
            ]
        });

        sval_test::assert_tokens(&Outer(1, Inner::B { b: 2, c: 3 }, 4), {
            use sval_test::Token::*;

            &[
                SeqBegin(None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(3),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
            ]
        });

        sval_test::assert_tokens(&Outer(1, Inner::C(2, 3), 4), {
            use sval_test::Token::*;

            &[
                SeqBegin(None),
                SeqValueBegin,
                I32(1),
                SeqValueEnd,
                SeqValueBegin,
                I32(2),
                SeqValueEnd,
                SeqValueBegin,
                I32(3),
                SeqValueEnd,
                SeqValueBegin,
                I32(4),
                SeqValueEnd,
                SeqEnd,
            ]
        });
    }
}
