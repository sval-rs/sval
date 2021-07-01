/*!
A stream for datastructures.

# The `Stream` trait

A [`Stream`] is a type that receives and works with abstract data-structures.

## Streams without state

A `Stream` might only care about a single kind of value.

## Streams with state

There are more methods on `Stream` that can be overriden for more complex
datastructures like sequences and maps. The following example uses a
[`stream::Stack`] to track the state of any sequences and maps and ensure
they're valid.

By default, the `Stack` type has a fixed depth. That means deeply nested
structures aren't supported. See the [`stream::Stack`] type for more details.

[`Value`]: ../value/trait.Value.html
[`stream::Stack`]: stack/struct.Stack.html
*/

mod error;
mod fmt;
pub mod stack;
mod value;

pub use self::{
    error::Source,
    fmt::Arguments,
    stack::Stack,
    value::Value,
};

/**
A receiver for the structure of a value.

The `Stream` trait has a flat, stateless structure, but it may need to work with
nested values. Implementations can use a [`Stack`] to track state for them.

# Implementing `Stream`

A stream may choose what kinds of structures it supports by selectively
implementing methods on the trait. Other methods default to returning
[`Error::unsupported`]. Implementations may also choose to return
`Error::unsupported` for other reasons.

[`Value`]: ../trait.Value.html
[`Error::unsupported`]: struct.Error.html#method.unsupported
*/
pub trait Stream<'v> {
    /**
    Stream a formattable type. Implementors should override this method if they
    expect to accept formattable types.
    */
    #[cfg(not(test))]
    #[inline]
    fn fmt(&mut self, v: &Arguments) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::fmt"))
    }
    #[cfg(test)]
    fn fmt(&mut self, v: &Arguments) -> Result;

    /**
    Stream an error. Implementors should override this method if they
    expect to accept errors.
    */
    #[cfg(not(test))]
    #[inline]
    fn error(&mut self, v: &Source) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::error"))
    }
    #[cfg(test)]
    fn error(&mut self, v: &Source) -> Result;

    /**
    Stream a signed integer. Implementors should override this method if they
    expect to accept signed integers.
    */
    #[cfg(not(test))]
    fn i64(&mut self, v: i64) -> Result {
        self.i128(v as i128)
    }
    #[cfg(test)]
    fn i64(&mut self, v: i64) -> Result;

    /**
    Stream an unsigned integer. Implementors should override this method if they
    expect to accept unsigned integers.
    */
    #[cfg(not(test))]
    fn u64(&mut self, v: u64) -> Result {
        self.u128(v as u128)
    }
    #[cfg(test)]
    fn u64(&mut self, v: u64) -> Result;

    /**
    Stream a 128bit signed integer. Implementors should override this method if they
    expect to accept 128bit signed integers.
    */
    #[cfg(not(test))]
    fn i128(&mut self, v: i128) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::i128"))
    }
    #[cfg(test)]
    fn i128(&mut self, v: i128) -> Result;

    /**
    Stream a 128bit unsigned integer. Implementors should override this method if they
    expect to accept 128bit unsigned integers.
    */
    #[cfg(not(test))]
    fn u128(&mut self, v: u128) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::u128"))
    }
    #[cfg(test)]
    fn u128(&mut self, v: u128) -> Result;

    /**
    Stream a floating point value. Implementors should override this method if they
    expect to accept floating point numbers.
    */
    #[cfg(not(test))]
    fn f64(&mut self, v: f64) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::f64"))
    }
    #[cfg(test)]
    fn f64(&mut self, v: f64) -> Result;

    /**
    Stream a boolean. Implementors should override this method if they
    expect to accept booleans.
    */
    #[cfg(not(test))]
    fn bool(&mut self, v: bool) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::bool"))
    }
    #[cfg(test)]
    fn bool(&mut self, v: bool) -> Result;

    /**
    Stream a unicode character.
    */
    #[cfg(not(test))]
    #[inline]
    fn char(&mut self, v: char) -> Result {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }
    #[cfg(test)]
    fn char(&mut self, v: char) -> Result;

    /**
    Stream a UTF-8 string slice. Implementors should override this method if they
    expect to accept strings.
    */
    #[cfg(not(test))]
    fn str(&mut self, v: &str) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::str"))
    }
    #[cfg(test)]
    fn str(&mut self, v: &str) -> Result;

    /**
    Stream an empty value. Implementors should override this method if they
    expect to accept empty values.
    */
    #[cfg(not(test))]
    fn none(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::none"))
    }
    #[cfg(test)]
    fn none(&mut self) -> Result;

    /**
    Begin a map. Implementors should override this method if they
    expect to accept maps.
    */
    #[cfg(not(test))]
    fn map_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(crate::Error::default_unsupported("Stream::map_begin"))
    }
    #[cfg(test)]
    fn map_begin(&mut self, len: Option<usize>) -> Result;

    /**
    Begin a map key. Implementors should override this method if they
    expect to accept maps.

    The key will be implicitly ended by the stream methods that follow it.
    */
    #[cfg(not(test))]
    fn map_key(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::map_key"))
    }
    #[cfg(test)]
    fn map_key(&mut self) -> Result;

    /**
    Begin a map value. Implementors should override this method if they
    expect to accept maps.

    The value will be implicitly ended by the stream methods that follow it.
    */
    #[cfg(not(test))]
    fn map_value(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::map_value"))
    }
    #[cfg(test)]
    fn map_value(&mut self) -> Result;

    /**
    End a map. Implementors should override this method if they
    expect to accept maps.
    */
    #[cfg(not(test))]
    fn map_end(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::map_end"))
    }
    #[cfg(test)]
    fn map_end(&mut self) -> Result;

    /**
    Begin a sequence. Implementors should override this method if they
    expect to accept sequences.
    */
    #[cfg(not(test))]
    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(crate::Error::default_unsupported("Stream::seq_begin"))
    }
    #[cfg(test)]
    fn seq_begin(&mut self, len: Option<usize>) -> Result;

    /**
    Begin a sequence element. Implementors should override this method if they
    expect to accept sequences.

    The element will be implicitly ended by the stream methods that follow it.
    */
    #[cfg(not(test))]
    fn seq_elem(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::seq_elem"))
    }
    #[cfg(test)]
    fn seq_elem(&mut self) -> Result;

    /**
    End a sequence. Implementors should override this method if they
    expect to accept sequences.
    */
    #[cfg(not(test))]
    fn seq_end(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::seq_end"))
    }
    #[cfg(test)]
    fn seq_end(&mut self) -> Result;

    /**
    Collect a map key.
    */
    #[cfg(not(test))]
    #[inline]
    fn map_key_collect(&mut self, k: &Value) -> Result {
        self.map_key()?;
        k.stream_owned(self).map(|_| ())
    }
    #[cfg(test)]
    fn map_key_collect(&mut self, k: &Value) -> Result;

    /**
    Collect a map value.
    */
    #[cfg(not(test))]
    #[inline]
    fn map_value_collect(&mut self, v: &Value) -> Result {
        self.map_value()?;
        v.stream_owned(self).map(|_| ())
    }
    #[cfg(test)]
    fn map_value_collect(&mut self, v: &Value) -> Result;

    /**
    Collect a sequence element.
    */
    #[cfg(not(test))]
    #[inline]
    fn seq_elem_collect(&mut self, v: &Value) -> Result {
        self.seq_elem()?;
        v.stream_owned(self).map(|_| ())
    }
    #[cfg(test)]
    fn seq_elem_collect(&mut self, v: &Value) -> Result;

    #[cfg(not(test))]
    #[inline]
    fn fmt_borrowed(&mut self, v: &Arguments<'v>) -> Result {
        self.fmt(v)
    }
    #[cfg(test)]
    fn fmt_borrowed(&mut self, v: &Arguments<'v>) -> Result;

    #[cfg(not(test))]
    #[inline]
    fn error_borrowed(&mut self, v: &Source<'v>) -> Result {
        self.error(v)
    }
    #[cfg(test)]
    fn error_borrowed(&mut self, v: &Source<'v>) -> Result;

    /**
    Stream a borrowed UTF-8 string slice.
    */
    #[cfg(not(test))]
    fn str_borrowed(&mut self, v: &'v str) -> Result {
        self.str(v)
    }
    #[cfg(test)]
    fn str_borrowed(&mut self, v: &'v str) -> Result;

    #[cfg(not(test))]
    #[inline]
    fn map_key_collect_borrowed(&mut self, k: &Value<'v>) -> Result {
        self.map_key_collect(k)
    }
    #[cfg(test)]
    fn map_key_collect_borrowed(&mut self, k: &Value<'v>) -> Result;

    #[cfg(not(test))]
    #[inline]
    fn map_value_collect_borrowed(&mut self, v: &Value<'v>) -> Result {
        self.map_value_collect(v)
    }
    #[cfg(test)]
    fn map_value_collect_borrowed(&mut self, v: &Value<'v>) -> Result;

    #[cfg(not(test))]
    #[inline]
    fn seq_elem_collect_borrowed(&mut self, v: &Value<'v>) -> Result {
        self.seq_elem_collect(v)
    }
    #[cfg(test)]
    fn seq_elem_collect_borrowed(&mut self, v: &Value<'v>) -> Result;
}

impl<'s, 'v, T: ?Sized> Stream<'v> for &'s mut T
where
    T: Stream<'v>,
{
    #[inline]
    fn fmt(&mut self, v: &Arguments) -> Result {
        (**self).fmt(v)
    }

    #[inline]
    fn fmt_borrowed(&mut self, v: &Arguments<'v>) -> Result {
        (**self).fmt_borrowed(v)
    }

    #[inline]
    fn error(&mut self, v: &Source) -> Result {
        (**self).error(v)
    }

    #[inline]
    fn error_borrowed(&mut self, v: &Source<'v>) -> Result {
        (**self).error_borrowed(v)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> Result {
        (**self).i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> Result {
        (**self).u64(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> Result {
        (**self).i128(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> Result {
        (**self).u128(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> Result {
        (**self).f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> Result {
        (**self).bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> Result {
        (**self).char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> Result {
        (**self).str(v)
    }

    #[inline]
    fn str_borrowed(&mut self, v: &'v str) -> Result {
        (**self).str_borrowed(v)
    }

    #[inline]
    fn none(&mut self) -> Result {
        (**self).none()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> Result {
        (**self).map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> Result {
        (**self).map_key()
    }

    #[inline]
    fn map_key_collect(&mut self, k: &Value) -> Result {
        (**self).map_key_collect(k)
    }

    #[inline]
    fn map_key_collect_borrowed(&mut self, k: &Value<'v>) -> Result {
        (**self).map_key_collect_borrowed(k)
    }

    #[inline]
    fn map_value(&mut self) -> Result {
        (**self).map_value()
    }

    #[inline]
    fn map_value_collect(&mut self, v: &Value) -> Result {
        (**self).map_value_collect(v)
    }

    #[inline]
    fn map_value_collect_borrowed(&mut self, v: &Value<'v>) -> Result {
        (**self).map_value_collect_borrowed(v)
    }

    #[inline]
    fn map_end(&mut self) -> Result {
        (**self).map_end()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        (**self).seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> Result {
        (**self).seq_elem()
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: &Value) -> Result {
        (**self).seq_elem_collect(v)
    }

    #[inline]
    fn seq_elem_collect_borrowed(&mut self, v: &Value<'v>) -> Result {
        (**self).seq_elem_collect_borrowed(v)
    }

    #[inline]
    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }
}

/**
The type returned by streaming methods.
*/
pub type Result<T = ()> = crate::std::result::Result<T, crate::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_is_object_safe() {
        fn _safe(_: &mut dyn Stream) {}
    }
}
