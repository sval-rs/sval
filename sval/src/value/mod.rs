#[doc(inline)]
pub use crate::Error;

use crate::{
    std::fmt,
    stream,
};

/**
A value with a streamable structure.
*/
pub trait Value {
    /** Stream this value. */
    fn stream(&self, stream: &mut Stream) -> Result<(), Error>;
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        (**self).stream(stream)
    }
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
    pub(crate) fn begin(stream: &'a mut dyn stream::Stream) -> Result<Self, Error> {
        stream.begin()?;

        Ok(Stream {
            stack: Stack::default(),
            stream,
        })
    }

    /**
    Stream a value.
    */
    #[inline]
    pub fn any(&mut self, v: impl Value) -> Result<(), Error> {
        v.stream(self)
    }

    /**
    Stream format arguments.
    */
    #[inline]
    pub fn fmt(&mut self, f: fmt::Arguments) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.fmt(pos, f)?;

        Ok(())
    }

    /**
    Stream a signed integer.
    */
    #[inline]
    pub fn i64(&mut self, v: i64) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.i64(pos, v)?;

        Ok(())
    }

    /**
    Stream an unsigned integer.
    */
    #[inline]
    pub fn u64(&mut self, v: u64) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.u64(pos, v)?;

        Ok(())
    }

    /**
    Stream a 128-bit signed integer.
    */
    #[inline]
    pub fn i128(&mut self, v: i128) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.i128(pos, v)?;

        Ok(())
    }

    /**
    Stream a 128-bit unsigned integer.
    */
    #[inline]
    pub fn u128(&mut self, v: u128) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.u128(pos, v)?;

        Ok(())
    }

    /**
    Stream a floating point value.
    */
    #[inline]
    pub fn f64(&mut self, v: f64) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.f64(pos, v)?;

        Ok(())
    }

    /**
    Stream a boolean.
    */
    #[inline]
    pub fn bool(&mut self, v: bool) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.bool(pos, v)?;

        Ok(())
    }

    /**
    Stream a unicode character.
    */
    #[inline]
    pub fn char(&mut self, v: char) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.char(pos, v)?;

        Ok(())
    }

    /**
    Stream a UTF8 string.
    */
    #[inline]
    pub fn str(&mut self, v: &str) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.str(pos, v)?;

        Ok(())
    }

    /**
    Stream an empty value.
    */
    #[inline]
    pub fn none(&mut self) -> Result<(), Error> {
        let pos = self.stack.primitive()?;
        self.stream.none(pos)?;

        Ok(())
    }

    /**
    Begin a map.
    */
    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> Result<&mut Stream<'a>, Error> {
        let pos = self.stack.map_begin()?;
        self.stream.map_begin(pos, len)?;

        Ok(self)
    }

    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.map_key()?;

        Ok(self)
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.map_value()?;

        Ok(self)
    }

    /**
    End a map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<(), Error> {
        let pos = self.stack.map_end()?;
        self.stream.map_end(pos)?;

        Ok(())
    }

    /**
    Begin a sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> Result<&mut Stream<'a>, Error> {
        let pos = self.stack.seq_begin()?;
        self.stream.seq_begin(pos, len)?;

        Ok(self)
    }

    /**
    Begin a sequence element.
    */
    #[inline]
    pub fn seq_elem(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.seq_elem()?;

        Ok(self)
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> Result<(), Error> {
        let pos = self.stack.seq_end()?;
        self.stream.seq_end(pos)?;

        Ok(())
    }

    /**
    End the stream.
    */
    #[inline]
    pub(crate) fn end(mut self) -> Result<(), Error> {
        self.stack.end()?;
        self.stream.end()
    }
}

/**
A container for the stream state.

The stack is stateful, and keeps track of open maps and sequences.
It serves as validation for operations performed on the stream and
as a way for a flat, stateless stream to know what it's currently
looking at.

It puts an arbitrary limit on the map and sequence depth so that
it doesn't need an allocator to work. For individual values in
a structured log this limit is ok, but might be a problem for a
truly general-purpose serialization framework.

The stack is designed to be no larger than a standard `Vec`.
The state of each slot encoded in a single byte.
*/
#[derive(Clone, Copy)]
struct Stack {
    inner: [Slot; Stack::SIZE],
    len: usize,
}

impl Stack {
    const SIZE: usize = 16;
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
    const SEQ_ELEM_DONE: u8 = Self::SEQ_ELEM | Self::DONE;

