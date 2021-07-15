/*
/!\ CAREFUL /!\

This module contains unsafe code with some tricky
invariants based on the state of the current slot.

We use a combination of property-based testing
and a reasonable test suite to try ensure safety
is maintained, but any changes here should be
reviewed carefully.
*/

use crate::std::fmt;

#[derive(Clone)]
pub struct Pos {
    slot: u8,
    depth: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth(usize);

impl Depth {
    pub fn root() -> Self {
        Depth(0)
    }
}

impl Pos {
    pub fn is_key(&self) -> bool {
        self.slot & Slot::MASK_POS == Slot::KEY
    }

    pub fn is_value(&self) -> bool {
        self.slot & Slot::MASK_POS == Slot::VAL
    }

    pub fn is_elem(&self) -> bool {
        self.slot & Slot::MASK_POS == Slot::ELEM
    }

    pub fn is_empty_map(&self) -> bool {
        self.slot == Slot::MAP_DONE
    }

    pub fn is_empty_seq(&self) -> bool {
        self.slot == Slot::SEQ_DONE
    }

    pub fn depth(&self) -> Depth {
        Depth(self.depth)
    }
}

struct Expecting {
    got: &'static str,
    expecting: &'static str,
}

impl fmt::Display for Expecting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid {}, expecting {}", self.got, self.expecting)
    }
}

