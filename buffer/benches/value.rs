#![feature(test)]

extern crate test;

#[cfg(feature = "alloc")]
#[derive(sval_derive::Value)]
struct OwnedData {
    id: i32,
    title: String,
    attributes: Vec<String>,
}

#[cfg(feature = "alloc")]
fn owned_data() -> OwnedData {
    OwnedData { id:  42, title: "A very important document".to_owned(), attributes: vec!["#1".to_owned(), "#2".to_owned(), "#3".to_owned()] }
}

#[cfg(feature = "alloc")]
#[bench]
fn owned(b: &mut test::Bencher) {
    b.iter(|| owned_data())
}

#[cfg(feature = "alloc")]
#[bench]
fn owned_collect_ref(b: &mut test::Bencher) {
    b.iter(|| {
        let data = owned_data();
        test::black_box(sval_buffer::ValueBuf::collect(&data).unwrap());
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn owned_collect(b: &mut test::Bencher) {
    b.iter(|| {
        let data = owned_data();
        test::black_box(sval_buffer::ValueBuf::collect_owned(data).unwrap());
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn owned_collect_ref_to_owned(b: &mut test::Bencher) {
    b.iter(|| {
        let data = owned_data();
        sval_buffer::ValueBuf::collect(&data).unwrap().into_owned().unwrap()
    })
}

#[cfg(feature = "alloc")]
#[bench]
fn owned_collect_to_owned(b: &mut test::Bencher) {
    b.iter(|| {
        let data = owned_data();
        sval_buffer::ValueBuf::collect_owned(&data).unwrap().into_owned().unwrap()
    })
}