    fn root() -> Self {
        Slot(Slot::ROOT)
    }

    #[inline]
    fn pos(&self) -> stream::Pos {
        match self.0 & Slot::MASK_EXPECT {
            Slot::ROOT => stream::Pos::Root,
            Slot::KEY => stream::Pos::Key,
            Slot::VAL => stream::Pos::Value,
            Slot::ELEM => stream::Pos::Elem,
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
    fn primitive(&mut self) -> Result<stream::Pos, Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - not be done

        match curr.0 & Slot::DONE {
            Slot::EMPTY => {
                curr.0 |= Slot::DONE;

                Ok(curr.pos())
            }
            _ => Err(Error::msg("invalid attempt to write primitive")),
        }
    }

    #[inline]
    fn map_begin(&mut self) -> Result<stream::Pos, Error> {
        if self.len >= Self::MAX_LEN {
            return Err(Error::msg("nesting limit reached"));
        }

        let curr = self.current();

        // The current slot must:
        // - not be done and
        // - be the root or
        // - be a map key or
        // - be a map value or
        // - be a seq element

        match curr.0 {
            Slot::ROOT | Slot::MAP_KEY | Slot::MAP_VAL | Slot::SEQ_ELEM => {
                self.len += 1;
                self.current_mut().0 = Slot::MAP;

                Ok(curr.pos())
            }
            _ => Err(Error::msg("invalid attempt to begin map")),
        }
    }

    #[inline]
    fn map_key(&mut self) -> Result<(), Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - be a fresh map (with no key or value) or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                curr.0 = Slot::MAP_KEY;

                Ok(())
            }
            _ => Err(Error::msg("invalid attempt to begin key")),
        }
    }

    #[inline]
    fn map_value(&mut self) -> Result<(), Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - be a map with a done key

        match curr.0 {
            Slot::MAP_KEY_DONE => {
                curr.0 = Slot::MAP_VAL;

                Ok(())
            }
            _ => Err(Error::msg("invalid attempt to begin value")),
        }
    }

    #[inline]
    fn map_end(&mut self) -> Result<stream::Pos, Error> {
        let curr = self.current();

        // The current slot must:
        // - be a fresh map or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                self.len -= 1;

                let mut curr = self.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos())
            }
            _ => Err(Error::msg("invalid attempt to end map")),
        }
    }

    #[inline]
    fn seq_begin(&mut self) -> Result<stream::Pos, Error> {
        if self.len >= Self::MAX_LEN {
            return Err(Error::msg("nesting limit reached"));
        }

        let curr = self.current();

        // The current slot must:
        // - not be done and
        // - be the root or
        // - be a map key or
        // - be a map value or
        // - be a seq element

        match curr.0 {
            Slot::ROOT | Slot::MAP_KEY | Slot::MAP_VAL | Slot::SEQ_ELEM => {
                self.len += 1;
                self.current_mut().0 = Slot::SEQ;

                Ok(curr.pos())
            }
            _ => Err(Error::msg("invalid attempt to begin sequence")),
        }
    }

    #[inline]
    fn seq_elem(&mut self) -> Result<(), Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - be a fresh sequence (with no element) or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ | Slot::SEQ_ELEM_DONE => {
                curr.0 = Slot::SEQ_ELEM;

                Ok(())
            }
            _ => Err(Error::msg("invalid attempt to begin element")),
        }
    }

    #[inline]
    fn seq_end(&mut self) -> Result<stream::Pos, Error> {
        let curr = self.current();

        // The current slot must:
        // - be a fresh sequence or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ | Slot::SEQ_ELEM_DONE => {
                self.len -= 1;

                let mut curr = self.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos())
            }
            _ => Err(Error::msg("invalid attempt to end sequence")),
        }
    }

    #[inline]
    fn end(&mut self) -> Result<(), Error> {
        // The stack must be on the root slot
        // It doesn't matter if the slot is
        // marked as done or not

        if self.len == 0 {
            Ok(())
        } else {
            Err(Error::msg("stack is not empty"))
        }
    }
}

#[cfg(test)]
mod benches;

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    mod std_support {
        use super::*;
        use crate::std::{mem, vec::Vec};

        #[test]
        fn stack_is_not_bigger_than_vec() {
            assert!(mem::size_of::<Stack>() <= mem::size_of::<Vec<Slot>>());
        }
    }
}
