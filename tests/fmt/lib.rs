#![cfg(test)]

extern crate sval;

use std::fmt::{
    self,
    Debug,
};

use sval::value::{
    self,
    Value,
};

struct OuterMap;
impl Debug for OuterMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = f.debug_map();

        map.entry(&1, &1);
        map.entry(&"a", &"a");
        map.entry(&"map", &InnerMap);
        map.entry(&"seq", &InnerSeq);
        map.entry(&"empty_map", &EmptyMap);
        map.entry(&"empty_seq", &EmptySeq);

        map.finish()
    }
}

impl Value for OuterMap {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        stream.map_key(1)?;
        stream.map_value(1)?;

        stream.map_key("a")?;
        stream.map_value("a")?;

        stream.map_key("map")?;
        stream.map_value(InnerMap)?;

        stream.map_key("seq")?;
        stream.map_value(InnerSeq)?;

        stream.map_key("empty_map")?;
        stream.map_value(EmptyMap)?;

        stream.map_key("empty_seq")?;
        stream.map_value(EmptySeq)?;

        stream.map_end()
    }
}

struct InnerMap;
impl Debug for InnerMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = f.debug_map();

        map.entry(&1, &1);
        map.entry(&"a", &"a");

        map.finish()
    }
}

impl Value for InnerMap {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        stream.map_key(1)?;
        stream.map_value(1)?;

        stream.map_key("a")?;
        stream.map_value("a")?;

        stream.map_end()
    }
}

struct EmptyMap;
impl Debug for EmptyMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = f.debug_map();
        map.finish()
    }
}

impl Value for EmptyMap {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;
        stream.map_end()
    }
}

struct OuterSeq;
impl Debug for OuterSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();

        list.entry(&1);
        list.entry(&"a");
        list.entry(&InnerMap);
        list.entry(&InnerSeq);
        list.entry(&EmptyMap);
        list.entry(&EmptySeq);

        list.finish()
    }
}

impl Value for OuterSeq {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.seq_begin(None)?;

        stream.seq_elem(1)?;
        stream.seq_elem("a")?;
        stream.seq_elem(InnerMap)?;
        stream.seq_elem(InnerSeq)?;
        stream.seq_elem(EmptyMap)?;
        stream.seq_elem(EmptySeq)?;

        stream.seq_end()
    }
}

struct InnerSeq;
impl Debug for InnerSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();

        list.entry(&1);
        list.entry(&"a");

        list.finish()
    }
}

impl Value for InnerSeq {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.seq_begin(None)?;

        stream.seq_elem(1)?;
        stream.seq_elem("a")?;

        stream.seq_end()
    }
}

struct EmptySeq;
impl Debug for EmptySeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();
        list.finish()
    }
}

impl Value for EmptySeq {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.seq_begin(None)?;
        stream.seq_end()
    }
}

struct WeirdMapKeys;
impl Debug for WeirdMapKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = f.debug_map();

        map.entry(&InnerMap, &1);
        map.entry(&InnerSeq, &"a");

        map.finish()
    }
}

impl Value for WeirdMapKeys {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        stream.map_key(InnerMap)?;
        stream.map_value(1)?;

        stream.map_key(InnerSeq)?;
        stream.map_value("a")?;

        stream.map_end()
    }
}

#[test]
fn sval_fmt_is_consistent() {
    fn check(value: (impl Value + Debug)) {
        let sval = format!("{:?}", sval::fmt::to_debug(&value));
        let std = format!("{:?}", value);

        assert_eq!(std, sval);
    }

    check(42);
    check("a string");
    check(OuterMap);
    check(OuterSeq);
    check(WeirdMapKeys);
}

#[test]
fn sval_alternate_fmt_is_consistent() {
    fn check(value: (impl Value + Debug)) {
        let sval = format!("{:#?}", sval::fmt::to_debug(&value));
        let std = format!("{:#?}", value);

        assert_eq!(std, sval);
    }

    check(42);
    check("a string");
    check(OuterMap);
    check(OuterSeq);
    check(WeirdMapKeys);
}

#[test]
fn sval_fmt_retains_flags() {
    fn check(value: (impl Value + Debug)) {
        let sval = format!("{:04?}", sval::fmt::to_debug(&value));
        let std = format!("{:04?}", value);

        assert_eq!(std, sval);
    }

    check(42);
    check("a string");
    check(OuterMap);
    check(OuterSeq);
    check(WeirdMapKeys);
}
