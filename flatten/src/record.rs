use crate::{
    flattener::{Flatten, Flattener},
    label::LabelBuf,
};
use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_record<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let label_stream = LabelBuf::default();

    let mut stream = Flattener::begin(
        RecordFlatten {
            stream,
            label_stream,
        },
        offset,
    );

    value.stream(&mut stream)?;

    Ok(stream.end())
}

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
