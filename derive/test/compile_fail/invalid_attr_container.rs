#[macro_use]
extern crate sval_derive;

#[derive(Value)]
#[sval(not_an_attr = "true")]
pub struct Record {
    a: i32,
}

fn main() {}
