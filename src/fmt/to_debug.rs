use crate::{
    std::fmt::{
        self,
        Debug,
        Formatter,
        Write,
    },
    stream,
    value,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToDebug<V>(pub(super) V);

impl<V> Debug for ToDebug<V>
where
    V: value::Value,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        crate::stream(&mut Stream::new(f), &self.0).map_err(crate::Error::into_fmt_error)?;

        Ok(())
    }
}

struct Stream<'a, 'b: 'a> {
    depth: usize,
    is_current_depth_empty: bool,
    fmt: &'a mut Formatter<'b>,
}

impl<'a, 'b: 'a> Stream<'a, 'b> {
    fn new(fmt: &'a mut Formatter<'b>) -> Self {
        Stream {
            depth: 0,
            is_current_depth_empty: false,
            fmt,
        }
    }

    fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }

    fn fmt(&mut self, v: impl fmt::Debug) -> stream::Result {
        v.fmt(&mut self.fmt)?;

        Ok(())
    }
}

impl<'a, 'b: 'a, 'v> stream::Stream<'v> for Stream<'a, 'b> {
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.fmt(v)
    }

    fn fmt_borrowed(&mut self, v: stream::Arguments<'v>) -> stream::Result {
        self.fmt(v)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.fmt(v)
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.fmt(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.fmt(v)
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.fmt(v)
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.fmt(v)
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.fmt(v)
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.fmt(v)
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.fmt(v)
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.fmt(v)
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.fmt(v)
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.fmt(v)
    }

    fn none(&mut self) -> stream::Result {
        self.fmt(format_args!("None"))
    }

    fn map_begin(&mut self, _: stream::MapMeta) -> stream::Result {
        self.is_current_depth_empty = true;
        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('{')?;

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
        if self.is_pretty() {
            if !self.is_current_depth_empty {
                self.fmt.write_char(',')?;
            }

            self.fmt.write_char('\n')?;
            pad(&mut self.fmt, self.depth)?;
        } else if !self.is_current_depth_empty {
            self.fmt.write_str(", ")?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.map_key()?;
        k.stream(self)
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.fmt.write_str(": ")?;

        Ok(())
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.map_value()?;
        v.stream(self)
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        if self.is_pretty() {
            self.depth -= 1;

            if !self.is_current_depth_empty {
                self.fmt.write_str(",\n")?;
                pad(&mut self.fmt, self.depth)?;
            }
        }

        self.fmt.write_char('}')?;

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_begin(&mut self, _: stream::SeqMeta) -> stream::Result {
        self.is_current_depth_empty = true;

        if self.is_pretty() {
            self.depth += 1;
        }

        self.fmt.write_char('[')?;

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
        if self.is_pretty() {
            if !self.is_current_depth_empty {
                self.fmt.write_char(',')?;
            }

            self.fmt.write_char('\n')?;
            pad(&mut self.fmt, self.depth)?;
        } else if !self.is_current_depth_empty {
            self.fmt.write_str(", ")?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.seq_elem()?;
        v.stream(self)
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        if self.is_pretty() {
            self.depth -= 1;

            if !self.is_current_depth_empty {
                self.fmt.write_str(",\n")?;
                pad(&mut self.fmt, self.depth)?;
            }
        }

        self.fmt.write_char(']')?;

        self.is_current_depth_empty = false;

        Ok(())
    }
}

fn pad(mut w: impl Write, amt: usize) -> fmt::Result {
    for _ in 0..amt {
        w.write_str("    ")?;
    }

    Ok(())
}
