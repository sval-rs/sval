use core::fmt::Write as _;

use crate::IndexAllocator;
use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_record_tuple<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let mut stream = Flatten {
        stream,
        index_alloc: IndexAllocator::start_from(offset),
        depth: 0,
    };

    value.stream(&mut stream)?;

    Ok(stream.index_alloc.current_offset)
}

struct Flatten<S> {
    stream: S,
    index_alloc: IndexAllocator,
    depth: usize,
}

impl<'sval, S: Stream<'sval>> Stream<'sval> for Flatten<S> {
    fn null(&mut self) -> sval::Result {
        todo!()
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        todo!()
    }

    fn text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        todo!()
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        todo!()
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        todo!()
    }

    fn text_end(&mut self) -> sval::Result {
        todo!()
    }

    fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        todo!()
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        todo!()
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        todo!()
    }

    fn binary_end(&mut self) -> sval::Result {
        todo!()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        todo!()
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        todo!()
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        todo!()
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        todo!()
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        todo!()
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        todo!()
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        todo!()
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        // TODO: if !self.in_field => err
        self.stream.i32(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        todo!()
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        todo!()
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        todo!()
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        todo!()
    }

    fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        todo!()
    }

    fn map_key_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn map_key_end(&mut self) -> sval::Result {
        todo!()
    }

    fn map_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn map_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn map_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        todo!()
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_end(&mut self) -> sval::Result {
        todo!()
    }

    fn enum_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        todo!()
    }

    fn enum_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tagged_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tag(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        todo!()
    }

    fn record_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.depth += 1;

        if self.depth == 1 {
            Ok(())
        } else {
            self.stream.record_begin(tag, label, index, num_entries)
        }
    }

    fn record_value_begin(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        if self.depth == 1 {
            self.stream
                .record_tuple_value_begin(tag, label, &self.index_alloc.next_begin(None))
        } else {
            self.stream.record_value_begin(tag, label)
        }
    }

    fn record_value_end(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        if self.depth == 1 {
            self.stream
                .record_tuple_value_end(tag, label, &self.index_alloc.next_end(None))
        } else {
            self.stream.record_value_end(tag, label)
        }
    }

    fn record_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.depth -= 1;

        if self.depth == 0 {
            Ok(())
        } else {
            self.stream.record_end(tag, label, index)
        }
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.depth += 1;

        if self.depth == 1 {
            Ok(())
        } else {
            self.stream.tuple_begin(tag, label, index, num_entries)
        }
    }

    fn tuple_value_begin(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        if self.depth == 1 {
            let index = self.index_alloc.next_begin(Some(index));

            with_index_to_label(&index, |label| {
                self.stream.record_tuple_value_begin(tag, &label, &index)
            })
        } else {
            self.stream.tuple_value_begin(tag, index)
        }
    }

    fn tuple_value_end(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        if self.depth == 1 {
            let index = self.index_alloc.next_end(Some(index));

            with_index_to_label(&index, |label| {
                self.stream.record_tuple_value_end(tag, &label, &index)
            })
        } else {
            self.stream.tuple_value_end(tag, index)
        }
    }

    fn tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.depth -= 1;

        if self.depth == 0 {
            Ok(())
        } else {
            self.stream.tuple_end(tag, label, index)
        }
    }

    fn record_tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.depth += 1;

        if self.depth == 1 {
            Ok(())
        } else {
            self.stream
                .record_tuple_begin(tag, label, index, num_entries)
        }
    }

    fn record_tuple_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        if self.depth == 1 {
            self.stream.record_tuple_value_begin(
                tag,
                label,
                &self.index_alloc.next_begin(Some(index)),
            )
        } else {
            self.stream.record_tuple_value_begin(tag, label, index)
        }
    }

    fn record_tuple_value_end(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        if self.depth == 1 {
            self.stream
                .record_tuple_value_end(tag, label, &self.index_alloc.next_end(Some(index)))
        } else {
            self.stream.record_tuple_value_end(tag, label, index)
        }
    }

    fn record_tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.depth -= 1;

        if self.depth == 0 {
            Ok(())
        } else {
            self.stream.record_tuple_end(tag, label, index)
        }
    }
}

fn with_index_to_label(
    index: &sval::Index,
    f: impl FnOnce(sval::Label) -> sval::Result,
) -> sval::Result {
    let mut inline = itoa::Buffer::new();
    let mut fallback = sval_buffer::TextBuf::new();
    let label = if let Some(index) = index.to_isize() {
        inline.format(index)
    } else {
        write!(&mut fallback, "{}", index).map_err(|_| sval::Error::new())?;
        fallback.as_str()
    };

    f(sval::Label::new_computed(label))
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
    fn flatten_map() {
        todo!()
    }

    #[test]
    fn flatten_seq() {
        todo!()
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
            &[],
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
            &[],
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
            &[],
        );
    }

    #[test]
    fn flatten_enum() {
        #[derive(Value)]
        enum Inner {
            A(i32),
            B { b: i32, c: i32 },
            C(i32, i32),
        }

        sval_test::assert_tokens(
            &Outer {
                a: 1,
                i: Inner::A(2),
                d: 4,
            },
            &[],
        );
    }
}
