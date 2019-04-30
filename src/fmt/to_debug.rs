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
    value,
    stream::{
        self,
        stack::{
            self,
            Stack,
        },
    },
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
}

// TODO: Support the indentation
impl<'a, 'b: 'a> stream::Stream for Stream<'a, 'b> {
    #[inline]
    fn begin(&mut self) -> Result<(), stream::Error> {
        self.stack.begin()?;

        Ok(())
    }

    #[inline]
    fn fmt(&mut self, v: stream::Arguments) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        let next_delim = self.next_delim(pos);
        if let Some(delim) = mem::replace(&mut self.delim, next_delim) {
            self.fmt.write_str(delim)?;
        }

        write!(self.fmt, "{:?}", v)?;

        Ok(())
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
    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        if self.is_pretty() {
            if self.stack.current().has_fields() {
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
    fn seq_end(&mut self) -> Result<(), stream::Error> {
        if self.is_pretty() {
            self.depth -= 1;

            if self.stack.current().has_fields() {
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

    #[inline]
    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
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
    fn map_key(&mut self) -> Result<(), stream::Error> {
        if self.is_pretty() {
            if self.stack.current().has_fields() {
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
    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.stack.map_value()?;

        Ok(())
    }

    #[inline]
    fn map_end(&mut self) -> Result<(), stream::Error> {
        if self.is_pretty() {
            self.depth -= 1;

            if self.stack.current().has_fields() {
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
    fn end(&mut self) -> Result<(), stream::Error> {
        self.stack.end()?;

        Ok(())
    }
}

fn pad(mut w: impl Write, amt: usize) -> fmt::Result {
    for _ in 0..amt {
        w.write_str("    ")?;
    }

    Ok(())
}
