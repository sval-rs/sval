use std::{
    collections::BTreeMap,
    fmt,
    mem,
};

use sval_value::Id;

use sval::stream::{self, Stream};

struct Fmt {
    delim: &'static str,
}

impl Fmt {
    fn next_delim(pos: stream::Pos) -> &'static str {
        use sval::stream::Pos::*;

        match pos {
            Root => "",
            Key => ": ",
            Value | Elem => ", ",
        }
    }
}

impl Stream for Fmt {
    fn fmt(&mut self, pos: stream::Pos, v: stream::Arguments) -> Result<(), stream::Error> {
        let delim = mem::replace(&mut self.delim, Self::next_delim(pos));
        print!("{}{:?}", delim, v);

        Ok(())
    }

    fn seq_begin(&mut self, _: stream::Pos, _: Option<usize>) -> Result<(), stream::Error> {
        let delim = mem::replace(&mut self.delim, "");
        print!("{}[", delim);

        Ok(())
    }

    fn seq_end(&mut self, pos: stream::Pos) -> Result<(), stream::Error> {
        self.delim = Self::next_delim(pos);
        print!("]");

        Ok(())
    }

    fn map_begin(&mut self, _: stream::Pos, _: Option<usize>) -> Result<(), stream::Error> {
        let delim = mem::replace(&mut self.delim, "");
        print!("{}{{", delim);

        Ok(())
    }

    fn map_end(&mut self, pos: stream::Pos) -> Result<(), stream::Error> {
        self.delim = Self::next_delim(pos);
        print!("}}");

        Ok(())
    }

    fn end(&mut self) -> Result<(), stream::Error> {
        println!();

        Ok(())
    }
}

fn main() {
    // A map that implements `sval::value::Value`
    let mut map = BTreeMap::new();

    map.insert(Id::new(1), vec!["Hello", "World"]);
    map.insert(Id::new(2), vec!["World", "Hello"]);

    sval::stream(map, Fmt { delim: "" }).unwrap();
}
