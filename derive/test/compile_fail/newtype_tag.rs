use sval_derive::*;

#[derive(Value)]
pub struct Newtype(#[sval(tag = "sval::tags::NUMBER")] i32);

fn main() {

}