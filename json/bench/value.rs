use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

#[path = "data.rs"]
mod data;

use data::*;

fn primitives(c: &mut Criterion) {
    c.bench_function("primitive_miniserde", |b| {
        b.iter(|| black_box(miniserde::json::to_string(&42)))
    });

    c.bench_function("primitive_serde", |b| {
        b.iter(|| black_box(serde_json::to_string(&42).unwrap()))
    });

    c.bench_function("primitive_erased_serde", |b| {
        b.iter(|| {
            let s: Box<dyn erased_serde::Serialize> = Box::new(42);
            black_box(serde_json::to_string(&*s).unwrap())
        })
    });

    c.bench_function("primitive_sval", |b| {
        b.iter(|| black_box(sval_json::stream_to_string(&42).unwrap()))
    });
}

fn twitter(c: &mut Criterion) {
    c.bench_function("twitter_miniserde", |b| {
        let s = input_struct();
        b.iter(|| black_box(miniserde::json::to_string(&s)))
    });

    c.bench_function("twitter_serde", |b| {
        let s = input_struct();
        b.iter(|| black_box(serde_json::to_string(&s).unwrap()))
    });

    c.bench_function("twitter_erased_serde", |b| {
        let s: Box<dyn erased_serde::Serialize> = Box::new(input_struct());
        b.iter(|| black_box(serde_json::to_string(&*s).unwrap()))
    });

    c.bench_function("twitter_sval", |b| {
        let s = input_struct();
        b.iter(|| black_box(sval_json::stream_to_string(&s).unwrap()))
    });

    c.bench_function("twitter_sval_dynamic", |b| {
        let s: Box<dyn sval_dynamic::Value> = Box::new(input_struct());
        b.iter(|| black_box(sval_json::stream_to_string(&*s).unwrap()))
    });

    c.bench_function("twitter_sval_to_serde", |b| {
        let s = input_struct();
        b.iter(|| black_box(serde_json::to_string(&sval_serde::ToSerialize::new(&s)).unwrap()))
    });

    c.bench_function("twitter_serde_to_sval", |b| {
        let s = input_struct();
        b.iter(|| black_box(sval_json::stream_to_string(sval_serde::ToValue::new(&s)).unwrap()))
    });

    c.bench_function("twitter_serde_to_sval_to_serde", |b| {
        let s = input_struct();
        b.iter(|| {
            black_box(
                serde_json::to_string(&sval_serde::ToSerialize::new(sval_serde::ToValue::new(&s)))
                    .unwrap(),
            )
        })
    });
}

fn collect(c: &mut Criterion) {
    c.bench_function("twitter_sval_collect", |b| {
        let s = input_struct();
        b.iter(|| black_box(sval_buffer::Value::collect(&s).unwrap()))
    });

    c.bench_function("twitter_sval_collect_owned", |b| {
        let s = input_struct();
        b.iter(|| {
            black_box(
                sval_buffer::Value::collect(&s)
                    .unwrap()
                    .into_owned()
                    .unwrap(),
            )
        })
    });

    c.bench_function("twitter_serde_collect", |b| {
        let s = input_struct();
        b.iter(|| black_box(serde_json::to_value(&s).unwrap()))
    });
}

criterion_group!(benches, primitives, twitter, collect);
criterion_main!(benches);
