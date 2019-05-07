use crate::{
    std::{
        fmt::{
            self,
            Debug,
            Formatter,
            Write,
        },
        mem,
    },
    stream::{
        self,
        stack::{
            self,
            Stack,
        },
    },
    value,
};

pub(super) struct ToDebug<V>(pub(super) V);

impl<V> Debug for ToDebug<V>
where
    V: value::Value,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut stream = stream::OwnedStream::begin(Stream::new(f))?;

        stream.any(&self.0)?;
        stream.end()?;

        Ok(())
    }
}

/**
The format stream.

This stream is an alternative implementation of `std::fmt::DebugMap` and `std::fmt::DebugList`.
It should be kept up to date with changes made upstream.
*/
struct Stream<'a, 'b: 'a> {
    /**
    The inner value is kept behind an `Option` and dropped in case it's poisoned.

    This is a little convoluted, but protects the inner state from being re-used
    after a panic or error is returned. It _shouldn't_ be observable unless the
    caller implementation of `Value` is buggy, so long as it's not possible to
    get a hold of this `Stream` type directly.
    */
    inner: Option<Inner<'a, 'b>>,
}

struct Inner<'a, 'b: 'a> {
    stack: Stack,
    depth: usize,
    delim: Option<&'static str>,
    fmt: &'a mut Formatter<'b>,
}

impl<'a, 'b: 'a> Stream<'a, 'b> {
    #[inline]
    fn new(fmt: &'a mut Formatter<'b>) -> Self {
        Stream {
            inner: Some(Inner {
                stack: Stack::new(),
                depth: 0,
                delim: None,
                fmt,
            }),
        }
    }

    #[inline]
    fn and_then(
        &mut self,
        f: impl FnOnce(&mut Inner<'a, 'b>) -> Result<(), stream::Error>,
    ) -> Result<(), stream::Error> {
        self.inner = match self.inner.take() {
            Some(mut inner) => match f(&mut inner) {
                Ok(()) => Some(inner),
                Err(e) => return Err(e),
            },
            None => return Err(stream::Error::msg("attempt to use poisoned writer")),
        };

        Ok(())
    }
}

impl<'a, 'b: 'a> Inner<'a, 'b> {
    #[inline]
    fn next_delim(&self, pos: stack::Pos) -> Option<&'static str> {
        if pos.is_value() || pos.is_elem() {
            return Some(if self.is_pretty() { "," } else { ", " });
        }

        if pos.is_key() {
            return Some(": ");
        }

        return None;
    }

    #[inline]
    fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }
}

