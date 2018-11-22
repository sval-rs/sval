use crate::{
    std::fmt,
    stream,
    test::{
        black_box,
        Bencher,
    },
    value,
};

struct EmptyStream;

impl stream::Stream for EmptyStream {
    #[inline(never)]
    fn fmt(&mut self, _: stream::Pos, _: fmt::Arguments) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn u64(&mut self, _: stream::Pos, _: u64) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn begin(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn end(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn map_begin(&mut self, _: stream::Pos, _: Option<usize>) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn map_end(&mut self, _: stream::Pos) -> Result<(), stream::Error> {
        Ok(())
    }
}

#[bench]
fn checked_begin(b: &mut Bencher) {
    b.iter(|| {
        let mut stream = EmptyStream;
        let stream = value::Stream::begin(&mut stream).unwrap();

        black_box(stream);
    })
}

#[bench]
fn stack_map(b: &mut Bencher) {
    b.iter(|| {
        let mut stack = value::Stack::new();

        stack.map_begin().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();

        stack.map_value().unwrap();
        stack.map_begin().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();

        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_end().unwrap();
        stack.map_end().unwrap();

        stack.end().unwrap();

        black_box(stack);
    })
}

#[bench]
fn stack_primitive(b: &mut Bencher) {
    b.iter(|| {
        let mut stack = value::Stack::new();

        stack.primitive().unwrap();
        stack.end().unwrap();

        black_box(stack);
    })
}

#[bench]
fn checked_stream_map(b: &mut Bencher) {
    b.iter(|| {
        let mut stream = EmptyStream;

        {
            let mut stream = value::Stream::begin(&mut stream).unwrap();

            stream.map_begin(None).unwrap();
            stream.map_key().unwrap().u64(1).unwrap();

            stream.map_value().unwrap().map_begin(None).unwrap();
            stream.map_key().unwrap().u64(2).unwrap();
            stream.map_value().unwrap().u64(42).unwrap();
            stream.map_end().unwrap();

            stream.map_end().unwrap();
            stream.end().unwrap();
        }

        black_box(stream);
    })
}

#[bench]
fn unchecked_stream_map(b: &mut Bencher) {
    b.iter(|| {
        let stream: &mut dyn stream::Stream = &mut EmptyStream;

        stream.map_begin(stream::Pos::Root, None).unwrap();
        stream.u64(stream::Pos::Key, 1).unwrap();

        stream.map_begin(stream::Pos::Value, None).unwrap();
        stream.u64(stream::Pos::Key, 2).unwrap();
        stream.u64(stream::Pos::Value, 42).unwrap();
        stream.map_end(stream::Pos::Value).unwrap();

        stream.map_end(stream::Pos::Root).unwrap();
        stream.end().unwrap();

        black_box(stream);
    })
}
