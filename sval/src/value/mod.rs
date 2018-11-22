#[doc(inline)]
pub use crate::Error;

use crate::{
    error::bail,
    stream,
};

use std::fmt;

pub trait Value {
    fn stream(&self, stream: Stream) -> Result<(), Error>;
}

/**
A value stream.
*/
pub struct Stream<'a> {
    stack: Stack,
    stream: &'a mut dyn stream::Stream,
}

impl<'a> Stream<'a> {
    /**
    Begin a new value stream.
    */
    #[inline]
    pub(crate) fn begin(stream: &'a mut dyn stream::Stream) -> Self {
        Stream {
            stack: Stack::default(),
            stream,
        }
    }

    /**
    Stream format arguments.
    */
    #[inline]
    pub fn fmt(&mut self, f: std::fmt::Arguments) -> Result<(), Error> {
        let expect = self.stack.primitive()?;
        self.stream.fmt(expect, f)?;

        Ok(())
    }

    /**
    Stream an unsigned integer.
    */
    #[inline]
    pub fn u64(&mut self, v: u64) -> Result<(), Error> {
        let expect = self.stack.primitive()?;
        self.stream.u64(expect, v)?;

        Ok(())
    }

    /**
    Begin a map.
    */
    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> Result<&mut Stream<'a>, Error> {
        let expect = self.stack.map_begin()?;
        self.stream.map_begin(expect, len)?;

        Ok(self)
    }

    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.key()?;

        Ok(self)
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.value()?;

        Ok(self)
    }

    /**
    End a map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<&mut Stream<'a>, Error> {
        let expect = self.stack.map_end()?;
        self.stream.map_end(expect)?;

        Ok(self)
    }

    /**
    End the stream.
    */
    #[inline]
    pub fn end(mut self) -> Result<(), Error> {
        self.stack.end()?;
        self.stream.end()
    }
}

#[derive(Clone)]
struct Stack {
    inner: [Slot; Stack::SIZE],
    len: usize,
}

impl Stack {
    const SIZE: usize = 8;
    const MAX_LEN: usize = Self::SIZE - 1;
}

#[derive(Clone, Copy)]
struct Slot(u8);

impl Slot {
    const EMPTY: u8 = 0b00000000;

    const DONE: u8 = 0b00000001;

    const ROOT: u8 = 0b10000000;
    const MAP: u8 = 0b01000000;
    const SEQ: u8 = 0b00100000;

    const KEY: u8 = 0b00010000;
    const VAL: u8 = 0b00001000;
    const ELEM: u8 = 0b00000100;

    const MASK_EXPECT: u8 = 0b10011100;

    const MAP_KEY: u8 = Self::MAP | Self::KEY;
    const MAP_KEY_DONE: u8 = Self::MAP_KEY | Self::DONE;

    const MAP_VAL: u8 = Self::MAP | Self::VAL;
    const MAP_VAL_DONE: u8 = Self::MAP_VAL | Self::DONE;
    const SEQ_ELEM: u8 = Self::SEQ | Self::ELEM;

    fn root() -> Self {
        Slot(Slot::ROOT)
    }

    #[inline]
    fn expect(&self) -> stream::Expect {
        match self.0 & Slot::MASK_EXPECT {
            Slot::ROOT => stream::Expect::Root,
            Slot::KEY => stream::Expect::Key,
            Slot::VAL => stream::Expect::Value,
            Slot::ELEM => stream::Expect::Elem,
            _ => unreachable!(),
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    fn new() -> Self {
        Stack {
            inner: [Slot::root(); Self::SIZE],
            len: 0,
        }
    }

    #[inline]
    fn current_mut(&mut self) -> &mut Slot {
        unsafe { self.inner.get_unchecked_mut(self.len) }
    }

    #[inline]
    fn current(&self) -> Slot {
        unsafe { *self.inner.get_unchecked(self.len) }
    }

    #[inline]
    fn primitive(&mut self) -> Result<stream::Expect, Error> {
        let mut curr = self.current_mut();

        match curr.0 & Slot::DONE {
            Slot::EMPTY => {
                curr.0 |= Slot::DONE;

                Ok(curr.expect())
            }
            _ => bail(format!("invalid attempt to write primitive: {:?}", curr)),
        }
    }

    #[inline]
    fn map_begin(&mut self) -> Result<stream::Expect, Error> {
        if self.len >= Self::MAX_LEN {
            bail("nesting limit reached")?;
        }

        let curr = self.current();

        match curr.0 {
            Slot::ROOT | Slot::MAP_KEY | Slot::MAP_VAL | Slot::SEQ_ELEM => {
                self.len += 1;
                self.current_mut().0 = Slot::MAP;

                Ok(curr.expect())
            }
            _ => bail(format!("invalid attempt to begin map: {:?}", curr)),
        }
    }

    #[inline]
    fn key(&mut self) -> Result<(), Error> {
        let mut curr = self.current_mut();

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                curr.0 = Slot::MAP_KEY;

                Ok(())
            }
            _ => bail(format!("invalid attempt to begin key: {:?}", curr)),
        }
    }

    #[inline]
    fn value(&mut self) -> Result<(), Error> {
        let mut curr = self.current_mut();

        match curr.0 {
            Slot::MAP_KEY_DONE => {
                curr.0 = Slot::MAP_VAL;

                Ok(())
            }
            _ => bail(format!("invalid attempt to begin value: {:?}", curr)),
        }
    }

    #[inline]
    fn map_end(&mut self) -> Result<stream::Expect, Error> {
        let curr = self.current();

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                self.len -= 1;

                let mut curr = self.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.expect())
            }
            _ => bail(format!("invalid attempt to end map: {:?}", curr)),
        }
    }

    #[inline]
    fn end(&mut self) -> Result<(), Error> {
        ensure!(self.len == 0, "stack is not empty")?;

        Ok(())
    }
}

impl fmt::Debug for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();

        if self.0 & Slot::ROOT != 0 {
            list.entry(&"ROOT");
        }

        if self.0 & Slot::MAP != 0 {
            list.entry(&"MAP");
        }

        if self.0 & Slot::SEQ != 0 {
            list.entry(&"SEQ");
        }

        if self.0 & Slot::KEY != 0 {
            list.entry(&"KEY");
        }

        if self.0 & Slot::VAL != 0 {
            list.entry(&"VAL");
        }

        if self.0 & Slot::ELEM != 0 {
            list.entry(&"ELEM");
        }

        if self.0 & Slot::DONE != 0 {
            list.entry(&"DONE");
        }

        list.finish()
    }
}

#[cfg(test)]
mod benches;