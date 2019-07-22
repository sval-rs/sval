/*!
A stream for datastructures.
*/

pub(crate) mod owned;
pub mod stack;

use crate::std::fmt;

#[doc(inline)]
pub use crate::Error;

pub use self::{
    fmt::Arguments,
    owned::{
        OwnedStream,
        RefMutStream,
    },
    stack::Stack,
};

/**
A receiver for the structure of a value.

The `Stream` trait has a flat, stateless structure, but it may need to work with
nested values. Implementations can use a [`Stack`] to track state for them.

The [`OwnedStream`] type is an ergonomic wrapper over a raw `Stream` that adds
the concept of [`Value`]s.

# Implementing `Stream`

A stream may choose what kinds of structures it supports by selectively
implementing methods on the trait. Other methods default to returning
[`Error::unsupported`]. Implementations may also choose to return
`Error::unsupported` for other reasons.

## Supporting primitives

The following stream can support any primitive value:

```
# struct MyStream;
use sval::{stream, Stream};

impl Stream for MyStream {
    fn fmt(&mut self, args: stream::Arguments) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn none(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }
}
```

## Supporting maps

In addition to the [methods needed for streaming primitives](#supporting-primitives),
a stream that supports maps needs to implement a few additional methods:

```
# struct MyStream;
use sval::{stream, Stream};

impl Stream for MyStream {
    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn map_value(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }
}
```

## Supporting sequences

In addition to the [methods needed for streaming primitives](#supporting-primitives),
a stream that supports sequences needs to implement a few additional methods:

```
# struct MyStream;
use sval::{stream, Stream};

impl Stream for MyStream {
    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }
}
```

## Supporting all structure

```
# struct MyStream;
use sval::{stream, Stream};

impl Stream for MyStream {
    fn fmt(&mut self, args: stream::Arguments) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn none(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn map_value(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }
}
```

[`Value`]: ../trait.Value.html
[`Error::unsupported`]: struct.Error.html#method.unsupported
*/
pub trait Stream {
    /**
    Stream a format.
    */
    fn fmt(&mut self, args: Arguments) -> Result {
        let _ = args;
        Err(Error::unsupported("Stream::fmt"))
    }

    /**
    Stream a signed integer.
    */
    fn i64(&mut self, v: i64) -> Result {
        self.i128(v as i128)
    }

    /**
    Stream an unsigned integer.
    */
    fn u64(&mut self, v: u64) -> Result {
        self.u128(v as u128)
    }

    /**
    Stream a 128bit signed integer.
    */
    fn i128(&mut self, v: i128) -> Result {
        let _ = v;
        Err(Error::unsupported("Stream::i128"))
    }

    /**
    Stream a 128bit unsigned integer.
    */
    fn u128(&mut self, v: u128) -> Result {
        let _ = v;
        Err(Error::unsupported("Stream::u128"))
    }

    /**
    Stream a floating point value.
    */
    fn f64(&mut self, v: f64) -> Result {
        let _ = v;
        Err(Error::unsupported("Stream::f64"))
    }

    /**
    Stream a boolean.
    */
    fn bool(&mut self, v: bool) -> Result {
        let _ = v;
        Err(Error::unsupported("Stream::bool"))
    }

    /**
    Stream a unicode character.
    */
    fn char(&mut self, v: char) -> Result {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }

    /**
    Stream a UTF-8 string slice.
    */
    fn str(&mut self, v: &str) -> Result {
        let _ = v;
        Err(Error::unsupported("Stream::str"))
    }

    /**
    Stream an empty value.
    */
    fn none(&mut self) -> Result {
        Err(Error::unsupported("Stream::none"))
    }

    /**
    Begin a map.
    */
    fn map_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(Error::unsupported("Stream::map_begin"))
    }

    /**
    Begin a map key.

    The key will be implicitly ended by the stream methods that follow it.
    */
    fn map_key(&mut self) -> Result {
        Err(Error::unsupported("Stream::map_key"))
    }

    /**
    Begin a map value.

    The value will be implicitly ended by the stream methods that follow it.
    */
    fn map_value(&mut self) -> Result {
        Err(Error::unsupported("Stream::map_value"))
    }

    /**
    End a map.
    */
    fn map_end(&mut self) -> Result {
        Err(Error::unsupported("Stream::map_end"))
    }

    /**
    Begin a sequence.
    */
    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(Error::unsupported("Stream::seq_begin"))
    }

    /**
    Begin a sequence element.

    The element will be implicitly ended by the stream methods that follow it.
    */
    fn seq_elem(&mut self) -> Result {
        Err(Error::unsupported("Stream::seq_elem"))
    }

    /**
    End a sequence.
    */
    fn seq_end(&mut self) -> Result {
        Err(Error::unsupported("Stream::seq_end"))
    }
}

impl<'a, T: ?Sized> Stream for &'a mut T
where
    T: Stream,
{
    #[inline]
    fn fmt(&mut self, args: Arguments) -> Result {
        (**self).fmt(args)
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
    fn map_value(&mut self) -> Result {
        (**self).map_value()
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
    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }
}

/**
The type returned by streaming methods.
*/
pub type Result = std::result::Result<(), Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_is_object_safe() {
        fn _safe(_: &mut dyn Stream) {}
    }
}
