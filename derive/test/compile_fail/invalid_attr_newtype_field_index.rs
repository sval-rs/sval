#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub struct Record (
    #[sval(index = 1)]
    i32,
);

fn main() {}
