use crate::{
    std::fmt,
    stream::{
        self,
        Stream,
    },
};

struct Expecting {
    expecting: &'static str,
}

impl fmt::Display for Expecting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expecting {}", self.expecting)
    }
}

/**
The expected position in the stream.
*/
#[derive(Clone)]
pub struct Pos {
    // TODO: Return a `u64` of the whole stack here to mask the depth off
    slot: u8,
    depth: u8,
}

/**
The depth of a position.

All positions within a map or sequence are guaranteed
to have the same depth or greater.
*/
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth(usize);

#[cfg(all(feature = "alloc", any(test, feature = "test")))]
impl Depth {
    pub(crate) fn root() -> Self {
        Depth(0)
    }
}

impl Pos {
    /**
    Whether the current position is a map key.
    */
    #[inline]
    pub fn is_key(&self) -> bool {
        self.slot & Slot::MASK_SLOT == Slot::IS_MAP_KEY
    }

    /**
    Whether the current position is a map value.
    */
    #[inline]
    pub fn is_value(&self) -> bool {
        self.slot & Slot::MASK_SLOT == Slot::IS_MAP_VALUE
    }

    /**
    Whether the current position is a sequence element.
    */
    #[inline]
    pub fn is_elem(&self) -> bool {
        self.slot & Slot::MASK_SLOT == Slot::IS_SEQ_ELEM
    }

    /**
    Whether the current position is a map value or sequence element.
    */
    #[inline]
    pub fn is_value_elem(&self) -> bool {
        self.slot & Slot::MASK_VALUE_ELEM != 0
    }

    /**
    Whether the current position is an empty map.
    */
    #[inline]
    pub fn is_empty_map(&self) -> bool {
        unimplemented!()
    }

    /**
    Whether the current position is an empty sequence.
    */
    #[inline]
    pub fn is_empty_seq(&self) -> bool {
        unimplemented!()
    }

    /**
    The depth of this position.
    */
    #[inline]
    pub fn depth(&self) -> Depth {
        Depth(self.depth as usize)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Slot(u8);

impl Slot {
    const IS_EMPTY: u8 =        0b0000_0000;
    const IS_MAP_KEY: u8 =      0b0000_0010;
    const IS_MAP_VALUE: u8 =    0b0000_0110;
    const IS_SEQ_ELEM: u8 =     0b0000_1000;
    const RESERVED: u8 =        0b0001_0000;

    const MASK_VALUE_ELEM: u8 = 0b0000_1100;

    const NEEDS_ITEM: u8 =      0b0000_0001;
    const NEEDS_MAP_KEY: u8 =   0b0000_0100;
    const NEEDS_MAP_VALUE: u8 = 0b0000_0010;
    const NEEDS_SEQ_ELEM: u8 =  0b0000_1000;

    const MASK_SLOT: u8 = u8::MAX >> (u8::BITS as u8 - Slot::SIZE as u8);

    // NOTE: This leaves us with 4 "spare" bits at the end of a 64bit stack
    // This is where we could encode whether or not the map or sequence is empty
    const SIZE: usize = 5;

    #[inline]
    fn pos(self, depth: u8) -> Pos {
        Pos {
            slot: self.0,
            depth,
        }
    }
}

impl fmt::Debug for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>8b}", self.0)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

type RawStack = u64;

#[derive(Clone)]
pub struct Stack {
    inner: RawStack,
    depth: u8,
}

impl Stack {
    const MAX_DEPTH: u8 = (RawStack::BITS as u8 / Slot::SIZE as u8);

    const MASK_SLOT_BEGIN: RawStack = (RawStack::MAX << Slot::SIZE) ^ (Slot::NEEDS_ITEM as RawStack) << Slot::SIZE;

    /**
    Create a new stack.
    */
    #[inline]
    pub fn new() -> Self {
        Stack {
            inner: Slot::NEEDS_ITEM as RawStack,
            depth: 0,
        }
    }

    /**
    Clear the stack so that it can be re-used.

    Any state it currently contains will be lost.
    */
    #[inline]
    pub fn clear(&mut self) {
        unimplemented!()
    }

