use core::fmt;

use crate::writer::Writer;

/**
Adapt an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].
*/
#[repr(transparent)]
pub struct ToFmt<V: ?Sized>(V);

impl<V: sval::Value> ToFmt<V> {
    /**
    Adapt an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].
    */
    pub fn new(value: V) -> ToFmt<V> {
        ToFmt(value)
    }
}

impl<V: sval::Value + ?Sized> ToFmt<V> {
    /**
    Adapt a reference to an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].
    */
    pub fn new_borrowed<'a>(value: &'a V) -> &'a ToFmt<V> {
        // SAFETY: `&'a V` and `&'a ToDebug<V>` have the same ABI
        unsafe { &*(value as *const _ as *const ToFmt<V>) }
    }
}

impl<V: sval::Value> fmt::Debug for ToFmt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.stream(&mut Writer::new(f)).map_err(|_| fmt::Error)?;

        Ok(())
    }
}

impl<V: sval::Value> fmt::Display for ToFmt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
