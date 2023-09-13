use sval::{Index, Label, Stream, Tag};

use crate::{
    index::IndexAllocator,
    label::{LabelBuf, LabelStream},
};

pub(crate) struct Flattener<'sval, S> {
    stream: S,
    state: FlattenerState<'sval>,
}

struct FlattenerState<'sval> {
    index_alloc: IndexAllocator,
    key_buf: LabelBuf<'sval>,
    depth: usize,
    in_flattening_enum: bool,
    in_flattening_map_key: bool,
}

pub(crate) trait Flatten<'sval> {
    type Stream: Stream<'sval>;
    type LabelStream: LabelStream<'sval>;

    fn stream(&mut self) -> &mut Self::Stream;
    fn label_stream(&mut self) -> &mut Self::LabelStream;

    fn flattened_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result;

    fn flattened_value_end(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result;
}

impl<'sval, S: Flatten<'sval>> Flattener<'sval, S> {
    pub(crate) fn begin(stream: S, start_from: usize) -> Self {
        Flattener {
            stream,
            state: FlattenerState {
                index_alloc: IndexAllocator::start_from(start_from),
                key_buf: LabelBuf::Empty,
                depth: 0,
                in_flattening_enum: false,
                in_flattening_map_key: false,
            },
        }
    }

    pub(crate) fn end(self) -> usize {
        self.state.index_alloc.current_offset()
    }

    fn value(
        &mut self,
        buffer: impl FnOnce(&mut S::LabelStream) -> sval::Result,
        passthru: impl FnOnce(&mut S::Stream) -> sval::Result,
    ) -> sval::Result {
        self.value_at_root(|_, _| sval::error(), buffer, passthru)
    }

    fn value_at_root(
        &mut self,
        at_root: impl FnOnce(&mut S, &mut FlattenerState<'sval>) -> sval::Result,
        buffer: impl FnOnce(&mut S::LabelStream) -> sval::Result,
        passthru: impl FnOnce(&mut S::Stream) -> sval::Result,
    ) -> sval::Result {
        if self.state.depth == 0 {
            at_root(&mut self.stream, &mut self.state)
        } else if self.state.in_flattening_map_key {
            buffer(self.stream.label_stream())
        } else {
            passthru(self.stream.stream())
        }
    }

    fn flattenable_begin(
        &mut self,
        flatten: impl FnOnce(&mut S, &mut FlattenerState<'sval>) -> sval::Result,
        passthru: impl FnOnce(&mut S::Stream) -> sval::Result,
    ) -> sval::Result {
        if self.state.in_flattening_map_key {
            return sval::error();
        }

        self.state.depth += 1;

        if self.state.depth == 1 {
            flatten(&mut self.stream, &mut self.state)
        } else {
            passthru(self.stream.stream())
        }
    }

    fn flattenable_value(
        &mut self,
        flatten: impl FnOnce(&mut S, &mut FlattenerState<'sval>) -> sval::Result,
        passthru: impl FnOnce(&mut S::Stream) -> sval::Result,
    ) -> sval::Result {
        if self.state.depth == 1 {
            flatten(&mut self.stream, &mut self.state)
        } else {
            passthru(self.stream.stream())
        }
    }

    fn flattenable_end(
        &mut self,
        flatten: impl FnOnce(&mut S, &mut FlattenerState<'sval>) -> sval::Result,
        passthru: impl FnOnce(&mut S::Stream) -> sval::Result,
    ) -> sval::Result {
        self.state.depth -= 1;

        if self.state.depth == 0 {
            flatten(&mut self.stream, &mut self.state)
        } else {
            passthru(&mut self.stream.stream())
        }
    }
}

impl<'sval, S: Flatten<'sval>> Stream<'sval> for Flattener<'sval, S> {
    fn null(&mut self) -> sval::Result {
        self.value(|buf| buf.null(), |stream| stream.null())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.value(|buf| buf.bool(value), |stream| stream.bool(value))
    }

    fn text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.value(|_| Ok(()), |stream| stream.text_begin(num_bytes))
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.value(
            |buf| buf.text_fragment(fragment),
            |stream| stream.text_fragment(fragment),
        )
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.value(
            |buf| buf.text_fragment_computed(fragment),
            |stream| stream.text_fragment_computed(fragment),
        )
    }

    fn text_end(&mut self) -> sval::Result {
        self.value(|_| Ok(()), |stream| stream.text_end())
    }

    fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.value(|_| sval::error(), |stream| stream.binary_begin(num_bytes))
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.value(|_| sval::error(), |stream| stream.binary_fragment(fragment))
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.value(
            |_| sval::error(),
            |stream| stream.binary_fragment_computed(fragment),
        )
    }

    fn binary_end(&mut self) -> sval::Result {
        self.value(|_| sval::error(), |stream| stream.binary_end())
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.value(|buf| buf.u8(value), |stream| stream.u8(value))
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.value(|buf| buf.u16(value), |stream| stream.u16(value))
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.value(|buf| buf.u32(value), |stream| stream.u32(value))
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.value(|buf| buf.u64(value), |stream| stream.u64(value))
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.value(|buf| buf.u128(value), |stream| stream.u128(value))
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.value(|buf| buf.i8(value), |stream| stream.i8(value))
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.value(|buf| buf.i16(value), |stream| stream.i16(value))
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.value(|buf| buf.i32(value), |stream| stream.i32(value))
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.value(|buf| buf.i64(value), |stream| stream.i64(value))
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.value(|buf| buf.i128(value), |stream| stream.i128(value))
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.value(|buf| buf.f32(value), |stream| stream.f32(value))
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.value(|buf| buf.f64(value), |stream| stream.f64(value))
    }

    fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.flattenable_begin(|_, _| Ok(()), |stream| stream.map_begin(num_entries))
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                stream.label_stream().map_key_begin()?;
                state.in_flattening_map_key = true;
                Ok(())
            },
            |stream| stream.map_key_begin(),
        )
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                stream.label_stream().map_key_end()?;
                state.in_flattening_map_key = false;
                Ok(())
            },
            |stream| stream.map_key_begin(),
        )
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                state.key_buf = stream.label_stream().take();

                state.key_buf.with_label(|label| {
                    let index = &state.index_alloc.next_begin(None);

                    stream.flattened_value_begin(None, label, index)
                })
            },
            |stream| stream.map_value_begin(),
        )
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                state.key_buf.with_label(|label| {
                    let index = &state.index_alloc.next_end(None);

                    stream.flattened_value_end(None, label, index)
                })
            },
            |stream| stream.map_value_end(),
        )
    }

    fn map_end(&mut self) -> sval::Result {
        self.flattenable_end(|_, _| Ok(()), |stream| stream.map_end())
    }

    fn seq_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.flattenable_begin(|_, _| Ok(()), |stream| stream.seq_begin(num_entries))
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                let index = &state.index_alloc.next_begin(None);

                with_index_to_label(index, None, |label| {
                    stream.flattened_value_begin(None, label, index)
                })
            },
            |stream| stream.seq_value_begin(),
        )
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                let index = &state.index_alloc.next_end(None);

                with_index_to_label(index, None, |label| {
                    stream.flattened_value_end(None, label, index)
                })
            },
            |stream| stream.seq_value_end(),
        )
    }

    fn seq_end(&mut self) -> sval::Result {
        self.flattenable_end(|_, _| Ok(()), |stream| stream.seq_end())
    }

    fn enum_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.value_at_root(
            |_, state| {
                state.in_flattening_enum = true;
                Ok(())
            },
            |_| sval::error(),
            |stream| stream.enum_begin(tag, label, index),
        )
    }

    fn enum_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.value_at_root(
            |_, _| Ok(()),
            |_| sval::error(),
            |stream| stream.enum_begin(tag, label, index),
        )
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        if self.state.in_flattening_enum {
            self.flattenable_begin(
                |stream, state| {
                    let index = &state.index_alloc.next_begin(index);

                    with_index_to_label(index, label, |label| {
                        stream.flattened_value_begin(None, label, index)
                    })
                },
                |stream| stream.tagged_begin(tag, label, index),
            )
        }
        // Unwrap tagged values; we'll either find a flattenable value inside or fail
        else if self.state.depth == 0 {
            Ok(())
        } else {
            self.value(|_| Ok(()), |stream| stream.tagged_begin(tag, label, index))
        }
    }

    fn tagged_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        if self.state.in_flattening_enum {
            self.flattenable_end(
                |stream, state| {
                    let index = &state.index_alloc.next_end(index);

                    with_index_to_label(index, label, |label| {
                        stream.flattened_value_end(None, label, index)
                    })
                },
                |stream| stream.tagged_end(tag, label, index),
            )
        } else if self.state.depth == 0 {
            Ok(())
        } else {
            self.value(|_| Ok(()), |stream| stream.tagged_end(tag, label, index))
        }
    }

    fn tag(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.value(
            |buf| {
                if let Some(label) = label {
                    buf.label(label)
                } else if let Some(index) = index {
                    buf.index(index)
                } else {
                    buf.null()
                }
            },
            |stream| stream.tag(tag, label, index),
        )
    }

    fn record_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.flattenable_begin(
            |_, _| Ok(()),
            |stream| stream.record_begin(tag, label, index, num_entries),
        )
    }

    fn record_value_begin(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                stream.flattened_value_begin(tag, label, &state.index_alloc.next_begin(None))
            },
            |stream| stream.record_value_begin(tag, label),
        )
    }

    fn record_value_end(&mut self, tag: Option<&Tag>, label: &Label) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                stream.flattened_value_end(tag, label, &state.index_alloc.next_end(None))
            },
            |stream| stream.record_value_end(tag, label),
        )
    }

    fn record_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_end(|_, _| Ok(()), |stream| stream.record_end(tag, label, index))
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.flattenable_begin(
            |_, _| Ok(()),
            |stream| stream.tuple_begin(tag, label, index, num_entries),
        )
    }

    fn tuple_value_begin(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                let index = &state.index_alloc.next_begin(Some(index));

                with_index_to_label(index, None, |label| {
                    stream.flattened_value_begin(tag, label, index)
                })
            },
            |stream| stream.tuple_value_begin(tag, index),
        )
    }

    fn tuple_value_end(&mut self, tag: Option<&Tag>, index: &Index) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                let index = &state.index_alloc.next_end(Some(index));

                with_index_to_label(index, None, |label| {
                    stream.flattened_value_end(tag, label, index)
                })
            },
            |stream| stream.tuple_value_end(tag, index),
        )
    }

    fn tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_end(|_, _| Ok(()), |stream| stream.tuple_end(tag, label, index))
    }

    fn record_tuple_begin(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.flattenable_begin(
            |_, _| Ok(()),
            |stream| stream.record_tuple_begin(tag, label, index, num_entries),
        )
    }

    fn record_tuple_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        index: &Index,
    ) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                stream.flattened_value_begin(tag, label, &state.index_alloc.next_begin(Some(index)))
            },
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
            |stream, state| {
                stream.flattened_value_end(tag, label, &state.index_alloc.next_end(Some(index)))
            },
            |stream| stream.record_tuple_value_end(tag, label, index),
        )
    }

    fn record_tuple_end(
        &mut self,
        tag: Option<&Tag>,
        label: Option<&Label>,
        index: Option<&Index>,
    ) -> sval::Result {
        self.flattenable_end(
            |_, _| Ok(()),
            |stream| stream.record_tuple_end(tag, label, index),
        )
    }
}

fn with_index_to_label(
    index: &Index,
    label: Option<&Label>,
    f: impl FnOnce(&Label) -> sval::Result,
) -> sval::Result {
    if let Some(label) = label {
        return f(label);
    }

    LabelBuf::from_index(index)?.with_label(f)
}
