#![feature(test)]

extern crate sval;
extern crate test;

use sval::{
    stream::{
        self,
        Stream,
    },
    value,
};

use test::{
    black_box,
    Bencher,
};

struct EmptyStream;

impl Stream for EmptyStream {
    #[inline(never)]
    fn str(&mut self, _: &str) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn u64(&mut self, _: u64) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn i64(&mut self, _: i64) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_elem(&mut self) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_end(&mut self) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_key(&mut self) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_value(&mut self) -> stream::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_end(&mut self) -> stream::Result {
        Ok(())
    }
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
fn stack_map(b: &mut Bencher) {
    b.iter(|| {
        let mut stack = stream::Stack::new();

        stack.map_begin().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();
        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();
        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();
        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();
        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();
        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_end().unwrap();

        stack.end().unwrap();

        black_box(stack);
    })
}

#[bench]
fn stack_new(b: &mut Bencher) {
    b.iter(|| {
        let stack = stream::Stack::new();

        black_box(stack);
    })
}

#[bench]
fn stack_primitive(b: &mut Bencher) {
    b.iter(|| {
        let mut stack = stream::Stack::new();

        stack.primitive().unwrap();

        black_box(stack);
    })
}

#[bench]
fn raw_stream_map(b: &mut Bencher) {
    b.iter(|| {
        let stream: &mut dyn Stream = &mut EmptyStream;

        stream.map_begin(None).unwrap();

        stream.map_key().unwrap();
        stream.str("a").unwrap();

        stream.map_value().unwrap();
        stream.str("b").unwrap();

        stream.map_key().unwrap();
        stream.str("c").unwrap();

        stream.map_value().unwrap();
        stream.str("d").unwrap();

        stream.map_value().unwrap();
        stream.str("e").unwrap();

        stream.map_end().unwrap();

        black_box(stream);
    })
}

#[bench]
fn stream_map(b: &mut Bencher) {
    b.iter(|| sval::stream(EmptyStream, Map))
}
