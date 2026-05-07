use crate::{
    flattener::{Flatten, Flattener},
    label::LabelBuf,
};
use sval::{Index, Label, Stream, Tag};

/**
Flatten the fields of a value onto a record.

The `offset` is the current length of the record. A new offset will be returned
with the length of the record after flattening the value.
 */
pub fn flatten_to_record<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: isize,
) -> sval::Result<isize> {
    let mut stream = FlattenToRecord::new(stream, offset);

    value.stream(&mut stream)?;

    Ok(stream.end().0)
}

/**
A stream that flattens the fields of a value onto a record.
*/
pub struct FlattenToRecord<'sval, S> {
    inner: Flattener<'sval, RecordFlatten<'sval, S>>,
}

impl<'sval, S: sval::Stream<'sval>> FlattenToRecord<'sval, S> {
    /**
    Wrap the given `stream`.

    The `offset` is the current length of the record being flattened onto.
    Call [`FlattenToRecord::end`] after streaming a value to get the new length of the record.
    */
    pub fn new(stream: S, offset: isize) -> Self {
        let label_stream = LabelBuf::default();

        FlattenToRecord {
            inner: Flattener::begin(
                RecordFlatten {
                    stream,
                    label_stream,
                },
                offset,
            ),
        }
    }

    /**
    Finish flattening a value.

    This method returns the length of the map after flattening that can be used to reconstruct a `FlattenToRecord` for a future value.
    */
    pub fn end(self) -> (isize, S) {
        let (offset, RecordFlatten { stream, .. }) = self.inner.end();

        (offset, stream)
    }
}

impl_stream_forward!({ impl<'sval, S: Stream<'sval>> Stream<'sval> for FlattenToRecord<'sval, S> } => x => { x.inner });

struct RecordFlatten<'sval, S> {
    stream: S,
    label_stream: LabelBuf<'sval>,
}

impl<'sval, S: Stream<'sval>> Flatten<'sval> for RecordFlatten<'sval, S> {
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
        _: &Index,
    ) -> sval::Result {
        self.stream.record_value_begin(tag, label)
    }

    fn flattened_value_end(&mut self, tag: Option<&Tag>, label: &Label, _: &Index) -> sval::Result {
        self.stream.record_value_end(tag, label)
    }
}
