/*!
A stream for datastructures.

# The `Stream` trait

A [`Stream`] is a type that receives and works with abstract data-structures.

## Streams without state

Implement the `Stream` trait to visit the structure of a [`Value`]:

```
use sval::stream::{self, Stream};

struct Fmt;

impl Stream for Fmt {
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        println!("{}", v);

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn none(&mut self) -> stream::Result {
        self.fmt(stream::Arguments::debug(&()))
    }
}
```

A `Stream` might only care about a single kind of value.
The following example overrides the provided `u64` method
to see whether a given value is a `u64`:

```
use sval::{
    value::Value,
    stream::{self, Stream, OwnedStream},
};

assert!(is_u64(42u64));

pub fn is_u64(v: impl Value) -> bool {
    OwnedStream::stream(IsU64(None), v)
        .map(|is_u64| is_u64.0.is_some())
        .unwrap_or(false)
}

struct IsU64(Option<u64>);
impl Stream for IsU64 {
    fn u64(&mut self, v: u64) -> stream::Result {
        self.0 = Some(v);

        Ok(())
    }
}
```

## Streams with state

There are more methods on `Stream` that can be overriden for more complex
datastructures like sequences and maps. The following example uses a
[`stream::Stack`] to track the state of any sequences and maps and ensure
they're valid:

```
use std::{fmt, mem};
use sval::stream::{self, stack, Stream, Stack};

struct Fmt {
    stack: stream::Stack,
    delim: &'static str,
}

impl Fmt {
    fn next_delim(pos: stack::Pos) -> &'static str {
        if pos.is_key() {
            return ": ";
        }

        if pos.is_value() || pos.is_elem() {
            return ", ";
        }

        return "";
    }
}

impl Stream for Fmt {
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        let pos = self.stack.primitive()?;

        let delim = mem::replace(&mut self.delim, Self::next_delim(pos));
        print!("{}{:?}", delim, v);

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.fmt(stream::Arguments::debug(&v))
    }

    fn none(&mut self) -> stream::Result {
        self.fmt(stream::Arguments::debug(&()))
    }

    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        self.stack.seq_begin()?;

        let delim = mem::replace(&mut self.delim, "");
        print!("{}[", delim);

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.stack.seq_elem()?;

        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
        let pos = self.stack.seq_end()?;

        self.delim = Self::next_delim(pos);
        print!("]");

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        self.stack.map_begin()?;

        let delim = mem::replace(&mut self.delim, "");
        print!("{}{{", delim);

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
        self.stack.map_key()?;

        Ok(())
    }

    fn map_value(&mut self) -> stream::Result {
        self.stack.map_value()?;

        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
        let pos = self.stack.map_end()?;

        self.delim = Self::next_delim(pos);
        print!("}}");

        Ok(())
    }
}
```

By default, the `Stack` type has a fixed depth. That means deeply nested
structures aren't supported. See the [`stream::Stack`] type for more details.

[`Value`]: ../value/trait.Value.html
[`stream::Stack`]: stack/struct.Stack.html
*/

pub(crate) mod owned;
pub mod stack;

use crate::std::{
    fmt,
};

pub use self::{
    owned::{
        OwnedStream,
        RefMutStream,
    },
    stack::Stack,
};

/**
A formattable value.
*/
pub struct Arguments<'a>(ArgumentsInner<'a>);

enum ArgumentsInner<'a> {
    Debug(&'a dyn fmt::Debug),
    Display(&'a dyn fmt::Display),
    Args(fmt::Arguments<'a>),
}

impl<'a> Arguments<'a> {
    /**
    Capture standard format arguments.

    Prefer the [`debug`] and [`display`] methods to create 
    `Arguments` over passing them through `format_args`,
    because `format_args` will clobber any flags a stream
    might want to format these arguments with.
    */
    pub fn new(v: fmt::Arguments<'a>) -> Self {
        Arguments(ArgumentsInner::Args(v))
    }

    /**
    Capture arguments from a debuggable value.
    */
    pub fn debug(v: &'a impl fmt::Debug) -> Self {
        Arguments(ArgumentsInner::Debug(v))
    }

    /**
    Capture arguments from a displayable value.
    */
    pub fn display(v: &'a impl fmt::Display) -> Self {
        Arguments(ArgumentsInner::Display(v))
    }
}

impl<'a> From<fmt::Arguments<'a>> for Arguments<'a> {
    fn from(v: fmt::Arguments<'a>) -> Self {
        Arguments(ArgumentsInner::Args(v))
    }
}

impl<'a> From<&'a dyn fmt::Debug> for Arguments<'a> {
    fn from(v: &'a dyn fmt::Debug) -> Self {
        Arguments(ArgumentsInner::Debug(v))
    }
}

impl<'a> From<&'a dyn fmt::Display> for Arguments<'a> {
    fn from(v: &'a dyn fmt::Display) -> Self {
        Arguments(ArgumentsInner::Display(v))
    }
}

