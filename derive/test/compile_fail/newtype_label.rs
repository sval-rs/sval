use sval_derive::*;

#[derive(Value)]
pub struct Newtype(#[sval(label = "value")] i32);

fn main() {

}