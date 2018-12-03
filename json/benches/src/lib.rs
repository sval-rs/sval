#![cfg(test)]
#![feature(test)]

extern crate test;

use sval_json_tests::Twitter;

fn input_json() -> String {
    std::fs::read_to_string("../tests/twitter.json").unwrap()
}

fn input_struct() -> Twitter {
    let j = input_json();
    serde_json::from_str(&j).unwrap()
}

#[bench]
fn primitive_miniserde(b: &mut test::Bencher) {
    b.iter(|| {
        miniserde::json::to_string(&42);
    });
}

#[bench]
fn primitive_serde(b: &mut test::Bencher) {
    b.iter(|| {
        serde_json::to_string(&42).unwrap();
    });
}

#[bench]
fn primitive_erased_serde(b: &mut test::Bencher) {
    let s: Box<dyn erased_serde::Serialize> = Box::new(42);

    b.iter(|| {
        serde_json::to_string(&s).unwrap();
    });
}

#[bench]
fn primitive_sval(b: &mut test::Bencher) {
    b.iter(|| {
        sval_json::to_string(&42).unwrap();
    });
}

#[bench]
fn twitter_miniserde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        miniserde::json::to_string(&s);
    });
}

#[bench]
fn twitter_serde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        serde_json::to_string(&s).unwrap();
    });
}

#[bench]
fn twitter_erased_serde(b: &mut test::Bencher) {
    let s: Box<dyn erased_serde::Serialize> = Box::new(input_struct());
    b.iter(|| {
        serde_json::to_string(&s).unwrap();
    });
}

#[bench]
fn twitter_sval(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        sval_json::to_string(&s).unwrap();
    });
}

#[bench]
fn twitter_sval_to_serde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        serde_json::to_string(&sval::serde::to_serialize(&s)).unwrap();
    });
}

#[bench]
fn twitter_serde_to_sval(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        sval_json::to_string(sval::serde::to_value(&s)).unwrap();
    });
}

#[bench]
fn twitter_serde_to_sval_to_serde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        serde_json::to_string(&sval::serde::to_serialize(sval::serde::to_value(&s))).unwrap();
    });
}
