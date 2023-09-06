use crate::flattener::{Flatten, Flattener};
use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_record<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let mut stream = Flattener::begin(RecordFlatten(stream), offset);

    value.stream(&mut stream)?;

    Ok(stream.end())
}

struct RecordFlatten<S>(S);

impl<'sval, S: Stream<'sval>> Flatten<'sval> for RecordFlatten<S> {
    type Stream = S;

    fn as_stream(&mut self) -> &mut Self::Stream {
        &mut self.0
    }

    fn flattened_value_begin(
        &mut self,
        tag: Option<&Tag>,
        label: &Label,
        _: &Index,
    ) -> sval::Result {
        self.0.record_value_begin(tag, label)
    }

    fn flattened_value_end(&mut self, tag: Option<&Tag>, label: &Label, _: &Index) -> sval::Result {
        self.0.record_value_end(tag, label)
    }
}
