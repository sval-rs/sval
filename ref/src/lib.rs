/*!
A variant of [`sval::Value`] for types that store references internally.
*/

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate core;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, collections, string, vec},
        core::{convert, fmt, hash, marker, mem, ops, result, str, write},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

/**
Stream a value through a stream.
*/
pub fn stream_ref<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: impl ValueRef<'sval>,
    ) -> Result {
    value.stream_ref(stream)
}

use sval::{Value, Stream, Result};

/**
A producer of structured data that stores a reference internally.

This trait is a variant of [`Value`] for wrapper types that keep a reference to a value internally.
In `Value`, the `'sval` lifetime comes from the borrow of `&'sval self`. In `ValueRef`, it comes
from the `'sval` lifetime in the trait itself.
*/
pub trait ValueRef<'sval>: Value {
    /**
    Stream this value through a [`Stream`].
    */
    fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result;
}

macro_rules! impl_value_ref_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
                let $bind = self;
                ($($forward)*).stream_ref(stream)
            }
        }
    };
}

impl_value_ref_forward!({impl<'sval, 'a, T: ValueRef<'sval> + ?Sized> ValueRef<'sval> for &'a T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_value_ref_forward!({impl<'sval, T: ValueRef<'sval> + ?Sized> ValueRef<'sval> for Box<T>} => x => { **x });
}
