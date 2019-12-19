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
fn primitive_sval_noop(b: &mut test::Bencher) {
    b.iter(|| {
        sval_noop(&42).unwrap();
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
fn twitter_sval_noop(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        sval_noop(&s).unwrap();
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

fn sval_noop(v: impl sval::Value) -> Result<(), sval::Error> {
    struct NoOp;

    impl sval::Stream for NoOp {
        #[inline(never)]
        fn fmt(&mut self, v: sval::stream::Arguments) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn i64(&mut self, v: i64) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn u64(&mut self, v: u64) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn i128(&mut self, v: i128) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn u128(&mut self, v: u128) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn f64(&mut self, v: f64) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn bool(&mut self, v: bool) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn char(&mut self, v: char) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn str(&mut self, v: &str) -> sval::stream::Result {
            let _ = v;
            Ok(())
        }

        #[inline(never)]
        fn none(&mut self) -> sval::stream::Result {
            Ok(())
        }

        #[inline(never)]
        fn map_begin(&mut self, len: Option<usize>) -> sval::stream::Result {
            let _ = len;
            Ok(())
        }

        #[inline(never)]
        fn map_key(&mut self) -> sval::stream::Result {
            Ok(())
        }

        #[inline(never)]
        fn map_value(&mut self) -> sval::stream::Result {
            Ok(())
        }

        #[inline(never)]
        fn map_end(&mut self) -> sval::stream::Result {
            Ok(())
        }

        #[inline(never)]
        fn seq_begin(&mut self, len: Option<usize>) -> sval::stream::Result {
            let _ = len;
            Ok(())
        }

        #[inline(never)]
        fn seq_elem(&mut self) -> sval::stream::Result {
            Ok(())
        }

        #[inline(never)]
        fn seq_end(&mut self) -> sval::stream::Result {
            Ok(())
        }
    }

    sval::stream(NoOp, v)?;

    Ok(())
}
