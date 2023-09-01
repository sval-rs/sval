#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub struct Record (
    #[sval(label = "foo")]
    i32,
    i32,
);

fn main() {}