impl<'a, 'b: 'a> stream::Stream for Stream<'a, 'b> {
    #[inline]
    fn begin(&mut self) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            inner.stack.begin()?;

            Ok(())
        })
    }

    #[inline]
    fn fmt(&mut self, v: stream::Arguments) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            let pos = inner.stack.primitive()?;

            let next_delim = inner.next_delim(pos);
            if let Some(delim) = mem::replace(&mut inner.delim, next_delim) {
                inner.fmt.write_str(delim)?;
            }

            v.fmt(inner.fmt)?;

            Ok(())
        })
    }

    #[inline]
    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        self.fmt(format_args!("{:?}", v))
    }

    #[inline]
    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        self.fmt(format_args!("{:?}", v))
    }

    #[inline]
    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        self.fmt(format_args!("{:?}", v))
    }

    #[inline]
    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        self.fmt(format_args!("{:?}", v))
    }

    #[inline]
    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        self.fmt(format_args!("{:?}", v))
    }

    #[inline]
    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        self.fmt(format_args!("{:?}", v))
    }

    #[inline]
    fn none(&mut self) -> Result<(), stream::Error> {
        self.fmt(format_args!("None"))
    }

    #[inline]
    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            if inner.is_pretty() {
                inner.depth += 1;
            }

            inner.stack.seq_begin()?;

            if let Some(delim) = inner.delim.take() {
                inner.fmt.write_str(delim)?;
            }

            inner.fmt.write_char('[')?;

            Ok(())
        })
    }

    #[inline]
    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            if inner.is_pretty() {
                if !inner.stack.current().is_empty_seq() {
                    if let Some(delim) = inner.delim.take() {
                        inner.fmt.write_str(delim)?;
                    }
                }

                inner.fmt.write_char('\n')?;
                pad(&mut inner.fmt, inner.depth)?;
            }

            inner.stack.seq_elem()?;

            Ok(())
        })
    }

    #[inline]
    fn seq_end(&mut self) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            if inner.is_pretty() {
                inner.depth -= 1;

                if !inner.stack.current().is_empty_seq() {
                    if let Some(delim) = inner.delim.take() {
                        inner.fmt.write_str(delim)?;
                    }

                    inner.fmt.write_char('\n')?;
                    pad(&mut inner.fmt, inner.depth)?;
                }
            }

            let pos = inner.stack.seq_end()?;

            inner.delim = inner.next_delim(pos);

            inner.fmt.write_char(']')?;

            Ok(())
        })
    }

    #[inline]
    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            if inner.is_pretty() {
                inner.depth += 1;
            }

            inner.stack.map_begin()?;

            if let Some(delim) = inner.delim.take() {
                inner.fmt.write_str(delim)?;
            }

            inner.fmt.write_char('{')?;

            Ok(())
        })
    }

    #[inline]
    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            if inner.is_pretty() {
                if !inner.stack.current().is_empty_map() {
                    if let Some(delim) = inner.delim.take() {
                        inner.fmt.write_str(delim)?;
                    }
                }

                inner.fmt.write_char('\n')?;
                pad(&mut inner.fmt, inner.depth)?;
            }

            inner.stack.map_key()?;

            Ok(())
        })
    }

    #[inline]
    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            inner.stack.map_value()?;

            Ok(())
        })
    }

    #[inline]
    fn map_end(&mut self) -> Result<(), stream::Error> {
        self.and_then(|inner| {
            if inner.is_pretty() {
                inner.depth -= 1;

                if !inner.stack.current().is_empty_map() {
                    if let Some(delim) = inner.delim.take() {
                        inner.fmt.write_str(delim)?;
                    }

                    inner.fmt.write_char('\n')?;
                    pad(&mut inner.fmt, inner.depth)?;
                }
            }

            let pos = inner.stack.map_end()?;

            inner.delim = inner.next_delim(pos);

            inner.fmt.write_char('}')?;

            Ok(())
        })
    }

    #[inline]
    fn end(&mut self) -> Result<(), stream::Error> {
        self.and_then(|inner| inner.stack.end())
    }
}

fn pad(mut w: impl Write, amt: usize) -> fmt::Result {
    for _ in 0..amt {
        w.write_str("    ")?;
    }

    Ok(())
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    use crate::{
        std::{
            cell::RefCell,
            rc::Rc,
        },
        value::{
            self,
            Value,
        },
    };

    #[test]
    fn stream_is_poisoned_after_err() {
        struct Writer {
            poisoned: Rc<RefCell<bool>>,
        }

        impl Write for Writer {
            fn write_str(&mut self, _: &str) -> fmt::Result {
                if *self.poisoned.borrow() {
                    Err(fmt::Error)
                } else {
                    Ok(())
                }
            }
        }

        let poisoned = Rc::new(RefCell::new(false));

        struct Check {
            poisoned: Rc<RefCell<bool>>,
        }

        impl Value for Check {
            fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
                stream.seq_begin(None)?;

                // An initial write should succeed
                assert!(stream.seq_elem(1).is_ok());

                // A subsequent write should fail if the underlying stream fails
                {
                    *self.poisoned.borrow_mut() = true;
                }
                assert!(stream.seq_elem(1).is_err());

                // A subsequent write should fail even if the stream is ok again
                {
                    *self.poisoned.borrow_mut() = false;
                }
                assert!(stream.seq_elem(1).is_err());

                Ok(())
            }
        }

        let _ = write!(
            Writer {
                poisoned: poisoned.clone()
            },
            "{:?}",
            ToDebug(Check { poisoned })
        );
    }
}
