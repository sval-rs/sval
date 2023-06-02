use crate::writer::{StreamWriter, Writer};

/**
Stream a value as a sequence of text fragments containing the formatted value.

This method will only call `text_fragment` methods on the given stream. It won't call
`text_begin` or `text_end`. It's suitable for use within [`sval::Value`] implementations
that stream as text.
*/
pub fn stream_to_text_fragments<'sval>(
    stream: &mut (impl sval::Stream<'sval> + ?Sized),
    value: impl sval::Value,
) -> sval::Result {
    value.stream(&mut Writer::new(StreamWriter(stream)))
}
