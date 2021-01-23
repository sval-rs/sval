#![cfg(feature = "std")]
#![feature(test)]

extern crate sval;
extern crate test;

use sval::value;

use test::{
    black_box,
    Bencher,
};

#[bench]
fn collect_primitive(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::collect(1);

        black_box(value);
    })
}

#[bench]
fn collect_primitive_string(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::collect("A string");

        black_box(value);
    })
}

#[bench]
fn allocate_string(b: &mut Bencher) {
    b.iter(|| {
        let value = String::from("A string");

        black_box(value);
    })
}

#[bench]
fn from_primitive(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::from(1);

        black_box(value);
    })
}

#[bench]
fn from_primitive_string(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::from("A string");

        black_box(value);
    })
}

#[bench]
fn clone_primitive(b: &mut Bencher) {
    let value = value::OwnedValue::collect(1);

    b.iter(|| {
        let value = value.clone();
        black_box(value);
    })
}

#[bench]
fn clone_primitive_string(b: &mut Bencher) {
    let value = value::OwnedValue::collect("A string");

    b.iter(|| {
        let value = value.clone();
        black_box(value);
    })
}

#[bench]
fn clone_primitive_allocated_string(b: &mut Bencher) {
    let value = value::OwnedValue::from(String::from("some string"));

    b.iter(|| {
        let value = value.clone();
        black_box(value);
    })
}

struct Map;
impl value::Value for Map {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        stream.map_key("a")?;
        stream.map_value(42)?;

        stream.map_key("b")?;
        stream.map_value(42)?;

        stream.map_key("c")?;
        stream.map_value(42)?;

        stream.map_key("d")?;
        stream.map_value(42)?;

        stream.map_key("e")?;
        stream.map_value(42)?;

        stream.map_end()
    }
}

#[bench]
fn collect_map(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::collect(Map);

        black_box(value);
    });
}

#[bench]
fn clone_map(b: &mut Bencher) {
    let value = value::OwnedValue::collect(Map);

    b.iter(|| {
        let value = value.clone();
        black_box(value);
    })
}
