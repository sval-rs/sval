use sval_derive::*;

#[derive(Value)]
pub struct Tuple(#[sval(label = "a")] i32, i32, i32);

fn main() {

}