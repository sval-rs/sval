#[doc(inline)]
pub use crate::Error;

use crate::std::fmt;

pub use self::fmt::Arguments;

/**
A value stream.

The `Stream` trait has a flat structure, but it may need to work with
nested values. The caller is expected to provide a [`Pos`] that tells
the stream how to interpret the values it's being given.
*/
pub trait Stream {
    /** Begin the stream. */
    fn begin(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /** Stream a format. */
    fn fmt(&mut self, args: Arguments) -> Result<(), Error>;

    /** Stream a signed integer. */
    fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /** Stream an unsigned integer. */
    fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /** Stream a 128bit signed integer. */
    fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /** Stream a 128bit unsigned integer. */
    fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /** Stream a floating point value. */
    fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /** Stream a boolean. */
    fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /** Stream a unicode character. */
    fn char(&mut self, v: char) -> Result<(), Error> {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }

    /** Stream a UTF-8 string slice. */
    fn str(&mut self, v: &str) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /** Stream an empty value. */
    fn none(&mut self) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", ()))
    }

    /** Begin a map. */
    fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error>;

    /** Begin a map key. */
    fn map_key(&mut self) -> Result<(), Error>;

    /** Begin a map value. */
    fn map_value(&mut self) -> Result<(), Error>;

    /** End a map. */
    fn map_end(&mut self) -> Result<(), Error>;

    /** Begin a sequence. */
    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error>;

    /** Begin a sequence element. */
    fn seq_elem(&mut self) -> Result<(), Error>;

    /** End a sequence. */
    fn seq_end(&mut self) -> Result<(), Error>;

    /** End the stream. */
    fn end(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, T: ?Sized> Stream for &'a mut T
where
    T: Stream,
{
    fn begin(&mut self) -> Result<(), Error> {
        (**self).begin()
    }

    fn fmt(&mut self, args: Arguments) -> Result<(), Error> {
        (**self).fmt(args)
    }

    fn i64(&mut self, v: i64) -> Result<(), Error> {
        (**self).i64(v)
    }

    fn u64(&mut self, v: u64) -> Result<(), Error> {
        (**self).u64(v)
    }

    fn i128(&mut self, v: i128) -> Result<(), Error> {
        (**self).i128(v)
    }

    fn u128(&mut self, v: u128) -> Result<(), Error> {
        (**self).u128(v)
    }

    fn f64(&mut self, v: f64) -> Result<(), Error> {
        (**self).f64(v)
    }

    fn bool(&mut self, v: bool) -> Result<(), Error> {
        (**self).bool(v)
    }

    fn char(&mut self, v: char) -> Result<(), Error> {
        (**self).char(v)
    }

    fn str(&mut self, v: &str) -> Result<(), Error> {
        (**self).str(v)
    }

    fn none(&mut self) -> Result<(), Error> {
        (**self).none()
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        (**self).map_begin(len)
    }

    fn map_key(&mut self) -> Result<(), Error> {
        (**self).map_key()
    }

    fn map_value(&mut self) -> Result<(), Error> {
        (**self).map_value()
    }

    fn map_end(&mut self) -> Result<(), Error> {
        (**self).map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        (**self).seq_begin(len)
    }

    fn seq_elem(&mut self) -> Result<(), Error> {
        (**self).seq_elem()
    }

    fn seq_end(&mut self) -> Result<(), Error> {
        (**self).seq_end()
    }

    fn end(&mut self) -> Result<(), Error> {
        (**self).end()
    }
}

/**
The expected position in the stream.
*/
#[derive(Clone, Copy)]
pub enum Pos {
    /** The root of the stream. */
    Root,
    /** A key within a map. */
    Key,
    /** A value within a map. */
    Value,
    /** An element within a sequence. */
    Elem,
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
pub struct Stack {
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
    fn pos(&self) -> Result<Pos, Error> {
        match self.0 & Slot::MASK_EXPECT {
            Slot::ROOT => Ok(Pos::Root),
            Slot::KEY => Ok(Pos::Key),
            Slot::VAL => Ok(Pos::Value),
            Slot::ELEM => Ok(Pos::Elem),
            _ => Err(Error::msg("invalid position")),
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
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
    pub fn primitive(&mut self) -> Result<Pos, Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - not be done

        match curr.0 & Slot::DONE {
            Slot::EMPTY => {
                curr.0 |= Slot::DONE;

                curr.pos()
            }
            _ => Err(Error::msg("invalid attempt to write primitive")),
        }
    }

    #[inline]
    pub fn map_begin(&mut self) -> Result<Pos, Error> {
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

                curr.pos()
            }
            _ => Err(Error::msg("invalid attempt to begin map")),
        }
    }

    #[inline]
    pub fn map_key(&mut self) -> Result<(), Error> {
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
    pub fn map_value(&mut self) -> Result<(), Error> {
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
    pub fn map_end(&mut self) -> Result<Pos, Error> {
        let curr = self.current();

        // The current slot must:
        // - be a fresh map or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                self.len -= 1;

                let mut curr = self.current_mut();
                curr.0 |= Slot::DONE;

                curr.pos()
            }
            _ => Err(Error::msg("invalid attempt to end map")),
        }
    }

    #[inline]
    pub fn seq_begin(&mut self) -> Result<Pos, Error> {
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

                curr.pos()
            }
            _ => Err(Error::msg("invalid attempt to begin sequence")),
        }
    }

    #[inline]
    pub fn seq_elem(&mut self) -> Result<(), Error> {
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
    pub fn seq_end(&mut self) -> Result<Pos, Error> {
        let curr = self.current();

        // The current slot must:
        // - be a fresh sequence or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ | Slot::SEQ_ELEM_DONE => {
                self.len -= 1;

                let mut curr = self.current_mut();
                curr.0 |= Slot::DONE;

                curr.pos()
            }
            _ => Err(Error::msg("invalid attempt to end sequence")),
        }
    }

    #[inline]
    pub fn end(&mut self) -> Result<(), Error> {
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
mod tests {
    #[cfg(feature = "std")]
    mod std_support {
        use crate::{
            std::{
                mem,
                vec::Vec,
            },
            stream::*,
        };

        #[test]
        fn stack_is_not_bigger_than_vec() {
            assert!(mem::size_of::<Stack>() <= mem::size_of::<Vec<Slot>>());
        }
    }
}
