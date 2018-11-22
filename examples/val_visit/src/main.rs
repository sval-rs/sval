use std::{
    collections::BTreeMap,
    fmt,
    mem,
};

use serde_value::{
    Complex,
    Id as SerdeId,
};
use val_value::Id;

use val::visit::{
    self,
    Visit,
};

struct Fmt {
    delim: &'static str,
}

impl Fmt {
    fn print(&mut self, args: fmt::Arguments) {
        let delim = mem::replace(&mut self.delim, "");
        print!("{}{}", delim, args);
    }
}

impl Visit for Fmt {
    fn fmt(&mut self, v: fmt::Arguments) -> Result<(), visit::Error> {
        self.print(v);
        self.delim = " ";

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), visit::Error> {
        self.print(format_args!("["));
        Ok(())
    }

    fn seq_elem(&mut self, elem: visit::Value) -> Result<(), visit::Error> {
        elem.visit(self)?;
        self.delim = ", ";

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("]"));
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result<(), visit::Error> {
        self.print(format_args!("{{"));
        Ok(())
    }

    fn map_key(&mut self, key: visit::Value) -> Result<(), visit::Error> {
        key.visit(self)?;
        self.delim = ": ";

        Ok(())
    }

    fn map_value(&mut self, value: visit::Value) -> Result<(), visit::Error> {
        value.visit(self)?;
        self.delim = ", ";

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("}}"));
        Ok(())
    }
}

fn main() {
    // A map that implements `val::value::Value`
    let mut map = BTreeMap::new();

    map.insert(Id::new(1), vec!["Hello", "World"]);
    map.insert(Id::new(2), vec!["World", "Hello"]);

    val::visit(map, Fmt { delim: "" }).unwrap();
    println!();

    // A map that implements `serde::Serialize`
    let mut map = BTreeMap::new();

    map.insert(SerdeId::new(1), vec!["Hello", "World"]);
    map.insert(SerdeId::new(2), vec!["World", "Hello"]);

    // A map that implements `serde::Serialize`
    let mut map = BTreeMap::new();

    map.insert(SerdeId::new(1), vec![Complex::Struct { a: 1, b: 2 }]);

    val::visit(val_serde::to_value(map), Fmt { delim: "" }).unwrap();
    println!();
}
