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
The depth of a position.

All positions within a map or sequence are guaranteed
to have the same depth or greater.
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

Implementations of the [`Stream`](../trait.Stream.html) trait are encouraged to use a
stack for validating their input.

The stack is stateful, and keeps track of open maps and sequences.
It serves as validation for operations performed on the stream and
as a way for a flat, stateless stream to know what it's currently
looking at. The stack enforces:

- Map keys and values aren't received outside of a map.
- Map keys are received before map values, and every key has a corresponding value.
- Sequence elements aren't received outside of a sequence.
- Every map and sequence is ended, and in the right order.
- Every map key, map value, and sequence element is followed by valid data.
*/
#[derive(Clone)]
pub struct Stack {
    inner: inner::Stack,
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
            inner: inner::Stack::new(),
        }
    }

    /**
    Clear the stack so that it can be re-used.

    Any state it currently contains will be lost.
    */
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
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

        if self.inner.depth() == 0 {
            // Clear the `DONE` bit so the stack
            // can be re-used
            self.inner.current_mut().0 = Slot::ROOT;

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
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - not be done

        match curr.0 & Slot::DONE {
            Slot::EMPTY => {
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(Error::msg("invalid attempt to write primitive")),
        }
    }

    /**
    Begin a new map.
    */
    #[inline]
    pub fn map_begin(&mut self) -> Result<Pos, Error> {
        let curr = self.inner.current();

        // The current slot must:
        // - not be done and
        // - be the root or
        // - be a map key or
        // - be a map value or
        // - be a seq element

        match curr.0 {
            Slot::ROOT | Slot::MAP_KEY | Slot::MAP_VAL | Slot::SEQ_ELEM => {
                self.inner.push_depth()?;
                self.inner.current_mut().0 = Slot::MAP;

                Ok(curr.pos(self.inner.depth()))
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
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - be a fresh map (with no key or value) or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                curr.0 = Slot::MAP_KEY;

                Ok(curr.pos(self.inner.depth()))
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
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - be a map with a done key

        match curr.0 {
            Slot::MAP_KEY_DONE => {
                curr.0 = Slot::MAP_VAL;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(Error::msg("invalid attempt to begin value")),
        }
    }

    /**
    Complete the current map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<Pos, Error> {
        let curr = self.inner.current();

        // The current slot must:
        // - be a fresh map or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP | Slot::MAP_VAL_DONE => {
                // The fact that the slot is not `Slot::ROOT`
                // guarantees that `depth > 0` and so this
                // will not overflow
                unsafe { self.inner.pop_depth(); }

                let mut curr = self.inner.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.inner.depth() + 1))
            }
            _ => Err(Error::msg("invalid attempt to end map")),
        }
    }

    /**
    Begin a new sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self) -> Result<Pos, Error> {
        let curr = self.inner.current();

        // The current slot must:
        // - not be done and
        // - be the root or
        // - be a map key or
        // - be a map value or
        // - be a seq element

        match curr.0 {
            Slot::ROOT | Slot::MAP_KEY | Slot::MAP_VAL | Slot::SEQ_ELEM => {
                self.inner.push_depth()?;
                self.inner.current_mut().0 = Slot::SEQ;

                Ok(curr.pos(self.inner.depth()))
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
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - be a fresh sequence (with no element) or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ | Slot::SEQ_ELEM_DONE => {
                curr.0 = Slot::SEQ_ELEM;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(Error::msg("invalid attempt to begin element")),
        }
    }

    /**
    Complete the current sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> Result<Pos, Error> {
        let curr = self.inner.current();

        // The current slot must:
        // - be a fresh sequence or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ | Slot::SEQ_ELEM_DONE => {
                // The fact that the slot is not `Slot::ROOT`
                // guarantees that `depth > 0` and so this
                // will not overflow
                unsafe { self.inner.pop_depth(); }

                let mut curr = self.inner.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.inner.depth() + 1))
            }
            _ => Err(Error::msg("invalid attempt to end sequence")),
        }
    }

    /**
    Whether or not the stack has seen a complete and valid stream. 
    */
    #[inline]
    pub fn can_end(&self) -> bool {
        self.inner.depth() == 0
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

        if self.inner.depth() == 0 {
            // Set the slot to done so it
            // can't be re-used without calling begin
            self.inner.current_mut().0 |= Slot::DONE;

            Ok(())
        } else {
            Err(Error::msg("stack is not empty"))
        }
    }
}

#[cfg(not(feature = "arbitrary-depth"))]
mod inner {
    use super::{Slot, Error};

    #[derive(Clone)]
    pub(super) struct Stack {
        slots: [Slot; Stack::SLOTS],
        depth: usize,
    }

    impl Stack {
        const SLOTS: usize = 16;
        const MAX_DEPTH: usize = Self::SLOTS - 1;

        #[inline]
        pub(super) fn new() -> Self {
            Stack {
                slots: [Slot::root(); Self::SLOTS],
                depth: 0,
            }
        }

        #[inline]
        pub(super) fn clear(&mut self) {
            *self = Stack {
                slots: [Slot::root(); Self::SLOTS],
                depth: 0,
            }
        }

        #[inline]
        pub(super) fn depth(&self) -> usize {
            self.depth
        }

        #[inline]
        pub(super) fn push_depth(&mut self) -> Result<(), Error> {
            if self.depth >= Self::MAX_DEPTH {
                return Err(Error::msg("nesting limit reached"));
            }

            self.depth += 1;

            Ok(())
        }

        // Callers must ensure `self.depth() > 0`
        #[inline]
        pub(super) unsafe fn pop_depth(&mut self) {
            self.depth -= 1;
        }

        #[inline]
        pub(super) fn current_mut(&mut self) -> &mut Slot {
            // The depth is guaranteed to be in-bounds
            // and pointing to initialized memory
            unsafe { self.slots.get_unchecked_mut(self.depth) }
        }

        #[inline]
        pub(super) fn current(&self) -> Slot {
            // The depth is guaranteed to be in-bounds
            // and pointing to initialized memory
            unsafe { *self.slots.get_unchecked(self.depth) }
        }
    }
}

#[cfg(feature = "arbitrary-depth")]
mod inner {
    use smallvec::SmallVec;

    use super::{Slot, Error};

    #[derive(Clone)]
    pub(super) struct Stack(SmallVec<[Slot; 16]>);

    impl Stack {
        #[inline]
        pub(super) fn new() -> Self {
            let mut slots = SmallVec::new();
            slots.push(Slot::root());

            Stack(slots)
        }

        #[inline]
        pub(super) fn clear(&mut self) {
            self.0.clear();
            self.0.push(Slot::root());
        }

        #[inline]
        pub(super) fn depth(&self) -> usize {
            self.0.len() - 1
        }

        #[inline]
        pub(super) fn push_depth(&mut self) -> Result<(), Error> {
            self.0.push(Slot::root());

            Ok(())
        }

        #[inline]
        pub(super) unsafe fn pop_depth(&mut self) {
            self.0.pop();
        }

        #[inline]
        pub(super) fn current_mut(&mut self) -> &mut Slot {
            self.0.last_mut().expect("missing stack slot")
        }

        #[inline]
        pub(super) fn current(&self) -> Slot {
            *self.0.last().expect("missing stack slot")
        }
    }
}
