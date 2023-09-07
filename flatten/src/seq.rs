use crate::flattener::{Flatten, Flattener};
use sval::{Index, Label, Stream, Tag};

pub fn flatten_to_seq<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl sval::Value + ?Sized),
    offset: usize,
) -> sval::Result<usize> {
    let mut stream = Flattener::begin(TupleFlatten(stream), offset);

    value.stream(&mut stream)?;

    Ok(stream.end())
}

struct TupleFlatten<S>(S);

impl<'sval, S: Stream<'sval>> Flatten<'sval> for TupleFlatten<S> {
    type Stream = S;

    fn as_stream(&mut self) -> &mut Self::Stream {
        &mut self.0
    }

    fn flattened_value_begin(&mut self, _: Option<&Tag>, _: &Label, _: &Index) -> sval::Result {
        self.0.seq_value_begin()
    }

    fn flattened_value_end(&mut self, _: Option<&Tag>, _: &Label, _: &Index) -> sval::Result {
        self.0.seq_value_end()
    }
}
