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
        cmp::Ordering,
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
pub struct Label<'computed> {
    // This field may point to some external data borrowed for `'computed`
    // or to the `_value_owned` field. It could be a `Cow<'computed, str>`,
    // but this way is cheaper to access because it avoids checking the
    // `Cow` variant
    value_computed: *const str,
    // Only one `backing_field_*` may be `Some`
    backing_field_static: Option<&'static str>,
    #[cfg(feature = "alloc")]
    backing_field_owned: Option<String>,
    _marker: PhantomData<&'computed str>,
}

// SAFETY: Label doesn't mutate or synchronize: it acts just like a `&str`
unsafe impl<'computed> Send for Label<'computed> {}
// SAFETY: Label doesn't mutate or synchronize: it acts just like a `&str`
unsafe impl<'computed> Sync for Label<'computed> {}

#[cfg(not(feature = "alloc"))]
impl<'computed> Clone for Label<'computed> {
    fn clone(&self) -> Self {
        Label {
            value_computed: self.value_computed,
            backing_field_static: self.backing_field_static,
            _marker: PhantomData,
        }
    }
}

impl<'computed> Label<'computed> {
    /**
    Create a new label from a static string value.

    For labels that can't satisfy the `'static` lifetime, use [`Label::new_computed`].
    For labels that need owned values, use [`Label::new_owned`].
    */
    pub const fn new(label: &'static str) -> Self {
        Label {
            value_computed: label as *const str,
            backing_field_static: Some(label),
            #[cfg(feature = "alloc")]
            backing_field_owned: None,
            _marker: PhantomData,
        }
    }

    /**
    Create a new label from a string value borrowed for the `'computed` lifetime.
    */
    pub const fn new_computed(label: &'computed str) -> Self {
        Label {
            value_computed: label as *const str,
            backing_field_static: None,
            #[cfg(feature = "alloc")]
            backing_field_owned: None,
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
        self.backing_field_static
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

The contents of a tag aren't considered public, only equality between two tag identifiers.
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

    // NOTE: This method is only private to avoid exposing it prematurely
    // There's no real reason we shouldn't
    const fn cloned(&self) -> Tag {
        Tag {
            id: self.id,
            data: self.data,
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
#[derive(Clone)]
pub struct Index(usize, Option<Tag>);

impl Index {
    /**
    Create a new index from a numeric value.
    */
    pub const fn new(index: usize) -> Self {
        Index(index, None)
    }

    /**
    Create a new None index from a 32bit numeric value.
    */
    pub const fn new_u32(index: u32) -> Self {
        Index(index as usize, None)
    }

    /**
    Associate a tag as a hint with this index.

    Tags don't contribute to equality or ordering of indexes but streams may
    use the them when interpreting the index value.
    */
    pub const fn with_tag(self, tag: &Tag) -> Self {
        Index(self.0, Some(tag.cloned()))
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

    /**
    Get the tag hint associated with the index, if present.

    Streams may use the tag when interpreting the index value.
    */
    pub const fn tag(&self) -> Option<&Tag> {
        self.1.as_ref()
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Index")
            .field("value", &self.0)
            .field("<tag>", &self.1)
            .finish()
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Index {}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Hash for Index {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
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

    fn tag(&self) -> Option<Tag> {
        None
    }

    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::String;

    impl<'computed> Clone for Label<'computed> {
        fn clone(&self) -> Self {
            if let Some(ref owned) = self.backing_field_owned {
                Label::new_owned(owned.clone())
            } else {
                Label {
                    value_computed: self.value_computed,
                    backing_field_static: self.backing_field_static,
                    backing_field_owned: None,
                    _marker: PhantomData,
                }
            }
        }
    }

    impl<'computed> Label<'computed> {
        /**
        Create an owned label from this one.

        This method will allocate if the label isn't based on a static string.
        */
        pub fn to_owned(&self) -> Label<'static> {
            if let Some(backing_field_static) = self.backing_field_static {
                Label::new(backing_field_static)
            } else {
                Label::new_owned(self.as_str().into())
            }
        }
    }

    impl Label<'static> {
        /**
        Create a new label from an owned string value.
        */
        pub fn new_owned(label: String) -> Self {
            Label {
                value_computed: label.as_str() as *const str,
                backing_field_static: None,
                backing_field_owned: Some(label),
                _marker: PhantomData,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_tag() {
        assert_eq!(Some(tags::RUST_UNIT), ().tag());
    }

    #[test]
    fn label_send_sync() {
        fn assert<T: Send + Sync>() {}

        assert::<Label>();
    }

    #[test]
    fn label_static() {
        let label = Label::new("a");

        assert_eq!("a", label.as_static_str().unwrap());
        assert_eq!("a", label.as_str());
    }

    #[test]
    fn label_computed() {
        let label = Label::new_computed("a");

        assert!(label.as_static_str().is_none());
        assert_eq!("a", label.as_str());
    }

    #[test]
    fn label_eq() {
        let a = Label::new("a");
        let b = Label::new_computed("a");
        let c = Label::new("b");

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn label_clone() {
        let a = Label::new("a");
        let b = a.clone();

        assert_eq!(a.value_computed, b.value_computed);
        assert_eq!(a.backing_field_static, b.backing_field_static);
    }

    #[test]
    fn index() {
        let small = Index::new(1);
        let large = Index::new(usize::MAX);

        if usize::MAX > (u32::MAX as usize) {
            assert!(large.to_u32().is_none());
        }

        assert_eq!(1, small.to_u32().unwrap());
    }

    #[test]
    fn index_tag() {
        let index = Index::new(1).with_tag(&tags::VALUE_OFFSET);

        assert_eq!(Some(&tags::VALUE_OFFSET), index.tag());
    }

    #[test]
    fn index_eq() {
        let a = Index::new(1);
        let b = Index::new_u32(1);
        let c = Index::new(2);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn tag_eq() {
        let a = Tag::new("a");
        let b = Tag::new("a");
        let c = Tag::new("b");

        assert_eq!(a.id, b.id);
        assert_ne!(a.id, c.id);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn tag_match() {
        const A: Tag = Tag::new("a");

        let a = Tag::new("a");

        match a {
            A => (),
            _ => panic!("unexpected tag `{:?}`", a),
        }
    }

    #[cfg(feature = "alloc")]
    mod alloc_support {
        use crate::data::*;

        #[test]
        fn label_owned() {
            let label = Label::new_owned(String::from("a"));

            assert!(label.as_static_str().is_none());
            assert_eq!("a", label.as_str());
        }

        #[test]
        fn label_owned_clone() {
            let a = Label::new_owned(String::from("a"));
            let b = a.clone();

            assert_ne!(
                a.backing_field_owned.as_ref().unwrap().as_ptr(),
                b.backing_field_owned.as_ref().unwrap().as_ptr()
            );
        }
    }
}
