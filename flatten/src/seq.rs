use crate::{
    flattener::{Flatten, Flattener},
    label::Empty,
};
use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_seq<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let label_stream = Empty;

    let mut stream = Flattener::begin(
        SeqFlatten {
            stream,
            label_stream,
        },
        offset,
    );

    value.stream(&mut stream)?;

    Ok(stream.end())
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
