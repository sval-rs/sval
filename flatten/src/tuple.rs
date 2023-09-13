use crate::{
    flattener::{Flatten, Flattener},
    label::LabelBuf,
};
use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_tuple<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let key_stream = LabelBuf::default();

    let mut stream = Flattener::begin(TupleFlatten { stream, key_stream }, offset);

    value.stream(&mut stream)?;

    Ok(stream.end())
}

struct TupleFlatten<'sval, S> {
    stream: S,
    key_stream: LabelBuf<'sval>,
}

impl<'sval, S: Stream<'sval>> Flatten<'sval> for TupleFlatten<'sval, S> {
    type Stream = S;
    type KeyStream = LabelBuf<'sval>;

    fn stream(&mut self) -> &mut Self::Stream {
        &mut self.stream
    }

    fn key_stream(&mut self) -> &mut Self::KeyStream {
        &mut self.key_stream
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
