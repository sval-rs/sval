use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_seq<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
) -> sval::Result {
    let mut stream = SeqFlatten { stream, depth: 0 };

    value.stream(&mut stream)?;

    Ok(())
}

struct SeqFlatten<S> {
    stream: S,
    depth: usize,
}

impl<'sval, S: Stream<'sval>> SeqFlatten<S> {
    fn value(&mut self, passthru: impl FnOnce(&mut S) -> sval::Result) -> sval::Result {
        if self.depth == 0 {
            sval::error()
        } else {
            passthru(&mut self.stream)
        }
    }

    fn flattenable_value(
        &mut self,
        flatten: impl FnOnce(&mut S) -> sval::Result,
        passthru: impl FnOnce(&mut S) -> sval::Result,
    ) -> sval::Result {
        if self.depth == 1 {
            flatten(&mut self.stream)
        } else {
            passthru(&mut self.stream)
        }
    }

    fn flattenable_begin(&mut self, passthru: impl FnOnce(&mut S) -> sval::Result) -> sval::Result {
        self.depth += 1;

        if self.depth == 1 {
            Ok(())
        } else {
            passthru(&mut self.stream)
        }
    }

    fn flattenable_end(&mut self, passthru: impl FnOnce(&mut S) -> sval::Result) -> sval::Result {
        self.depth -= 1;

        if self.depth == 0 {
            Ok(())
        } else {
            passthru(&mut self.stream)
        }
    }
}

impl<'sval, S: Stream<'sval>> Stream<'sval> for SeqFlatten<S> {
    fn null(&mut self) -> sval::Result {
        self.value(|stream| stream.null())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.value(|stream| stream.bool(value))
    }

    fn text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.value(|stream| stream.text_begin(num_bytes))
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.stream.text_fragment_computed(fragment)
    }

    fn text_end(&mut self) -> sval::Result {
        self.value(|stream| stream.text_end())
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.value(|stream| stream.i64(value))
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.value(|stream| stream.f64(value))
    }

    fn seq_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.flattenable_begin(|stream| stream.seq_begin(num_entries))
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.stream.seq_value_begin()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.stream.seq_value_end()
    }

    fn seq_end(&mut self) -> sval::Result {
        self.flattenable_end(|stream| stream.seq_end())
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.stream.text_fragment(fragment)
    }

    fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.value(|stream| stream.binary_begin(num_bytes))
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.stream.binary_fragment(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.stream.binary_fragment_computed(fragment)
    }

    fn binary_end(&mut self) -> sval::Result {
        self.value(|stream| stream.binary_end())
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.value(|stream| stream.u8(value))
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.value(|stream| stream.u16(value))
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.value(|stream: &mut S| stream.u32(value))
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.value(|stream| stream.u64(value))
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.value(|stream: &mut S| stream.u128(value))
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.value(|stream| stream.i8(value))
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.value(|stream| stream.i16(value))
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.value(|stream| stream.i32(value))
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.value(|stream| stream.i128(value))
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.value(|stream| stream.f32(value))
    }

    fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.flattenable_begin(|stream| stream.map_begin(num_entries))
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.flattenable_value(|_| Ok(()), |stream| stream.map_key_begin())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.flattenable_value(|_| Ok(()), |stream| stream.map_key_end())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_begin(),
            |stream| stream.map_value_begin(),
        )
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_end(),
            |stream| stream.map_value_end(),
        )
    }

    fn map_end(&mut self) -> sval::Result {
        self.flattenable_end(|stream| stream.map_end())
    }

    fn enum_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_begin(|stream| stream.enum_begin(tag, label, index))
    }

    fn enum_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_end(|stream| stream.enum_end(tag, label, index))
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        if self.depth == 0 {
            Ok(())
        } else {
            self.flattenable_value(
                |stream| {
                    stream.seq_value_begin()?;
                    stream.tagged_begin(tag, label, index)
                },
                |stream| stream.tagged_begin(tag, label, index),
            )
        }
    }

    fn tagged_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        if self.depth == 0 {
            Ok(())
        } else {
            self.flattenable_value(
                |stream| {
                    stream.tagged_end(tag, label, index)?;
                    stream.seq_value_end()
                },
                |stream| stream.tagged_end(tag, label, index),
            )
        }
    }

    fn tag(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.value(|stream| stream.tag(tag, label, index))
    }

    fn record_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.flattenable_begin(|stream| stream.record_begin(tag, label, index, num_entries))
    }

    fn record_value_begin(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_begin(),
            |stream| stream.record_value_begin(tag, label),
        )
    }

    fn record_value_end(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_end(),
            |stream| stream.record_value_end(tag, label),
        )
    }

    fn record_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_end(|stream| stream.record_end(tag, label, index))
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.flattenable_begin(|stream| stream.tuple_begin(tag, label, index, num_entries))
    }

    fn tuple_value_begin(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_begin(),
            |stream| stream.tuple_value_begin(tag, index),
        )
    }

    fn tuple_value_end(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_end(),
            |stream| stream.tuple_value_end(tag, index),
        )
    }

    fn tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_end(|stream| stream.tuple_end(tag, label, index))
    }

    fn record_tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.flattenable_begin(|stream| stream.record_tuple_begin(tag, label, index, num_entries))
    }

    fn record_tuple_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_begin(),
            |stream| stream.record_tuple_value_begin(tag, label, index),
        )
    }

    fn record_tuple_value_end(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.flattenable_value(
            |stream| stream.seq_value_end(),
            |stream| stream.record_tuple_value_end(tag, label, index),
        )
    }

    fn record_tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_end(|stream| stream.record_tuple_end(tag, label, index))
    }
}