    /**
    Get the current position in the stack.
    */
    #[inline]
    pub fn current(&self) -> Pos {
        unimplemented!()
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
    - `Option<T>`.
    */
    #[inline]
    pub fn primitive(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT & Slot::NEEDS_ITEM;
        const VALID: u8 = Slot::NEEDS_ITEM;
        const EXPECT_NEXT: RawStack = Slot::NEEDS_ITEM as RawStack;

        if self.inner as u8 & MASK == VALID {
            self.inner ^= EXPECT_NEXT;

            Ok(Slot(self.inner as u8).pos(self.depth))
        } else {
            err_invalid("a primitive")
        }
    }

    /**
    Begin a new map.

    The map must be completed by calling `map_end`.
    */
    #[inline]
    pub fn map_begin(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT & Slot::NEEDS_ITEM;
        const VALID: u8 = Slot::NEEDS_ITEM;
        const EXPECT: RawStack = (Slot::NEEDS_MAP_KEY | Slot::NEEDS_MAP_VALUE) as RawStack;

        if self.depth == Self::MAX_DEPTH {
            return err_invalid("more depth at the start of a map");
        }

        if self.inner as u8 & MASK == VALID {
            self.inner = (self.inner << Slot::SIZE) & Self::MASK_SLOT_BEGIN | EXPECT;
            self.depth += 1;

            Ok(Slot(self.inner as u8).pos(self.depth))
        } else {
            err_invalid("the start of a map")
        }
    }

    /**
    Begin a map key.

    The key will be implicitly completed by the value
    that follows it.
    */
    #[inline]
    pub fn map_key(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT;
        const VALID: u8 = Slot::NEEDS_MAP_KEY | Slot::NEEDS_MAP_VALUE;
        const EXPECT: RawStack = (Slot::NEEDS_MAP_KEY | Slot::NEEDS_ITEM) as RawStack;

        if self.inner as u8 & MASK == VALID {
            self.inner ^= EXPECT;

            Ok(Slot(self.inner as u8).pos(self.depth))
        } else {
            err_invalid("a map key")
        }
    }

    /**
    Begin a map value.

    The value will be implicitly completed by the value
    that follows it.
    */
    #[inline]
    pub fn map_value(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT;
        const VALID: u8 = Slot::NEEDS_MAP_VALUE;
        const EXPECT: RawStack = (Slot::NEEDS_MAP_KEY | Slot::NEEDS_ITEM) as RawStack;

        if self.inner as u8 & MASK == VALID {
            self.inner ^= EXPECT;

            Ok(Slot(self.inner as u8).pos(self.depth))
        } else {
            err_invalid("a map value")
        }
    }

    /**
    Complete the current map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT;
        const VALID: u8 = Slot::NEEDS_MAP_KEY | Slot::NEEDS_MAP_VALUE;

        if self.inner as u8 & MASK == VALID {
            self.inner = self.inner >> Slot::SIZE;
            self.depth -= 1;

            Ok(Slot(self.inner as u8).pos(self.depth + 1))
        } else {
            err_invalid("the end of a map")
        }
    }

    /**
    Begin a new sequence.

    the sequence must be completed by calling `seq_end`.
    */
    #[inline]
    pub fn seq_begin(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT & Slot::NEEDS_ITEM;
        const VALID: u8 = Slot::NEEDS_ITEM;
        const EXPECT: RawStack = (Slot::NEEDS_SEQ_ELEM) as RawStack;

        if self.depth == Self::MAX_DEPTH {
            return err_invalid("more depth at the start of a sequence");
        }

        if self.inner as u8 & MASK == VALID {
            self.inner = (self.inner << Slot::SIZE) & Self::MASK_SLOT_BEGIN | EXPECT;
            self.depth += 1;

            Ok(Slot(self.inner as u8).pos(self.depth))
        } else {
            err_invalid("the start of a sequence")
        }
    }

    /**
    Begin a sequence element.

    The element will be implicitly completed by the value
    that follows it.
    */
    #[inline]
    pub fn seq_elem(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT;
        const VALID: u8 = Slot::NEEDS_SEQ_ELEM;
        const EXPECT: RawStack = Slot::NEEDS_ITEM as RawStack;

        if self.inner as u8 & MASK == VALID {
            self.inner |= EXPECT;

            Ok(Slot(self.inner as u8).pos(self.depth))
        } else {
            err_invalid("a sequence element")
        }
    }

    /**
    Complete the current sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> Result<Pos, crate::Error> {
        const MASK: u8 = Slot::MASK_SLOT;
        const VALID: u8 = Slot::NEEDS_SEQ_ELEM;

        if self.inner as u8 & MASK == VALID {
            self.inner = self.inner >> Slot::SIZE;
            self.depth -= 1;

            Ok(Slot(self.inner as u8).pos(self.depth + 1))
        } else {
            err_invalid("the end of a sequence")
        }
    }

    /**
    Whether or not the stack has seen a complete and valid stream.
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
    pub fn end(&mut self) -> Result<(), crate::Error> {
        // In order to end the stream, the stack must be completed
        const MASK: u8 = !Slot::NEEDS_ITEM;
        const VALID: u8 = Slot::IS_EMPTY;

        if self.depth == 0 && self.inner as u8 & MASK == VALID {
            Ok(())
        } else {
            err_invalid("the end of the stream")
        }
    }
}

#[cold]
fn err_invalid<T>(expecting: &'static str) -> Result<T, crate::Error> {
    Err(crate::Error::custom(&Expecting {
        expecting,
    }))
}