macro_rules! expecting {
    ($slot:expr, $got:expr) => {{
        fn expecting(slot: Slot) -> &'static dyn fmt::Display {
            match slot.0 {
                Slot::ROOT | Slot::SEQ_ELEM | Slot::MAP_KEY | Slot::MAP_VAL => &Expecting {
                    got: $got,
                    expecting: "a primitive, map begin or seq begin",
                },
                Slot::SEQ_DONE | Slot::SEQ_ELEM_DONE => &Expecting {
                    got: $got,
                    expecting: "a seq elem or seq end",
                },
                Slot::MAP_DONE | Slot::MAP_VAL_DONE => &Expecting {
                    got: $got,
                    expecting: "a map key or map done",
                },
                Slot::MAP_KEY_DONE => &Expecting {
                    got: $got,
                    expecting: "a map value",
                },
                slot if slot & Slot::DONE == Slot::DONE => &Expecting {
                    got: $got,
                    expecting: "nothing",
                },
                _ => unreachable!(),
            }
        }

        expecting($slot)
    }};
}

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

    const MASK_POS: u8 = Self::ROOT | Self::KEY | Self::VAL | Self::ELEM;

    const MAP_DONE: u8 = Self::MAP | Self::DONE;

    const MAP_KEY: u8 = Self::MAP | Self::KEY;
    const MAP_KEY_DONE: u8 = Self::MAP_KEY | Self::DONE;

    const MAP_VAL: u8 = Self::MAP | Self::VAL;
    const MAP_VAL_DONE: u8 = Self::MAP_VAL | Self::DONE;

    const SEQ_DONE: u8 = Self::SEQ | Self::DONE;

    const SEQ_ELEM: u8 = Self::SEQ | Self::ELEM;
    const SEQ_ELEM_DONE: u8 = Self::SEQ_ELEM | Self::DONE;

    fn root() -> Self {
        Slot(Slot::ROOT)
    }

    fn pos(self, depth: usize) -> Pos {
        Pos {
            slot: self.0,
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
    pub fn new() -> Self {
        Stack {
            inner: inner::Stack::new(),
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn primitive(&mut self) -> Result<Pos, crate::Error> {
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - not be done

        match curr.0 & Slot::DONE {
            Slot::EMPTY => {
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(crate::Error::custom(expecting!(*curr, "primitive"))),
        }
    }

    pub fn map_begin(&mut self) -> Result<Pos, crate::Error> {
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
                self.inner.current_mut().0 = Slot::MAP_DONE;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(crate::Error::custom(expecting!(curr, "map begin"))),
        }
    }

    pub fn map_key(&mut self) -> Result<Pos, crate::Error> {
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - be a fresh map (with no key or value) or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP_DONE | Slot::MAP_VAL_DONE => {
                curr.0 = Slot::MAP_KEY;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(crate::Error::custom(expecting!(*curr, "map key"))),
        }
    }

    pub fn map_value(&mut self) -> Result<Pos, crate::Error> {
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - be a map with a done key

        match curr.0 {
            Slot::MAP_KEY_DONE => {
                curr.0 = Slot::MAP_VAL;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(crate::Error::custom(expecting!(*curr, "map value"))),
        }
    }

    pub fn map_end(&mut self) -> Result<Pos, crate::Error> {
        let curr = self.inner.current();

        // The current slot must:
        // - be a fresh map or
        // - be a map with a done value

        match curr.0 {
            Slot::MAP_DONE | Slot::MAP_VAL_DONE => {
                // The fact that the slot is not `Slot::ROOT`
                // guarantees that `depth > 0` and so this
                // will not overflow
                unsafe {
                    self.inner.pop_depth();
                }

                let mut curr = self.inner.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.inner.depth() + 1))
            }
            _ => Err(crate::Error::custom(expecting!(curr, "map end"))),
        }
    }

    pub fn seq_begin(&mut self) -> Result<Pos, crate::Error> {
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
                self.inner.current_mut().0 = Slot::SEQ_DONE;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(crate::Error::custom(expecting!(curr, "seq begin"))),
        }
    }

    pub fn seq_elem(&mut self) -> Result<Pos, crate::Error> {
        let mut curr = self.inner.current_mut();

        // The current slot must:
        // - be a fresh sequence (with no element) or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ_DONE | Slot::SEQ_ELEM_DONE => {
                curr.0 = Slot::SEQ_ELEM;

                Ok(curr.pos(self.inner.depth()))
            }
            _ => Err(crate::Error::custom(expecting!(*curr, "seq elem"))),
        }
    }

    pub fn seq_end(&mut self) -> Result<Pos, crate::Error> {
        let curr = self.inner.current();

        // The current slot must:
        // - be a fresh sequence or
        // - be a sequence with a done element

        match curr.0 {
            Slot::SEQ_DONE | Slot::SEQ_ELEM_DONE => {
                // The fact that the slot is not `Slot::ROOT`
                // guarantees that `depth > 0` and so this
                // will not overflow
                unsafe {
                    self.inner.pop_depth();
                }

                let mut curr = self.inner.current_mut();
                curr.0 |= Slot::DONE;

                Ok(curr.pos(self.inner.depth() + 1))
            }
            _ => Err(crate::Error::custom(expecting!(curr, "seq end"))),
        }
    }

    pub fn can_end(&self) -> bool {
        self.inner.depth() == 0
    }

    pub fn end(&mut self) -> Result<(), crate::Error> {
        // The stack must be on the root slot
        // It doesn't matter if the slot is
        // marked as done or not

        if self.inner.depth() == 0 {
            // Set the slot to done so it
            // can't be re-used without calling begin
            self.inner.current_mut().0 |= Slot::DONE;

            Ok(())
        } else {
            Err(crate::Error::custom(&"stack is not empty"))
        }
    }
}

#[cfg(not(feature = "arbitrary-depth"))]
mod inner {
    use super::Slot;

    #[derive(Clone)]
    pub(super) struct Stack {
        slots: [Slot; Stack::SLOTS],
        depth: usize,
    }

    impl Stack {
        const SLOTS: usize = 16;
        const MAX_DEPTH: usize = Self::SLOTS - 1;

        pub(super) fn new() -> Self {
            Stack {
                slots: [Slot::root(); Self::SLOTS],
                depth: 0,
            }
        }

        pub(super) fn clear(&mut self) {
            *self = Stack {
                slots: [Slot::root(); Self::SLOTS],
                depth: 0,
            }
        }

        pub(super) fn depth(&self) -> usize {
            self.depth
        }

        pub(super) fn push_depth(&mut self) -> Result<(), crate::Error> {
            if self.depth >= Self::MAX_DEPTH {
                return Err(crate::Error::custom(&"nesting limit reached"));
            }

            self.depth += 1;

            Ok(())
        }

        // Callers must ensure `self.depth() > 0`

        pub(super) unsafe fn pop_depth(&mut self) {
            self.depth -= 1;
        }

