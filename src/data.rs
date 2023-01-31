mod binary;
mod map;
mod number;
mod option;
mod seq;
mod text;

pub mod tags;

use crate::{
    std::{
        borrow::Borrow,
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
    },
    Result, Stream, Value,
};

#[cfg(feature = "alloc")]
use crate::std::string::String;

pub(crate) use self::number::*;

pub use self::{binary::*, text::*};

/**
A textual label for some value.
*/
// NOTE: Implementing `Clone` on this type would need to be done manually
// If `value_computed` points to `_value_owned` then the clone will need
// to update its pointer accordingly
pub struct Label<'computed> {
    // This field may point to some external data borrowed for `'computed`
    // or to the `_value_owned` field. It could be a `Cow<'computed, str>`,
    // but this way is cheaper to access because it avoids checking the
    // `Cow` variant
    value_computed: *const str,
    value_static: Option<&'static str>,
    #[cfg(feature = "alloc")]
    _value_owned: Option<String>,
    _marker: PhantomData<&'computed str>,
}

impl<'computed> Label<'computed> {
    /**
    Create a new label from a static string value.

    For labels that can't satisfy the `'static` lifetime, use [`Label::from_computed`].
    For labels that need owned values, use [`Label::from_owned`].
    */
    pub const fn new(label: &'static str) -> Self {
        Label {
            value_computed: label as *const str,
            value_static: Some(label),
            #[cfg(feature = "alloc")]
            _value_owned: None,
            _marker: PhantomData,
        }
    }

    /**
    Create a new label from a string value borrowed for the `'computed` lifetime.
    */
    pub const fn from_computed(label: &'computed str) -> Self {
        Label {
            value_computed: label as *const str,
            value_static: None,
            #[cfg(feature = "alloc")]
            _value_owned: None,
            _marker: PhantomData,
        }
    }

    /**
    Get the value of the label as a string.
    */
    pub const fn as_str(&self) -> &str {
        // SAFETY: The borrow of the `value_computed` field can't outlive
        // the label itself, so even if `value_computed` points to `_value_owned`
        // it will never dangle.
        unsafe { &*self.value_computed }
    }

    /**
    Try get the value of the label as a static string.

    For labels that were created over computed data this method will return `None`.
    */
    pub const fn as_static_str(&self) -> Option<&'static str> {
        self.value_static
    }
}

impl<'a, 'b> PartialEq<Label<'b>> for Label<'a> {
    fn eq(&self, other: &Label<'b>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> Eq for Label<'a> {}

impl<'a> Hash for Label<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<'a> Borrow<str> for Label<'a> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> fmt::Debug for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Label").field(&self.as_str()).finish()
    }
}

/**
A type tag for a value.

Tags are additional hints that a stream may use to interpret a value differently,
or to avoid some unnecessary work.
*/
#[derive(Clone, PartialEq, Eq)]
pub struct Tag {
    id: u64,
    data: &'static str,
}

impl Tag {
    /**
    Create a new tag from a static string value.
    */
    pub const fn new(data: &'static str) -> Self {
        // Fast, non-cryptographic hash used by rustc and Firefox.
        // Adapted from: https://github.com/rust-lang/rustc-hash/blob/master/src/lib.rs to work in CTFE
        //
        // We use hashes for quick tag comparison, if they collide then we'll compare the full value
        const fn compute_id(bytes: &[u8]) -> u64 {
            // Copyright 2015 The Rust Project Developers. See the COPYRIGHT
            // file at the top-level directory of this distribution and at
            // http://rust-lang.org/COPYRIGHT.
            //
            // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
            // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
            // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
            // option. This file may not be copied, modified, or distributed
            // except according to those terms.

            const K: u64 = 0x517cc1b727220a95u64;

            let mut hash = 0u64;
            let mut b = 0;

            while b + 8 <= bytes.len() {
                let i = [
                    bytes[b + 0],
                    bytes[b + 1],
                    bytes[b + 2],
                    bytes[b + 3],
                    bytes[b + 4],
                    bytes[b + 5],
                    bytes[b + 6],
                    bytes[b + 7],
                ];

                let i = u64::from_ne_bytes(i);

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);

                b += 8;
            }

            if b + 4 <= bytes.len() {
                let i = [bytes[b + 0], bytes[b + 1], bytes[b + 2], bytes[b + 3]];

                let i = u32::from_ne_bytes(i) as u64;

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);

                b += 4;
            }

            if b + 2 <= bytes.len() {
                let i = [bytes[b + 0], bytes[b + 1]];

                let i = u16::from_ne_bytes(i) as u64;

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);

                b += 2;
            }

            if b + 1 <= bytes.len() {
                let i = bytes[b + 0] as u64;

                hash = (hash.rotate_left(5) ^ i).wrapping_mul(K);
            }

            hash
        }

        Tag {
            id: compute_id(data.as_bytes()),
            data,
        }
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Tag").field(&self.data).finish()
    }
}

/**
The index of a value in its parent context.
*/
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index(usize);

impl Index {
    /**
    Create a new index from a numeric value.
    */
    pub const fn new(index: usize) -> Self {
        Index(index)
    }

    /**
    Create a new index from a 32bit numeric value.
    */
    pub const fn new_u32(index: u32) -> Self {
        Index(index as usize)
    }

    /**
    Try get the index as a numeric value.
    */
    pub const fn to_usize(&self) -> Option<usize> {
        Some(self.0)
    }

    /**
    Try get the index as a 32-bit numeric value.
    */
    pub const fn to_u32(&self) -> Option<u32> {
        if self.0 <= u32::MAX as usize {
            Some(self.0 as u32)
        } else {
            None
        }
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Index").field(&self.0).finish()
    }
}

impl Value for () {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.tag(Some(&tags::RUST_UNIT), None, None)
    }
}

impl Value for bool {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.bool(*self)
    }

    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::String;

    impl<'computed> Label<'computed> {
        /**
        Create an owned label from this one.

        This method will allocate if the label isn't based on a static string.
        */
        pub fn to_owned(&self) -> Label<'static> {
            if let Some(value_static) = self.value_static {
                Label::new(value_static)
            } else {
                Label::from_owned(self.as_str().into())
            }
        }
    }

    impl Label<'static> {
        /**
        Create a new label from an owned string value.
        */
        pub fn from_owned(label: String) -> Self {
            Label {
                value_computed: label.as_str() as *const str,
                value_static: None,
                _value_owned: Some(label),
                _marker: PhantomData,
            }
        }
    }
}
