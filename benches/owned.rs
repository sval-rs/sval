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

struct Map;

impl value::Value for Map {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        stream.map_key(1)?;

        stream.map_value_begin()?.map_begin(None)?;

        stream.map_key(2)?;

        stream.map_value(42)?;

        stream.map_end()?;

        stream.map_end()
    }
}

#[bench]
fn collect_complex(b: &mut Bencher) {
    b.iter(|| {
        let value = value::OwnedValue::collect(Map);

        black_box(value);
    });
}

#[bench]
fn clone_complex(b: &mut Bencher) {
    let value = value::OwnedValue::collect(Map);

    b.iter(|| {
        let value = value.clone();
        black_box(value);
    })
}
