use std::{
    mem,
    fmt,
    collections::BTreeMap,
};

use typelib::Id;
use val::visit::{self, Visit};

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
    fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", v));
        self.delim = " ";

        Ok(())
    }

    fn seq_begin(&mut self) -> Result<(), visit::Error> {
        self.print(format_args!("["));
        Ok(())
    }

    fn seq_elem(&mut self, elem: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", elem));
        self.delim = ", ";

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("]"));
        Ok(())
    }

    fn map_begin(&mut self) -> Result<(), visit::Error> {
        self.print(format_args!("{{"));
        Ok(())
    }

    fn map_key(&mut self, key: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", key));
        self.delim = ": ";

        Ok(())
    }

    fn map_value(&mut self, value: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", value));
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
    let mut map = BTreeMap::new();

    map.insert(Id::new(1), vec!["Hello", "World"]);
    map.insert(Id::new(2), vec!["World", "Hello"]);

    val::visit(map, Fmt { delim: "" }).unwrap();
    println!();
}
