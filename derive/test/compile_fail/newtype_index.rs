use sval_derive::*;

#[derive(Value)]
pub struct Newtype(#[sval(index = 1)] i32);

fn main() {

}