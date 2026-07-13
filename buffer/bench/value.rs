use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

#[macro_use]
extern crate sval_derive_macros;

#[path = "data.rs"]
mod data;

use data::*;

fn export_logs_service_request(c: &mut Criterion) {
    c.bench_function("export_logs_service_request", |b| {
        b.iter(|| export_logs_service_request_data())
    });

    c.bench_function("export_logs_service_request_collect_ref", |b| {
        b.iter(|| {
            let data = export_logs_service_request_data();
            black_box(sval_buffer::ValueBuf::collect(&data).unwrap());
        })
    });

    c.bench_function("export_logs_service_request_collect", |b| {
        b.iter(|| {
            let data = export_logs_service_request_data();
            black_box(sval_buffer::ValueBuf::collect_owned(data).unwrap());
        })
    });

    c.bench_function("export_logs_service_request_collect_ref_to_owned", |b| {
        b.iter(|| {
            let data = export_logs_service_request_data();
            sval_buffer::ValueBuf::collect(&data)
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });

    c.bench_function("export_logs_service_request_collect_to_owned", |b| {
        b.iter(|| {
            let data = export_logs_service_request_data();
            sval_buffer::ValueBuf::collect_owned(&data)
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });
}

fn borrowed(c: &mut Criterion) {
    c.bench_function("borrowed", |b| b.iter(|| borrowed_data()));

    c.bench_function("borrowed_collect_ref", |b| {
        b.iter(|| {
            let data = borrowed_data();
            black_box(sval_buffer::ValueBuf::collect(&data).unwrap());
        })
    });

    c.bench_function("borrowed_collect", |b| {
        b.iter(|| {
            let data = borrowed_data();
            black_box(sval_buffer::ValueBuf::collect_owned(data).unwrap());
        })
    });

    c.bench_function("borrowed_collect_ref_to_owned", |b| {
        b.iter(|| {
            let data = borrowed_data();
            sval_buffer::ValueBuf::collect(&data)
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });

    c.bench_function("borrowed_collect_to_owned", |b| {
        b.iter(|| {
            let data = borrowed_data();
            sval_buffer::ValueBuf::collect_owned(&data)
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });
}

fn owned(c: &mut Criterion) {
    c.bench_function("owned", |b| b.iter(|| owned_data()));

    c.bench_function("owned_collect_ref", |b| {
        b.iter(|| {
            let data = owned_data();
            black_box(sval_buffer::ValueBuf::collect(&data).unwrap());
        })
    });

    c.bench_function("owned_collect", |b| {
        b.iter(|| {
            let data = owned_data();
            black_box(sval_buffer::ValueBuf::collect_owned(data).unwrap());
        })
    });

    c.bench_function("owned_collect_ref_to_owned", |b| {
        b.iter(|| {
            let data = owned_data();
            sval_buffer::ValueBuf::collect(&data)
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });

    c.bench_function("owned_collect_to_owned", |b| {
        b.iter(|| {
            let data = owned_data();
            sval_buffer::ValueBuf::collect_owned(&data)
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });
}

fn into_owned(c: &mut Criterion) {
    // The conversion on its own, starting from an already-collected borrowed buffer
    c.bench_function("borrowed_into_owned", |b| {
        let data = borrowed_data();

        b.iter_batched(
            || sval_buffer::ValueBuf::collect(&data).unwrap(),
            |buf| buf.into_owned().unwrap(),
            BatchSize::SmallInput,
        )
    });

    // The conversion when there's nothing borrowed left to convert
    c.bench_function("computed_into_owned", |b| {
        let data = borrowed_data();

        b.iter_batched(
            || sval_buffer::ValueBuf::collect_owned(&data).unwrap(),
            |buf| buf.into_owned().unwrap(),
            BatchSize::SmallInput,
        )
    });
}

fn clone(c: &mut Criterion) {
    // Cloning a buffer with no owned parts (a raw byte copy)
    c.bench_function("clone_borrowed", |b| {
        let data = borrowed_data();
        let value = sval_buffer::Value::collect(&data).unwrap();

        b.iter(|| black_box(value.clone()))
    });

    // Cloning (and dropping) a buffer where the text parts are owned
    c.bench_function("clone_owned", |b| {
        let data = borrowed_data();
        let value = sval_buffer::Value::collect(&data)
            .unwrap()
            .into_owned()
            .unwrap();

        b.iter(|| black_box(value.clone()))
    });
}

fn stream(c: &mut Criterion) {
    struct Count(usize);

    impl<'sval> sval::Stream<'sval> for Count {
        fn null(&mut self) -> sval::Result {
            self.0 += 1;
            Ok(())
        }

        fn bool(&mut self, _: bool) -> sval::Result {
            self.0 += 1;
            Ok(())
        }

        fn i64(&mut self, _: i64) -> sval::Result {
            self.0 += 1;
            Ok(())
        }

        fn f64(&mut self, _: f64) -> sval::Result {
            self.0 += 1;
            Ok(())
        }

        fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
            Ok(())
        }

        fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
            self.0 += fragment.len();
            Ok(())
        }

        fn text_end(&mut self) -> sval::Result {
            Ok(())
        }

        fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
            Ok(())
        }

        fn seq_value_begin(&mut self) -> sval::Result {
            Ok(())
        }

        fn seq_value_end(&mut self) -> sval::Result {
            Ok(())
        }

        fn seq_end(&mut self) -> sval::Result {
            Ok(())
        }
    }

    // Streaming a buffered value back out
    c.bench_function("stream_borrowed", |b| {
        let data = borrowed_data();
        let value = sval_buffer::Value::collect(&data).unwrap();

        b.iter(|| {
            let mut stream = Count(0);
            sval::stream(&mut stream, &value).unwrap();
            black_box(stream.0)
        })
    });
}

fn text_heavy(c: &mut Criterion) {
    // Many small strings: the per-text costs (borrowed conversion, computed
    // buffering) dominate
    let attributes: Vec<String> = (0..16).map(|i| format!("attribute-value-{i}")).collect();
    let attributes: Vec<&str> = attributes.iter().map(|a| a.as_str()).collect();
    let attributes = attributes.as_slice();

    c.bench_function("text_heavy_collect_ref_to_owned", |b| {
        b.iter(|| {
            sval_buffer::ValueBuf::collect(black_box(&attributes))
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });

    c.bench_function("text_heavy_collect_owned", |b| {
        b.iter(|| black_box(sval_buffer::ValueBuf::collect_owned(black_box(&attributes)).unwrap()))
    });

    c.bench_function("text_heavy_clone_owned", |b| {
        let value = sval_buffer::Value::collect(&attributes)
            .unwrap()
            .into_owned()
            .unwrap();

        b.iter(|| black_box(value.clone()))
    });
}

fn histogram(c: &mut Criterion) {
    // An exponential histogram: a seq of 160 bucket midpoint/count tuples.
    // Nothing is borrowed or owned, so this measures pure container and
    // primitive encoding
    let buckets = histogram_data();
    let buckets = buckets.as_slice();

    c.bench_function("histogram_collect", |b| {
        b.iter(|| black_box(sval_buffer::ValueBuf::collect(black_box(&buckets)).unwrap()))
    });

    c.bench_function("histogram_collect_to_owned", |b| {
        b.iter(|| {
            sval_buffer::ValueBuf::collect(black_box(&buckets))
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });
}

fn small_key_map(c: &mut Criterion) {
    // A map of 32 small borrowed string keys to integers
    let map = small_key_map_data();

    c.bench_function("small_key_map_collect_ref", |b| {
        b.iter(|| black_box(sval_buffer::ValueBuf::collect(black_box(&map)).unwrap()))
    });

    c.bench_function("small_key_map_collect_ref_to_owned", |b| {
        b.iter(|| {
            sval_buffer::ValueBuf::collect(black_box(&map))
                .unwrap()
                .into_owned()
                .unwrap()
        })
    });

    // The conversion on its own
    c.bench_function("small_key_map_into_owned", |b| {
        b.iter_batched(
            || sval_buffer::ValueBuf::collect(&map).unwrap(),
            |buf| buf.into_owned().unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    borrowed,
    owned,
    into_owned,
    clone,
    stream,
    text_heavy,
    histogram,
    small_key_map,
    export_logs_service_request
);
criterion_main!(benches);