impl<'a> fmt::Debug for Arguments<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ArgumentsInner::Debug(v) => v.fmt(f),
            ArgumentsInner::Display(v) => v.fmt(f),
            ArgumentsInner::Args(v) => v.fmt(f),
        }
    }
}

impl<'a> fmt::Display for Arguments<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ArgumentsInner::Debug(v) => v.fmt(f),
            ArgumentsInner::Display(v) => v.fmt(f),
            ArgumentsInner::Args(v) => v.fmt(f),
        }
    }
}

/**
A streamable error.

This type shouldn't be confused with [`sval::Error`], which is
used to communicate errors back to callers.
The purpose of the `Source` type is to let streams serialize
error types, that may have backtraces and other metadata.

`Source`s can only be created when the `std` feature is available,
but streams can still work with them by formatting them or passing
them along even in no-std environments where the `Error` trait isn't available.
*/
pub struct Source<'a> {
    #[cfg(feature = "std")]
    inner: self::std_support::SourceError<'a>,
    #[cfg(not(feature = "std"))]
    _marker: crate::std::marker::PhantomData<&'a dyn crate::std::any::Any>,
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<'a> Source<'a> {
    pub(crate) fn empty() -> Self {
        Source {
            _marker: Default::default(),
        }
    }
}

impl<'a> fmt::Debug for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            fmt::Debug::fmt(&self.inner, f)
        }

        #[cfg(not(feature = "std"))]
        {
            f.debug_struct("Source").finish()
        }
    }
}

impl<'a> fmt::Display for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            fmt::Display::fmt(&self.inner, f)
        }

        #[cfg(not(feature = "std"))]
        {
            f.debug_struct("Source").finish()
        }
    }
}

/**
A receiver for the structure of a value.

The `Stream` trait has a flat, stateless structure, but it may need to work with
nested values. Implementations can use a [`Stack`] to track state for them.

The [`OwnedStream`] type is an ergonomic wrapper over a raw `Stream` that adds
the concept of [`Value`](../value/trait.Value.html)s.

# Implementing `Stream`

A stream may choose what kinds of structures it supports by selectively
implementing methods on the trait. Other methods default to returning
[`Error::unsupported`]. Implementations may also choose to return
`Error::unsupported` for other reasons.

## Supporting primitives

The following stream can support any primitive value:

```
# struct MyStream;
use sval::stream::{self, Stream};

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
use sval::stream::{self, Stream};

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
use sval::stream::{self, Stream};

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
use sval::stream::{self, Stream};

impl Stream for MyStream {
    fn fmt(&mut self, args: stream::Arguments) -> stream::Result {
#       /*
        ..
#       */

        Ok(())
    }

    fn error(&mut self, source: stream::Source) -> stream::Result {
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
    Stream a debuggable type.
    */
    #[cfg(not(test))]
    fn fmt(&mut self, v: Arguments) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::fmt"))
    }
    #[cfg(test)]
    fn fmt(&mut self, v: Arguments) -> Result;

    /**
    Stream an error.
    */
    #[cfg(not(test))]
    fn error(&mut self, v: Source) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::error"))
    }
    #[cfg(test)]
    fn error(&mut self, v: Source) -> Result;

    /**
    Stream a signed integer.
    */
    #[cfg(not(test))]
    fn i64(&mut self, v: i64) -> Result {
        self.i128(v as i128)
    }
    #[cfg(test)]
    fn i64(&mut self, v: i64) -> Result;

    /**
    Stream an unsigned integer.
    */
    #[cfg(not(test))]
    fn u64(&mut self, v: u64) -> Result {
        self.u128(v as u128)
    }
    #[cfg(test)]
    fn u64(&mut self, v: u64) -> Result;

    /**
    Stream a 128bit signed integer.
    */
    #[cfg(not(test))]
    fn i128(&mut self, v: i128) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::i128"))
    }
    #[cfg(test)]
    fn i128(&mut self, v: i128) -> Result;

    /**
    Stream a 128bit unsigned integer.
    */
    #[cfg(not(test))]
    fn u128(&mut self, v: u128) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::u128"))
    }
    #[cfg(test)]
    fn u128(&mut self, v: u128) -> Result;

    /**
    Stream a floating point value.
    */
    #[cfg(not(test))]
    fn f64(&mut self, v: f64) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::f64"))
    }
    #[cfg(test)]
    fn f64(&mut self, v: f64) -> Result;

    /**
    Stream a boolean.
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
    Stream a UTF-8 string slice.
    */
    #[cfg(not(test))]
    fn str(&mut self, v: &str) -> Result {
        let _ = v;
        Err(crate::Error::default_unsupported("Stream::str"))
    }
    #[cfg(test)]
    fn str(&mut self, v: &str) -> Result;

    /**
    Stream an empty value.
    */
    #[cfg(not(test))]
    fn none(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::none"))
    }
    #[cfg(test)]
    fn none(&mut self) -> Result;

    /**
    Begin a map.
    */
    #[cfg(not(test))]
    fn map_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(crate::Error::default_unsupported("Stream::map_begin"))
    }
    #[cfg(test)]
    fn map_begin(&mut self, len: Option<usize>) -> Result;

    /**
    Begin a map key.

    The key will be implicitly ended by the stream methods that follow it.
    */
    #[cfg(not(test))]
    fn map_key(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::map_key"))
    }
    #[cfg(test)]
    fn map_key(&mut self) -> Result;

    /**
    Begin a map value.

    The value will be implicitly ended by the stream methods that follow it.
    */
    #[cfg(not(test))]
    fn map_value(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::map_value"))
    }
    #[cfg(test)]
    fn map_value(&mut self) -> Result;

    /**
    End a map.
    */
    #[cfg(not(test))]
    fn map_end(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::map_end"))
    }
    #[cfg(test)]
    fn map_end(&mut self) -> Result;

    /**
    Begin a sequence.
    */
    #[cfg(not(test))]
    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(crate::Error::default_unsupported("Stream::seq_begin"))
    }
    #[cfg(test)]
    fn seq_begin(&mut self, len: Option<usize>) -> Result;

    /**
    Begin a sequence element.

    The element will be implicitly ended by the stream methods that follow it.
    */
    #[cfg(not(test))]
    fn seq_elem(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::seq_elem"))
    }
    #[cfg(test)]
    fn seq_elem(&mut self) -> Result;

    /**
    End a sequence.
    */
    #[cfg(not(test))]
    fn seq_end(&mut self) -> Result {
        Err(crate::Error::default_unsupported("Stream::seq_end"))
    }
    #[cfg(test)]
    fn seq_end(&mut self) -> Result;
}

