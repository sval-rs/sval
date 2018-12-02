/*!
A fixed-size, stateful stack for streams.
*/

use super::Error;

/**
The expected position in the stream.
*/
#[derive(Clone)]
pub struct Pos {
    slot: u8,
    depth: usize,
}

/**
The depth of the position.
*/
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth(usize);

impl Pos {
    /**
    Whether the current position is a map key.
    */
    #[inline]
    pub fn is_key(&self) -> bool {
        self.slot == Slot::KEY
    }

    /**
    Whether the current position is a map value.
    */
    #[inline]
    pub fn is_value(&self) -> bool {
        self.slot == Slot::VAL
    }

    /**
    Whether the current position is a sequence element.
    */
    #[inline]
    pub fn is_elem(&self) -> bool {
        self.slot == Slot::ELEM
    }

    /**
    The depth of this position.
    */
    #[inline]
    pub fn depth(&self) -> Depth {
        Depth(self.depth)
    }
}

/**
A container for the stream state.

Implementations of the [`Stream`](trait.Stream.html) trait are encouraged to use a
stack for validating their input.

The stack is stateful, and keeps track of open maps and sequences.
It serves as validation for operations performed on the stream and
as a way for a flat, stateless stream to know what it's currently
looking at.
*/
#[derive(Clone)]
pub struct Stack {
    inner: [Slot; Stack::SIZE],
    depth: usize,
}

impl Stack {
    const SIZE: usize = 16;
    const MAX_LEN: usize = Self::SIZE - 1;
}

#[derive(Clone, Copy)]
struct Slot(u8);

impl Slot {
    const EMPTY: u8 = 0b0000_0000;

    const DONE: u8 = 0b0000_0001;

    const ROOT: u8 = 0b1000_0000;
    const MAP: u8 = 0b0100_0000;
    const SEQ: u8 = 0b0010_0000;

    const KEY: u8 = 0b0001_0000;
    const VAL: u8 = 0b0000_1000;
    const ELEM: u8 = 0b0000_0100;

    const MASK_EXPECT: u8 = 0b1001_1100;

    const MAP_KEY: u8 = Self::MAP | Self::KEY;
    const MAP_KEY_DONE: u8 = Self::MAP_KEY | Self::DONE;

    const MAP_VAL: u8 = Self::MAP | Self::VAL;
    const MAP_VAL_DONE: u8 = Self::MAP_VAL | Self::DONE;

    const SEQ_ELEM: u8 = Self::SEQ | Self::ELEM;
    const SEQ_ELEM_DONE: u8 = Self::SEQ_ELEM | Self::DONE;

    #[inline]
    fn root() -> Self {
        Slot(Slot::ROOT)
    }

