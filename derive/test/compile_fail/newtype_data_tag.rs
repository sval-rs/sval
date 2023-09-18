use sval_derive::*;

#[derive(Value)]
pub struct Newtype(#[sval(data_tag = "sval::tags::NUMBER")] i32);

fn main() {

}