impl<'a, T: ?Sized> Stream for &'a mut T
where
    T: Stream,
{
    #[inline]
    fn fmt(&mut self, v: Arguments) -> Result {
        (**self).fmt(v)
    }

    #[inline]
    fn error(&mut self, v: Source) -> Result {
        (**self).error(v)
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
pub type Result = crate::std::result::Result<(), crate::Error>;

#[cfg(feature = "std")]
mod std_support {
    use crate::std::{
        fmt,
        error::Error,
        marker::PhantomData,
        mem,
    };

    use super::Source;

    pub(super) struct SourceError<'a> {
        extended: ExtendedLifetimeError,
        _marker: PhantomData<&'a dyn Error>,
    }

    impl<'a> fmt::Debug for SourceError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self.extended, f)
        }
    }

    impl<'a> fmt::Display for SourceError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.extended, f)
        }
    }

    /**
    A wrapper over an error type with an artificially extended lifetime.

    Borrows of this value are returned by `SourceError` but it's important
    that callers can't get at the inner `&'static dyn Error` directly. They
    also can't downcast the value to a `ExtendedLifetimeError` or the inner
    value even if it does support downcasting, but they can iterate its causes
    and grab a backtrace.
    */
    struct ExtendedLifetimeError(&'static dyn Error);

    impl fmt::Debug for ExtendedLifetimeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self.0, f)
        }
    }

    impl fmt::Display for ExtendedLifetimeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self.0, f)
        }
    }

    impl Error for ExtendedLifetimeError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.0.source()
        }

        // NOTE: Once backtraces are stable, add them here too
    }

    impl<'a> Error for &'a ExtendedLifetimeError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.0.source()
        }

        // NOTE: Once backtraces are stable, add them here too
    }

    impl<'a> Source<'a> {
        /**
        Capture an error source from a standard error.

        This method is only available when the `std` feature is enabled.
        */
        pub fn new(err: &'a impl Error) -> Self {
            Source::from(err as &'a dyn Error)
        }

        /**
        Get the value of the underlying error.
        */
        pub fn get(&self) -> &(dyn Error + 'static) {
            &self.inner.extended
        }

        /**
        Unwrap the inner error.
        */
        pub(crate) fn to_error<'b>(&'b self) -> impl Error + 'b {
            &self.inner.extended
        }
    }

    impl<'a> From<&'a dyn Error> for Source<'a> {
        fn from(err: &'a dyn Error) -> Self {
            Source {
                inner: SourceError {
                    // SAFETY: We're careful not to expose the actual value with the fake lifetime
                    // Calling code can only interact with it through an arbitrarily short borrow
                    // that's bound to `'a` from `self`, which is the real McCoy lifetime of the error
                    extended: ExtendedLifetimeError(unsafe { mem::transmute::<&'a dyn Error, &'static dyn Error>(err) }),
                    _marker: PhantomData,
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_is_object_safe() {
        fn _safe(_: &mut dyn Stream) {}
    }
}