        pub(super) fn current_mut(&mut self) -> &mut Slot {
            #[cfg(debug_assertions)]
            {
                self.slots.get_mut(self.depth).expect("depth out of bounds")
            }
            #[cfg(not(debug_assertions))]
            {
                // The depth is guaranteed to be in-bounds
                // and pointing to initialized memory
                unsafe { self.slots.get_unchecked_mut(self.depth) }
            }
        }

        pub(super) fn current(&self) -> Slot {
            #[cfg(debug_assertions)]
            {
                *self.slots.get(self.depth).expect("depth out of bounds")
            }
            #[cfg(not(debug_assertions))]
            {
                // The depth is guaranteed to be in-bounds
                // and pointing to initialized memory
                unsafe { *self.slots.get_unchecked(self.depth) }
            }
        }
    }
}

#[cfg(feature = "arbitrary-depth")]
mod inner {
    use smallvec::SmallVec;

    use super::Slot;

    #[derive(Clone)]
    pub(super) struct Stack(SmallVec<[Slot; 16]>);

    impl Stack {
        pub(super) fn new() -> Self {
            let mut slots = SmallVec::new();
            slots.push(Slot::root());

            Stack(slots)
        }

        pub(super) fn clear(&mut self) {
            self.0.clear();
            self.0.push(Slot::root());
        }

        pub(super) fn depth(&self) -> usize {
            self.0.len() - 1
        }

        pub(super) fn push_depth(&mut self) -> Result<(), crate::Error> {
            self.0.push(Slot::root());

            Ok(())
        }

        pub(super) unsafe fn pop_depth(&mut self) {
            self.0.pop();
        }

        pub(super) fn current_mut(&mut self) -> &mut Slot {
            self.0.last_mut().expect("missing stack slot")
        }

