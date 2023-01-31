use core::fmt;

use crate::writer::Writer;

pub fn to_debug<V: sval::Value>(value: V) -> ToDebug<V> {
    ToDebug(value)
}

pub struct ToDebug<V>(V);

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
