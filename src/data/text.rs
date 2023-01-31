use crate::{
    std::fmt::{self, Write},
    Result, Stream, Value,
};

/**
Stream the output of a generic debug format as a text string.
*/
pub fn stream_debug<'sval, T: fmt::Debug>(
    mut stream: &mut (impl Stream<'sval> + ?Sized),
    text: T,
) -> fmt::Result {
    struct Writer<S>(S);

    impl<'a, S: Stream<'a>> Write for Writer<S> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.text_fragment_computed(s).map_err(|_| fmt::Error)?;

            Ok(())
        }
    }

    stream.text_begin(None).map_err(|_| fmt::Error)?;

    write!(Writer(&mut stream), "{:?}", text)?;

    stream.text_end().map_err(|_| fmt::Error)
}

/**
Stream the output of a generic user-facing format as a text string.
*/
pub fn stream_display<'sval, T: fmt::Display>(
    mut stream: &mut (impl Stream<'sval> + ?Sized),
    text: T,
) -> fmt::Result {
    struct Writer<S>(S);

    impl<'a, S: Stream<'a>> Write for Writer<S> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.text_fragment_computed(s).map_err(|_| fmt::Error)?;

            Ok(())
        }
    }

    stream.text_begin(None).map_err(|_| fmt::Error)?;

    write!(Writer(&mut stream), "{}", text)?;

    stream.text_end().map_err(|_| fmt::Error)
}

impl Value for char {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        let mut buf = [0; 4];
        let value = &*self.encode_utf8(&mut buf);

        stream.text_begin(Some(value.len()))?;
        stream.text_fragment_computed(value)?;
        stream.text_end()
    }
}

impl Value for str {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.text_begin(Some(self.len()))?;
        stream.text_fragment(self)?;
        stream.text_end()
    }

    fn to_text(&self) -> Option<&str> {
        Some(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::String;

    impl Value for String {
        fn stream<'a, S: Stream<'a> + ?Sized>(&'a self, stream: &mut S) -> Result {
            (&**self).stream(stream)
        }

        #[inline]
        fn to_text(&self) -> Option<&str> {
            Some(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_cast() {
        assert_eq!(Some("a string"), "a string".to_text());
    }
}