        pub(super) fn current(&self) -> Slot {
            *self.0.last().expect("missing stack slot")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    mod prop_test {
        use super::*;

        use crate::std::vec::Vec;

        use quickcheck::{
            quickcheck,
            Arbitrary,
            Gen,
        };

        // FIXME: This test isn't very clever about how a
        // sequence of commands is generated. It's more likely
        // to come up with a set that fails early than one
        // that manages to push and pop the stack depth a lot.
        // We could instead weight commands so we're more likely
        // to generate something valid with a degree of randomness.

        #[derive(Clone, Copy, Debug)]
        enum Command {
            Primitive,
            MapBegin,
            MapKey,
            MapValue,
            MapEnd,
            SeqBegin,
            SeqElem,
            SeqEnd,
            End,
        }

        impl Arbitrary for Command {
            fn arbitrary<G: Gen>(g: &mut G) -> Command {
                match g.next_u32() % 9 {
                    0 => Command::Primitive,
                    1 => Command::MapBegin,
                    2 => Command::MapKey,
                    3 => Command::MapValue,
                    4 => Command::MapEnd,
                    5 => Command::SeqBegin,
                    6 => Command::SeqElem,
                    7 => Command::SeqEnd,
                    8 => Command::End,
                    _ => unreachable!(),
                }
            }
        }

        quickcheck! {
            fn stack_does_not_panic(cmd: Vec<Command>) -> bool {
                let mut stack = Stack::new();

                for cmd in cmd {
                    match cmd {
                        Command::Primitive => {
                            let _ = stack.primitive();
                        },
                        Command::MapBegin => {
                            let _ = stack.map_begin();
                        },
                        Command::MapKey => {
                            let _ = stack.map_key();
                        },
                        Command::MapValue => {
                            let _ = stack.map_value();
                        },
                        Command::MapEnd => {
                            let _ = stack.map_end();
                        },
                        Command::SeqBegin => {
                            let _ = stack.seq_begin();
                        },
                        Command::SeqElem => {
                            let _ = stack.seq_elem();
                        },
                        Command::SeqEnd => {
                            let _ = stack.seq_end();
                        },
                        Command::End => {
                            let _ = stack.end();
                        },
                    }
                }

                // So long as the stack doesn't panic we're happy
                true
            }
        }
    }

    #[cfg(feature = "arbitrary-depth")]
    mod arbitrary_depth {
        use super::*;

        #[test]
        fn stack_spills_to_heap_on_overflow() {
            let mut stack = Stack::new();

            for _ in 0..64 {
                stack.map_begin().unwrap();
                stack.map_key().unwrap();
            }
        }
    }

    #[cfg(not(feature = "arbitrary-depth"))]
    mod fixed_depth {
        use super::*;

        #[test]
        fn error_overflow_stack() {
            let mut stack = Stack::new();

            for _ in 0..15 {
                stack.map_begin().unwrap();
                stack.map_key().unwrap();
            }

            // The 16th attempt to begin a map should fail
            assert!(stack.map_begin().is_err());
        }
    }

    #[test]
    fn primitive() {
        let mut stack = Stack::new();

        stack.primitive().unwrap();
        stack.end().unwrap();
    }

    #[test]
    fn end_empty_stack() {
        let mut stack = Stack::new();

        stack.end().unwrap();
    }

    #[test]
    fn error_double_primitive() {
        let mut stack = Stack::new();

        stack.primitive().unwrap();

        assert!(stack.primitive().is_err());
    }

    #[test]
    fn simple_map() {
        let mut stack = Stack::new();

        stack.map_begin().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();

        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_end().unwrap();

        stack.end().unwrap();
    }

    #[test]
    fn empty_map() {
        let mut stack = Stack::new();

        stack.map_begin().unwrap();
        stack.map_end().unwrap();

        stack.end().unwrap();
    }

    #[test]
    fn error_end_map_as_seq() {
        let mut stack = Stack::new();

        stack.map_begin().unwrap();

        assert!(stack.seq_end().is_err());
    }

    #[test]
    fn error_end_seq_as_map() {
        let mut stack = Stack::new();

        stack.seq_begin().unwrap();

        assert!(stack.map_end().is_err());
    }

    #[test]
    fn error_end_map_at_root_depth() {
        let mut stack = Stack::new();

        assert!(stack.map_end().is_err());
    }

    #[test]
    fn error_end_seq_at_root_depth() {
        let mut stack = Stack::new();

        assert!(stack.seq_end().is_err());
    }

    #[test]
    fn error_primitive_without_map_key() {
        let mut stack = Stack::new();

        stack.map_begin().unwrap();

        assert!(stack.primitive().is_err());
    }

    #[test]
    fn error_primitive_without_map_value() {
        let mut stack = Stack::new();

        stack.map_begin().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();

        assert!(stack.primitive().is_err());
    }

    #[test]
    fn error_map_value_without_map_key() {
        let mut stack = Stack::new();

        stack.map_begin().unwrap();

        assert!(stack.map_value().is_err());
    }

    #[test]
    fn error_end_incomplete_map() {
        let mut stack = Stack::new();

        stack.map_begin().unwrap();
        stack.map_key().unwrap();

        assert!(stack.map_end().is_err());
    }

    #[test]
    fn error_map_key_outside_map() {
        let mut stack = Stack::new();

        assert!(stack.map_key().is_err());
    }

    #[test]
    fn error_map_value_outside_map() {
        let mut stack = Stack::new();

        assert!(stack.map_value().is_err());
    }

    #[test]
    fn simple_seq() {
        let mut stack = Stack::new();

        stack.seq_begin().unwrap();

        stack.seq_elem().unwrap();
        stack.primitive().unwrap();

        stack.seq_end().unwrap();

        stack.end().unwrap();
    }

    #[test]
    fn empty_seq() {
        let mut stack = Stack::new();

        stack.seq_begin().unwrap();
        stack.seq_end().unwrap();

        stack.end().unwrap();
    }

    #[test]
    fn error_primitive_without_seq_elem() {
        let mut stack = Stack::new();

        stack.seq_begin().unwrap();

        assert!(stack.primitive().is_err());
    }

    #[test]
    fn error_end_incomplete_seq() {
        let mut stack = Stack::new();

        stack.seq_begin().unwrap();

        stack.seq_elem().unwrap();

        assert!(stack.seq_end().is_err());
    }

    #[test]
    fn error_seq_elem_outside_seq() {
        let mut stack = Stack::new();

        assert!(stack.seq_elem().is_err());
    }
}
