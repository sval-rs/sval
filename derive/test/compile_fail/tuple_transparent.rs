use sval_derive::*;

#[derive(Value)]
#[sval(transparent)]
pub struct Tuple(i32, i32, i32);

fn main() {

}