    #[inline]
    fn pos(self, depth: usize) -> Pos {
        Pos {
            slot: self.0 & Slot::MASK_EXPECT,
            depth,
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    /**
    Create a new stack.
    */
    #[inline]
    pub fn new() -> Self {
        Stack {
            inner: [Slot::root(); Self::SIZE],
            depth: 0,
        }
    }

    /**
    Clear the stack so that it can be re-used.

    Any state it currently contains will be lost.
    */
    #[inline]
    pub fn clear(&mut self) {
        *self = Stack {
            inner: [Slot::root(); Self::SIZE],
            depth: 0,
        };
    }

    #[inline]
    fn current_mut(&mut self) -> &mut Slot {
        unsafe { self.inner.get_unchecked_mut(self.depth) }
    }

    #[inline]
    fn current(&self) -> Slot {
        unsafe { *self.inner.get_unchecked(self.depth) }
    }

    /**
    Ensure the stack is ready for a new value.

    This method only needs to be called by [`Stream`]s that
    can be re-used.
    */
    #[inline]
    pub fn begin(&mut self) -> Result<(), Error> {
        // The stack must be on the root slot
        // It doesn't matter if the slot is
        // marked as done or not

        if self.depth == 0 {
            // Clear the `DONE` bit so the stack
            // can be re-used
            self.current_mut().0 = Slot::ROOT;

            Ok(())
        } else {
            Err(Error::msg("stack is not empty"))
        }
    }

    /**
    Push a primitive.

    A primitive is a simple value that isn't a map or sequence.
    That includes:

    - [`Arguments`](struct.Arguments.html)
    - `u64`, `i64`, `u128`, `i128`
    - `f64`
    - `bool`
    - `char`, `&str`
    - `Option<T>`
    */
    #[inline]
    pub fn primitive(&mut self) -> Result<Pos, Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - not be done

        match curr.0 & Slot::DONE {
            Slot::EMPTY => {
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.depth))
            }
            _ => Err(Error::msg("invalid attempt to write primitive")),
        }
    }

    /**
    Begin a new map.
    */
    #[inline]
    pub fn map_begin(&mut self) -> Result<Pos, Error> {
        if self.depth >= Self::MAX_LEN {
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
                self.depth += 1;
                self.current_mut().0 = Slot::MAP;

                Ok(curr.pos(self.depth))
            }
            _ => Err(Error::msg("invalid attempt to begin map")),
        }
    }

    /**
    Begin a map key.

    The key will be implicitly completed by the value
    that follows it.
    */
    #[inline]
    pub fn map_key(&mut self) -> Result<Pos, Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - be a fresh map (with no key or value) or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                curr.0 = Slot::MAP_KEY;

                Ok(curr.pos(self.depth))
            }
            _ => Err(Error::msg("invalid attempt to begin key")),
        }
    }

    /**
    Begin a map value.

    The value will be implicitly completed by the value
    that follows it.
    */
    #[inline]
    pub fn map_value(&mut self) -> Result<Pos, Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - be a map with a done key

        match curr.0 {
            Slot::MAP_KEY_DONE => {
                curr.0 = Slot::MAP_VAL;

                Ok(curr.pos(self.depth))
            }
            _ => Err(Error::msg("invalid attempt to begin value")),
        }
    }

    /**
    Complete the current map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<Pos, Error> {
        let curr = self.current();

        // The current slot must:
        // - be a fresh map or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                self.depth -= 1;

                let mut curr = self.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.depth + 1))
            }
            _ => Err(Error::msg("invalid attempt to end map")),
        }
    }

    /**
    Begin a new sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self) -> Result<Pos, Error> {
        if self.depth >= Self::MAX_LEN {
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
                self.depth += 1;
                self.current_mut().0 = Slot::SEQ;

                Ok(curr.pos(self.depth))
            }
            _ => Err(Error::msg("invalid attempt to begin sequence")),
        }
    }

    /**
    Begin a sequence element.

    The element will be implicitly completed by the value
    that follows it.
    */
    #[inline]
    pub fn seq_elem(&mut self) -> Result<Pos, Error> {
        let mut curr = self.current_mut();

        // The current slot must:
        // - be a fresh sequence (with no element) or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ | Slot::SEQ_ELEM_DONE => {
                curr.0 = Slot::SEQ_ELEM;

                Ok(curr.pos(self.depth))
            }
            _ => Err(Error::msg("invalid attempt to begin element")),
        }
    }

    /**
    Complete the current sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> Result<Pos, Error> {
        let curr = self.current();

        // The current slot must:
        // - be a fresh sequence or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ | Slot::SEQ_ELEM_DONE => {
                self.depth -= 1;

                let mut curr = self.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.depth + 1))
            }
            _ => Err(Error::msg("invalid attempt to end sequence")),
        }
    }

    /**
    Whether or not the stack has seen a valid stream. 
    */
    #[inline]
    pub fn can_end(&self) -> bool {
        self.depth == 0
    }

    /**
    Complete the stack.

    This stack may be re-used after being completed
    by calling `begin`.
    */
    #[inline]
    pub fn end(&mut self) -> Result<(), Error> {
        // The stack must be on the root slot
        // It doesn't matter if the slot is
        // marked as done or not

        if self.depth == 0 {
            // Set the slot to done so it
            // can't be re-used without calling begin
            self.current_mut().0 |= Slot::DONE;

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
            stream::stack::*,
        };

        #[test]
        fn stack_is_not_bigger_than_vec() {
            assert!(mem::size_of::<Stack>() <= mem::size_of::<Vec<Slot>>());
        }
    }
}
