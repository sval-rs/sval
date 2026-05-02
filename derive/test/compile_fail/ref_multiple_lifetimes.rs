use sval_derive::*;

#[derive(Value)]
#[sval(ref)]
pub struct MultipleLifetimes<'a, 'b> {
    a: &'a i32,
    b: &'b i32,
}

fn main() {}
