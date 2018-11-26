use std::{
    collections::BTreeMap,
    mem,
};

use serde_value::Id as SerdeId;
use sval_value::Id;

use sval::stream::{
    self,
    Stream,
};

struct Fmt {
    stack: stream::Stack,
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
    fn begin(&mut self) -> Result<(), stream::Error> {
        // Begin the stack
        self.stack.begin()?;

        Ok(())
    }

    fn fmt(&mut self, v: stream::Arguments) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        let delim = mem::replace(&mut self.delim, Self::next_delim(pos));
        print!("{}{:?}", delim, v);

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.stack.seq_begin()?;

        let delim = mem::replace(&mut self.delim, "");
        print!("{}[", delim);

        Ok(())
    }

    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.stack.seq_elem()?;

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.seq_end()?;

        self.delim = Self::next_delim(pos);
        print!("]");

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.stack.map_begin()?;

        let delim = mem::replace(&mut self.delim, "");
        print!("{}{{", delim);

        Ok(())
    }

    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.stack.map_key()?;

        Ok(())
    }

    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.stack.map_value()?;

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.map_end()?;

        self.delim = Self::next_delim(pos);
        print!("}}");

        Ok(())
    }

    fn end(&mut self) -> Result<(), stream::Error> {
        self.stack.end()?;

        println!();

        Ok(())
    }
}

fn main() {
    // A map that implements `sval::value::Value`
    let mut map = BTreeMap::new();

    map.insert(Id::new(1), vec!["Hello", "World"]);
    map.insert(Id::new(2), vec!["World", "Hello"]);

    // Write the map using `sval`
    sval::stream(
        &map,
        Fmt {
            stack: Default::default(),
            delim: "",
        },
    )
    .unwrap();

    // Write the map using `serde_json`
    println!("{}\n", serde_json::to_string(&sval::serde::to_serialize(&map)).unwrap());

    // A map that implements `serde::Serialize`
    let mut map = BTreeMap::new();

    // Write the map using `serde_json`
    println!("{}\n", serde_json::to_string(&map).unwrap());

    map.insert(SerdeId::new(1), vec!["Hello", "World"]);
    map.insert(SerdeId::new(2), vec!["World", "Hello"]);

    // Write the map using `sval`
    sval::serde::stream(
        &map,
        Fmt {
            stack: Default::default(),
            delim: "",
        },
    )
    .unwrap();
}
