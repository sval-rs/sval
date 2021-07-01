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

/**
The result of calling [`sval::fmt::to_debug`](fn.to_debug.html).
*/
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToDebug<V>(pub(super) V);

impl<V> Debug for ToDebug<V>
where
    V: value::Value,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        crate::stream(Stream::new(f), &self.0).map_err(crate::Error::into_fmt_error)?;

        Ok(())
    }
}

/**
The format stream.
This stream is an alternative implementation of `std::fmt::DebugMap` and `std::fmt::DebugList`.
It should be kept up to date with changes made upstream.
*/
struct Stream<'a, 'b: 'a> {
    stack: Stack,
    depth: usize,
    delim: Option<&'static str>,
    fmt: &'a mut Formatter<'b>,
}

impl<'a, 'b: 'a> Stream<'a, 'b> {
    #[inline]
    fn new(fmt: &'a mut Formatter<'b>) -> Self {
        Stream {
            stack: Stack::new(),
            depth: 0,
            delim: None,
            fmt,
        }
    }

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

    #[inline]
    fn fmt(&mut self, v: impl fmt::Debug) -> stream::Result {
        let pos = self.stack.primitive()?;

        let next_delim = self.next_delim(pos);
        if let Some(delim) = mem::replace(&mut self.delim, next_delim) {
            self.fmt.write_str(delim)?;
        }

        v.fmt(&mut self.fmt)?;

        Ok(())
    }
}

impl<'a, 'b: 'a, 'v> stream::Stream<'v> for Stream<'a, 'b> {
    #[inline]
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> stream::Result {
        self.fmt(v)
    }

    #[inline]
    fn none(&mut self) -> stream::Result {
        self.fmt(format_args!("None"))
    }

    #[inline]
    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.is_pretty() {
            self.depth += 1;
        }

        self.stack.map_begin()?;

        if let Some(delim) = self.delim.take() {
            self.fmt.write_str(delim)?;
        }

        self.fmt.write_char('{')?;

        Ok(())
    }

    #[inline]
    fn map_key(&mut self) -> stream::Result {
        if self.is_pretty() {
            if !self.stack.current().is_empty_map() {
                if let Some(delim) = self.delim.take() {
                    self.fmt.write_str(delim)?;
                }
            }

            self.fmt.write_char('\n')?;
            pad(&mut self.fmt, self.depth)?;
        }

        self.stack.map_key()?;

        Ok(())
    }

    #[inline]
    fn map_key_collect(&mut self, k: &stream::Value) -> stream::Result {
        self.map_key()?;
        k.stream(self)
    }

    #[inline]
    fn map_value(&mut self) -> stream::Result {
        self.stack.map_value()?;

        Ok(())
    }

    #[inline]
    fn map_value_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.map_value()?;
        v.stream(self)
    }

    #[inline]
    fn map_end(&mut self) -> stream::Result {
        if self.is_pretty() {
            self.depth -= 1;

            if !self.stack.current().is_empty_map() {
                if let Some(delim) = self.delim.take() {
                    self.fmt.write_str(delim)?;
                }

                self.fmt.write_char('\n')?;
                pad(&mut self.fmt, self.depth)?;
            }
        }

        let pos = self.stack.map_end()?;

        self.delim = self.next_delim(pos);

        self.fmt.write_char('}')?;

        Ok(())
    }

    #[inline]
    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.is_pretty() {
            self.depth += 1;
        }

        self.stack.seq_begin()?;

        if let Some(delim) = self.delim.take() {
            self.fmt.write_str(delim)?;
        }

        self.fmt.write_char('[')?;

        Ok(())
    }

    #[inline]
    fn seq_elem(&mut self) -> stream::Result {
        if self.is_pretty() {
            if !self.stack.current().is_empty_seq() {
                if let Some(delim) = self.delim.take() {
                    self.fmt.write_str(delim)?;
                }
            }

            self.fmt.write_char('\n')?;
            pad(&mut self.fmt, self.depth)?;
        }

        self.stack.seq_elem()?;

        Ok(())
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.seq_elem()?;
        v.stream(self)
    }

    #[inline]
    fn seq_end(&mut self) -> stream::Result {
        if self.is_pretty() {
            self.depth -= 1;

            if !self.stack.current().is_empty_seq() {
                if let Some(delim) = self.delim.take() {
                    self.fmt.write_str(delim)?;
                }

                self.fmt.write_char('\n')?;
                pad(&mut self.fmt, self.depth)?;
            }
        }

        let pos = self.stack.seq_end()?;

        self.delim = self.next_delim(pos);

        self.fmt.write_char(']')?;

        Ok(())
    }
}

fn pad(mut w: impl Write, amt: usize) -> fmt::Result {
    for _ in 0..amt {
        w.write_str("    ")?;
    }

    Ok(())
}
