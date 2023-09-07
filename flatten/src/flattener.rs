use core::fmt::Write as _;
use sval::{Index, Label, Stream, Tag};
use sval_buffer::TextBuf;

pub(crate) struct Flattener<'sval, S> {
    stream: S,
    state: FlattenerState<'sval>,
}

struct FlattenerState<'sval> {
    map_key_buf: LabelBuf<'sval>,
    index_alloc: IndexAllocator,
    depth: usize,
    in_flattening_enum: bool,
    in_flattening_map_key: bool,
}

pub(crate) trait Flatten<'sval> {
    type Stream: Stream<'sval>;

    fn as_stream(&mut self) -> &mut Self::Stream;

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
                map_key_buf: LabelBuf::new(),
                index_alloc: IndexAllocator::start_from(start_from),
                depth: 0,
                in_flattening_enum: false,
                in_flattening_map_key: false,
            },
        }
    }

    pub(crate) fn end(self) -> usize {
        self.state.index_alloc.current_offset
    }

    fn value(
        &mut self,
        buffer: impl FnOnce(&mut LabelBuf<'sval>) -> sval::Result,
        passthru: impl FnOnce(&mut S::Stream) -> sval::Result,
    ) -> sval::Result {
        self.value_at_root(|_, _| sval::error(), buffer, passthru)
    }

    fn value_at_root(
        &mut self,
        at_root: impl FnOnce(&mut S, &mut FlattenerState<'sval>) -> sval::Result,
        buffer: impl FnOnce(&mut LabelBuf<'sval>) -> sval::Result,
        passthru: impl FnOnce(&mut S::Stream) -> sval::Result,
    ) -> sval::Result {
        if self.state.depth == 0 {
            at_root(&mut self.stream, &mut self.state)
        } else if self.state.in_flattening_map_key {
            buffer(&mut self.state.map_key_buf)
        } else {
            passthru(&mut self.stream.as_stream())
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
            passthru(&mut self.stream.as_stream())
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
            passthru(&mut self.stream.as_stream())
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
            passthru(&mut self.stream.as_stream())
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
        self.value(|buf| buf.u128(value), |stream| stream.u8(value))
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.value(|buf| buf.u128(value), |stream| stream.u16(value))
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.value(|buf| buf.u128(value), |stream| stream.u32(value))
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.value(|buf| buf.u128(value), |stream| stream.u64(value))
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.value(|buf| buf.u128(value), |stream| stream.u128(value))
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.value(|buf| buf.i128(value), |stream| stream.i8(value))
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.value(|buf| buf.i128(value), |stream| stream.i16(value))
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.value(|buf| buf.i128(value), |stream| stream.i32(value))
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.value(|buf| buf.i128(value), |stream| stream.i64(value))
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.value(|buf| buf.i128(value), |stream| stream.i128(value))
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.value(|buf| buf.f64(value), |stream| stream.f32(value))
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.value(|buf| buf.f64(value), |stream| stream.f64(value))
    }

    fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.flattenable_begin(|_, _| Ok(()), |stream| stream.map_begin(num_entries))
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.flattenable_value(
            |_, state| {
                state.map_key_buf = LabelBuf::new();
                state.in_flattening_map_key = true;
                Ok(())
            },
            |stream| stream.map_key_begin(),
        )
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.flattenable_value(
            |_, state| {
                state.in_flattening_map_key = false;
                Ok(())
            },
            |stream| stream.map_key_begin(),
        )
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.flattenable_value(
            |stream, state| {
                state.map_key_buf.with_label(|label| {
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
                state.map_key_buf.with_label(|label| {
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

struct IndexAllocator {
    initial_offset: usize,
    current_offset: usize,
}

impl IndexAllocator {
    fn start_from(offset: usize) -> Self {
        IndexAllocator {
            initial_offset: offset,
            current_offset: offset,
        }
    }

    fn next_begin(&mut self, incoming: Option<&Index>) -> Index {
        match incoming {
            // If there's an incoming tag then merge it into the current set
            Some(incoming) => match (incoming.tag(), incoming.to_usize()) {
                // If the incoming tag is a value offset then increment it by our starting point
                (Some(&sval::tags::VALUE_OFFSET), Some(incoming)) => {
                    Index::new(incoming + self.initial_offset).with_tag(&sval::tags::VALUE_OFFSET)
                }
                // If the incoming tag is not a value offset then just use it directly
                _ => incoming.clone(),
            },
            // If there's no incoming tag then construct one
            None => Index::new(self.current_offset).with_tag(&sval::tags::VALUE_OFFSET),
        }
    }

    fn next_end(&mut self, incoming: Option<&Index>) -> Index {
        let index = self.next_begin(incoming);
        self.current_offset += 1;

        index
    }
}

enum LabelBuf<'sval> {
    Empty,
    Text(TextBuf<'sval>),
    I128(i128),
    U128(u128),
    F64(f64),
}

impl<'sval> LabelBuf<'sval> {
    fn new() -> Self {
        LabelBuf::Empty
    }

    fn from_index(index: &Index) -> sval::Result<Self> {
        let mut buf = LabelBuf::new();
        buf.index(index)?;

        Ok(buf)
    }

    fn i128(&mut self, v: impl TryInto<i128>) -> sval::Result {
        *self = LabelBuf::I128(v.try_into().map_err(|_| sval::Error::new())?);
        Ok(())
    }

    fn u128(&mut self, v: impl TryInto<u128>) -> sval::Result {
        *self = LabelBuf::U128(v.try_into().map_err(|_| sval::Error::new())?);
        Ok(())
    }

    fn f64(&mut self, v: impl TryInto<f64>) -> sval::Result {
        *self = LabelBuf::F64(v.try_into().map_err(|_| sval::Error::new())?);
        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        self.text_fragment("null")
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        self.text_fragment(if v { "true" } else { "false" })
    }

    fn label(&mut self, label: &Label) -> sval::Result {
        if let Some(label) = label.as_static_str() {
            self.text_fragment(label)
        } else {
            self.text_fragment_computed(label.as_str())
        }
    }

    fn index(&mut self, index: &Index) -> sval::Result {
        if let Some(index) = index.to_isize() {
            self.i128(index)
        } else if let Some(index) = index.to_usize() {
            self.u128(index)
        } else {
            let buf = self.text_buf()?;
            write!(buf, "{}", index).map_err(|_| sval::Error::new())
        }
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.text_buf()?
            .push_fragment(fragment)
            .map_err(|_| sval::Error::new())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.text_buf()?
            .push_fragment_computed(fragment)
            .map_err(|_| sval::Error::new())
    }

    fn text_buf(&mut self) -> sval::Result<&mut TextBuf<'sval>> {
        match self {
            LabelBuf::Text(buf) => Ok(buf),
            _ => {
                *self = LabelBuf::Text(TextBuf::new());
                if let LabelBuf::Text(buf) = self {
                    Ok(buf)
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn with_label(&self, f: impl FnOnce(&Label) -> sval::Result) -> sval::Result {
        match self {
            LabelBuf::Empty => f(&Label::new_computed("")),
            LabelBuf::Text(text) => f(&Label::new_computed(text.as_str())),
            LabelBuf::I128(v) => {
                let mut buf = itoa::Buffer::new();
                f(&Label::new_computed(buf.format(*v)))
            }
            LabelBuf::U128(v) => {
                let mut buf = itoa::Buffer::new();
                f(&Label::new_computed(buf.format(*v)))
            }
            LabelBuf::F64(v) => {
                let mut buf = ryu::Buffer::new();
                f(&Label::new_computed(buf.format(*v)))
            }
        }
    }
}
