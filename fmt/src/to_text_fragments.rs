use core::fmt::{self, Write as _};

/**
Stream a displayable value using [`sval::Stream::text_fragment_computed`], without calling [`sval::Stream::text_begin`] or [`sval::Stream::text_end`].

This function can be used to support implementations of [`TokenWrite`].
*/
pub fn stream_display_to_text_fragments<'sval>(
    stream: &mut (impl sval::Stream<'sval> + ?Sized),
    value: impl fmt::Display,
) -> fmt::Result {
    write!(TextFragmentWriter::new(stream), "{}", value)
}

/**
Stream a displayable value using [`sval::Stream::tagged_text_fragment_computed`], without calling [`sval::Stream::text_begin`] or [`sval::Stream::text_end`].

This function can be used to support implementations of [`TokenWrite`].
*/
pub fn stream_display_to_tagged_text_fragments<'sval>(
    stream: &mut (impl sval::Stream<'sval> + ?Sized),
    tag: &sval::Tag,
    value: impl fmt::Display,
) -> fmt::Result {
    write!(TaggedTextFragmentWriter::new(stream, tag), "{}", value)
}

struct TextFragmentWriter<S> {
    stream: S,
}

impl<S> TextFragmentWriter<S> {
    pub fn new(stream: S) -> Self {
        TextFragmentWriter { stream }
    }
}

impl<'sval, S: sval::Stream<'sval>> fmt::Write for TextFragmentWriter<S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.stream
            .text_fragment_computed(s)
            .map_err(|_| fmt::Error)
    }
}

struct TaggedTextFragmentWriter<'a, S> {
    tag: &'a sval::Tag,
    stream: S,
}

impl<'a, S> TaggedTextFragmentWriter<'a, S> {
    pub fn new(stream: S, tag: &'a sval::Tag) -> Self {
        TaggedTextFragmentWriter { tag, stream }
    }
}

impl<'a, 'sval, S: sval::Stream<'sval>> fmt::Write for TaggedTextFragmentWriter<'a, S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.stream
            .tagged_text_fragment_computed(self.tag, s)
            .map_err(|_| fmt::Error)
    }
}
