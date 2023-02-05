use core::fmt;

use crate::writer::Writer;

/**
Adapt an [`sval::Value`] into a [`fmt::Debug`].
*/
pub fn to_debug<V: sval::Value>(value: V) -> ToDebug<V> {
    ToDebug(value)
}

/**
Adapt a reference to an [`sval::Value`] into a [`fmt::Debug`].
*/
pub fn to_debug_ref<'a, V: sval::Value + ?Sized>(value: &'a V) -> &'a ToDebug<V> {
    // SAFETY: `&'a V` and `&'a ToDebug<V>` have the same ABI
    unsafe { &*(value as *const _ as *const ToDebug<V>) }
}

/**
Adapt an [`sval::Value`] into a [`fmt::Debug`].
*/
#[repr(transparent)]
pub struct ToDebug<V: ?Sized>(V);

impl<V: sval::Value> fmt::Debug for ToDebug<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.stream(&mut Writer::new(f)).map_err(|_| fmt::Error)?;

        Ok(())
    }
}

impl<V: sval::Value> fmt::Display for ToDebug<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
