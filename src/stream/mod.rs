/*!
A stream for datastructures.

# The `Stream` trait

A [`Stream`] is a type that receives and works with abstract data-structures.

[`Value`]: ../value/trait.Value.html
*/

mod error;
mod fmt;
mod meta;
mod tag;
mod value;

pub use self::{
    error::Source,
    fmt::Arguments,
    meta::{
        map_meta,
        seq_meta,
        MapMeta,
        SeqMeta,
    },
    tag::{
        tag,
        Tag,
    },
    value::Value,
};

/**
A receiver for the structure of a value.

The `Stream` trait has a flat, stateless structure, but it may need to work with
nested values.

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
    fn fmt(&mut self, v: Arguments) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::fmt"))
    }
    #[cfg(test)]
    fn fmt(&mut self, v: Arguments) -> Result;

    /**
    Stream an error. Implementors should override this method if they
    expect to accept errors.
    */
    #[cfg(not(test))]
    fn error(&mut self, v: Source) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::error"))
    }
    #[cfg(test)]
    fn error(&mut self, v: Source) -> Result;

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
    Stream a tag.
    */
    #[cfg(not(test))]
    fn tag(&mut self, tag: Tag) -> Result {
        let _ = tag;
        Err(crate::Error::default_unsupported("Stream::tag"))
    }
    #[cfg(test)]
    fn tag(&mut self, tag: Tag) -> Result;

    /**
    Begin a map. Implementors should override this method if they
    expect to accept maps.
    */
    #[cfg(not(test))]
    fn map_begin(&mut self, meta: MapMeta) -> Result {
        let _ = meta;
        Err(crate::Error::default_unsupported("Stream::map_begin"))
    }
    #[cfg(test)]
    fn map_begin(&mut self, meta: MapMeta) -> Result;

    /**
    Begin a tagged map. Implementors should override this method if they
    expect to accept maps.
    */
    #[cfg(not(test))]
    fn tagged_map_begin(&mut self, tag: Tag, meta: MapMeta) -> Result {
        let _ = tag;
        let _ = meta;
        Err(crate::Error::default_unsupported(
            "Stream::tagged_map_begin",
        ))
    }
    #[cfg(test)]
    fn tagged_map_begin(&mut self, tag: Tag, meta: MapMeta) -> Result;

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
    End a tagged map. Implementors should override this method if they
    expect to accept maps.
    */
    #[cfg(not(test))]
    fn tagged_map_end(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::tagged_map_end"))
    }
    #[cfg(test)]
    fn tagged_map_end(&mut self) -> Result;

    /**
    Begin a sequence. Implementors should override this method if they
    expect to accept sequences.
    */
    #[cfg(not(test))]
    fn seq_begin(&mut self, meta: SeqMeta) -> Result {
        let _ = meta;
        Err(crate::Error::default_unsupported("Stream::seq_begin"))
    }
    #[cfg(test)]
    fn seq_begin(&mut self, meta: SeqMeta) -> Result;

    /**
    Begin a tagged sequence. Implementors should override this method if they
    expect to accept sequences.
    */
    #[cfg(not(test))]
    fn tagged_seq_begin(&mut self, tag: Tag, meta: SeqMeta) -> Result {
        let _ = tag;
        let _ = meta;
        Err(crate::Error::default_unsupported(
            "Stream::tagged_seq_begin",
        ))
    }
    #[cfg(test)]
    fn tagged_seq_begin(&mut self, tag: Tag, meta: SeqMeta) -> Result;

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
    End a tagged sequence. Implementors should override this method if they
    expect to accept sequences.
    */
    #[cfg(not(test))]
    fn tagged_seq_end(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::tagged_seq_end"))
    }
    #[cfg(test)]
    fn tagged_seq_end(&mut self) -> Result;

    /**
    Collect a map key.
    */
    #[cfg(not(test))]
    fn map_key_collect(&mut self, k: Value) -> Result {
        self.map_key()?;
        k.stream_owned(self)
    }
    #[cfg(test)]
    fn map_key_collect(&mut self, k: Value) -> Result;

    /**
    Collect a map value.
    */
    #[cfg(not(test))]
    fn map_value_collect(&mut self, v: Value) -> Result {
        self.map_value()?;
        v.stream_owned(self)
    }
    #[cfg(test)]
    fn map_value_collect(&mut self, v: Value) -> Result;

    /**
    Collect a sequence element.
    */
    #[cfg(not(test))]
    fn seq_elem_collect(&mut self, v: Value) -> Result {
        self.seq_elem()?;
        v.stream_owned(self)
    }
    #[cfg(test)]
    fn seq_elem_collect(&mut self, v: Value) -> Result;

    #[cfg(not(test))]
    fn fmt_borrowed(&mut self, v: Arguments<'v>) -> Result {
        self.fmt(v)
    }
    #[cfg(test)]
    fn fmt_borrowed(&mut self, v: Arguments<'v>) -> Result;

    #[cfg(not(test))]
    fn error_borrowed(&mut self, v: Source<'v>) -> Result {
        self.error(v)
    }
    #[cfg(test)]
    fn error_borrowed(&mut self, v: Source<'v>) -> Result;

    /**
    Stream a borrowed UTF-8 string slice.
    */
    #[cfg(not(test))]
    fn str_borrowed(&mut self, v: &'v str) -> Result {
        self.str(v)
    }
    #[cfg(test)]
    fn str_borrowed(&mut self, v: &'v str) -> Result;

    /**
    Stream a tag.
    */
    #[cfg(not(test))]
    fn tag_borrowed(&mut self, tag: Tag<'v>) -> Result {
        self.tag(tag)
    }
    #[cfg(test)]
    fn tag_borrowed(&mut self, tag: Tag<'v>) -> Result;

    /**
    Begin a borrowed tagged map. Implementors should override this method if they
    expect to accept maps.
    */
    #[cfg(not(test))]
    fn tagged_map_begin_borrowed(&mut self, tag: Tag<'v>, meta: MapMeta) -> Result {
        self.tagged_map_begin(tag, meta)
    }
    #[cfg(test)]
    fn tagged_map_begin_borrowed(&mut self, tag: Tag<'v>, meta: MapMeta) -> Result;

    #[cfg(not(test))]
    fn map_key_collect_borrowed(&mut self, k: Value<'v>) -> Result {
        self.map_key_collect(k)
    }
    #[cfg(test)]
    fn map_key_collect_borrowed(&mut self, k: Value<'v>) -> Result;

    #[cfg(not(test))]
    fn map_value_collect_borrowed(&mut self, v: Value<'v>) -> Result {
        self.map_value_collect(v)
    }
    #[cfg(test)]
    fn map_value_collect_borrowed(&mut self, v: Value<'v>) -> Result;

    /**
    Begin a tagged sequence. Implementors should override this method if they
    expect to accept sequences.
    */
    #[cfg(not(test))]
    fn tagged_seq_begin_borrowed(&mut self, tag: Tag<'v>, meta: SeqMeta) -> Result {
        self.tagged_seq_begin(tag, meta)
    }
    #[cfg(test)]
    fn tagged_seq_begin_borrowed(&mut self, tag: Tag<'v>, meta: SeqMeta) -> Result;

    #[cfg(not(test))]
    fn seq_elem_collect_borrowed(&mut self, v: Value<'v>) -> Result {
        self.seq_elem_collect(v)
    }
    #[cfg(test)]
    fn seq_elem_collect_borrowed(&mut self, v: Value<'v>) -> Result;
}

impl<'s, 'v, T: ?Sized> Stream<'v> for &'s mut T
where
    T: Stream<'v>,
{
    fn fmt(&mut self, v: Arguments) -> Result {
        (**self).fmt(v)
    }

    fn fmt_borrowed(&mut self, v: Arguments<'v>) -> Result {
        (**self).fmt_borrowed(v)
    }

    fn error(&mut self, v: Source) -> Result {
        (**self).error(v)
    }

    fn error_borrowed(&mut self, v: Source<'v>) -> Result {
        (**self).error_borrowed(v)
    }

    fn i64(&mut self, v: i64) -> Result {
        (**self).i64(v)
    }

    fn u64(&mut self, v: u64) -> Result {
        (**self).u64(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        (**self).i128(v)
    }

    fn u128(&mut self, v: u128) -> Result {
        (**self).u128(v)
    }

    fn f64(&mut self, v: f64) -> Result {
        (**self).f64(v)
    }

    fn bool(&mut self, v: bool) -> Result {
        (**self).bool(v)
    }

    fn char(&mut self, v: char) -> Result {
        (**self).char(v)
    }

    fn str(&mut self, v: &str) -> Result {
        (**self).str(v)
    }

    fn str_borrowed(&mut self, v: &'v str) -> Result {
        (**self).str_borrowed(v)
    }

    fn none(&mut self) -> Result {
        (**self).none()
    }

    fn map_begin(&mut self, meta: MapMeta) -> Result {
        (**self).map_begin(meta)
    }

    fn map_key(&mut self) -> Result {
        (**self).map_key()
    }

    fn map_key_collect(&mut self, k: Value) -> Result {
        (**self).map_key_collect(k)
    }

    fn map_key_collect_borrowed(&mut self, k: Value<'v>) -> Result {
        (**self).map_key_collect_borrowed(k)
    }

    fn map_value(&mut self) -> Result {
        (**self).map_value()
    }

    fn map_value_collect(&mut self, v: Value) -> Result {
        (**self).map_value_collect(v)
    }

    fn map_value_collect_borrowed(&mut self, v: Value<'v>) -> Result {
        (**self).map_value_collect_borrowed(v)
    }

    fn map_end(&mut self) -> Result {
        (**self).map_end()
    }

    fn seq_begin(&mut self, meta: SeqMeta) -> Result {
        (**self).seq_begin(meta)
    }

    fn seq_elem(&mut self) -> Result {
        (**self).seq_elem()
    }

    fn seq_elem_collect(&mut self, v: Value) -> Result {
        (**self).seq_elem_collect(v)
    }

    fn seq_elem_collect_borrowed(&mut self, v: Value<'v>) -> Result {
        (**self).seq_elem_collect_borrowed(v)
    }

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
